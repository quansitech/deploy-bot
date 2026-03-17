//! Deployment executor

use std::path::PathBuf;
use std::sync::Arc;
use tracing::{info, error};

use crate::config;
use crate::deploy::manager::{Deployment, DeploymentManager, DeploymentStatus};
use crate::git;
use crate::installer;
use crate::runner;

/// Get user display string for logs (e.g., "[www-data]" or "[default]")
fn get_user_log_prefix(run_user: Option<&str>) -> String {
    match run_user {
        Some(user) => format!("[{user}]"),
        None => String::new(),
    }
}

/// Strip ANSI escape codes from string (for clean log output)
fn strip_ansi_codes(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Check for CSI (Control Sequence Introducer)
            if chars.peek() == Some(&'[') {
                chars.next(); // consume '['
                // Skip until we find the end of the sequence (letter)
                while let Some(&ch) = chars.peek() {
                    if ch.is_ascii_alphabetic() {
                        chars.next(); // consume the final letter
                        break;
                    } else {
                        chars.next();
                    }
                }
            }
        } else {
            result.push(c);
        }
    }
    result
}

/// Execute a deployment task
pub async fn execute_deployment(
    deployment: Deployment,
    workspace_dir: &str,
    docker_compose_path: Option<&str>,
    docker_compose_command: Option<config::DockerComposeCommand>,
    deployment_manager: Arc<DeploymentManager>,
) {
    let deployment_id = deployment.id.clone();
    let project_name = deployment.project_name.clone();

    info!("Starting deployment for project: {}", project_name);
    info!("docker_compose_path: {:?}, docker_service: {:?}", docker_compose_path, deployment.project.docker_service);
    deployment_manager.add_log(&deployment_id, "info", &format!("Starting deployment for project: {project_name}"));
    deployment_manager.add_log(&deployment_id, "info", &format!("docker_compose_path: {:?}, docker_service: {:?}", docker_compose_path, deployment.project.docker_service));

    // Update status to Running
    deployment_manager.update_status(&deployment_id, DeploymentStatus::Running);
    deployment_manager.add_log(&deployment_id, "info", "Status changed to: running");

    // Get project config
    let project = &deployment.project;
    let project_dir = PathBuf::from(workspace_dir).join(&project_name);

    // Step 1: Pull repository
    let user_prefix = get_user_log_prefix(project.run_user.as_deref());
    deployment_manager.add_log(&deployment_id, "info", &format!("{user_prefix}Pulling repository..."));
    match git::pull_repo(
        project.repo_url.clone(),
        project_dir.clone(),
        project.branch.clone(),
        None, // SSH key - TODO: add to config
        project.run_user.as_deref(),
    ).await {
        Ok(_) => {
            info!("Git pull successful for {}", project_name);
            deployment_manager.add_log(&deployment_id, "info", "Git pull successful");
        }
        Err(e) => {
            error!("Git pull failed for {}: {}", project_name, e);
            deployment_manager.add_log(&deployment_id, "error", &format!("Git pull failed: {e}"));
            deployment_manager.update_status(&deployment_id, DeploymentStatus::Failed);
            deployment_manager.add_log(&deployment_id, "info", "Deployment failed");
            return;
        }
    }

    // Step 2: Install dependencies
    let user_prefix = get_user_log_prefix(project.run_user.as_deref());
    deployment_manager.add_log(&deployment_id, "info", &format!("{user_prefix}Installing dependencies..."));
    info!("Installing dependencies: install_command={:?}, docker_service={:?}, working_dir={:?}, run_user={:?}",
        project.install_command, project.docker_service, project.working_dir, project.run_user);

    // Create log callback for real-time log streaming
    let deployment_manager_clone = deployment_manager.clone();
    let deployment_id_clone = deployment_id.clone();
    let log_callback: Option<installer::tasks::LogCallback> = Some(std::sync::Arc::new(move |line: String| {
        deployment_manager_clone.add_log(&deployment_id_clone, "info", &line);
    }));

    match installer::tasks::install_dependencies(
        &project_dir,
        &project.project_type,
        project.install_command.as_deref(),
        &project.env,
        docker_compose_path,
        docker_compose_command,
        project.docker_service.as_deref(),
        project.working_dir.as_deref(),
        project.run_user.as_deref(),
        log_callback,
    ).await {
        Ok(output) => {
            info!("Dependencies installed for {}", project_name);
            // Add command output to deployment logs (strip ANSI codes)
            if !output.is_empty() {
                let clean_output = strip_ansi_codes(&output);
                for line in clean_output.lines().take(20) {
                    deployment_manager.add_log(&deployment_id, "info", line);
                }
                if clean_output.lines().count() > 20 {
                    deployment_manager.add_log(&deployment_id, "info", &format!("... and {} more lines", clean_output.lines().count() - 20));
                }
            }
            deployment_manager.add_log(&deployment_id, "info", &format!("{user_prefix}Dependencies installed successfully"));
        }
        Err(e) => {
            error!("Dependency installation failed for {}: {}", project_name, e);
            deployment_manager.add_log(&deployment_id, "error", &format!("Dependency installation failed: {e}"));
            deployment_manager.update_status(&deployment_id, DeploymentStatus::Failed);
            deployment_manager.add_log(&deployment_id, "info", "Deployment failed");
            return;
        }
    }

    // Step 3: Build project (use custom command if provided, otherwise use default for project type)
    deployment_manager.add_log(&deployment_id, "info", &format!("{user_prefix}Running build step"));
    match runner::task::run_build(
        &project_dir,
        &project.project_type,
        project.build_command.as_deref(),
        &project.env,
        docker_compose_path,
        docker_compose_command,
        project.docker_service.as_deref(),
        project.working_dir.as_deref(),
        project.run_user.as_deref(),
    ).await {
        Ok(output) => {
            info!("Build step completed for {}", project_name);
            // Add command output to deployment logs (strip ANSI codes)
            if !output.is_empty() {
                let clean_output = strip_ansi_codes(&output);
                for line in clean_output.lines().take(20) {
                    deployment_manager.add_log(&deployment_id, "info", line);
                }
                if clean_output.lines().count() > 20 {
                    deployment_manager.add_log(&deployment_id, "info", &format!("... and {} more lines", clean_output.lines().count() - 20));
                }
            }
            deployment_manager.add_log(&deployment_id, "info", &format!("{user_prefix}Build step completed"));
        }
        Err(e) => {
            error!("Build step failed for {}: {}", project_name, e);
            deployment_manager.add_log(&deployment_id, "error", &format!("Build step failed: {e}"));
            deployment_manager.update_status(&deployment_id, DeploymentStatus::Failed);
            deployment_manager.add_log(&deployment_id, "info", "Deployment failed");
            return;
        }
    }

    // Step 4: Run extra_command if configured
    if let Some(ref extra_cmd) = project.extra_command {
        deployment_manager.add_log(&deployment_id, "info", &format!("{user_prefix}Running extra command: {extra_cmd}"));
        match runner::task::run_command(
            &project_dir,
            extra_cmd,
            &project.env,
            docker_compose_path,
            docker_compose_command,
            project.docker_service.as_deref(),
            project.working_dir.as_deref(),
            project.run_user.as_deref(),
        ).await {
            Ok(output) => {
                info!("Extra command successful for {}", project_name);
                if !output.is_empty() {
                    let clean_output = strip_ansi_codes(&output);
                    for line in clean_output.lines().take(20) {
                        deployment_manager.add_log(&deployment_id, "info", line);
                    }
                    if clean_output.lines().count() > 20 {
                        deployment_manager.add_log(&deployment_id, "info", &format!("... and {} more lines", clean_output.lines().count() - 20));
                    }
                }
                deployment_manager.add_log(&deployment_id, "info", &format!("{user_prefix}Extra command completed successfully"));
            }
            Err(e) => {
                error!("Extra command failed for {}: {}", project_name, e);
                deployment_manager.add_log(&deployment_id, "error", &format!("Extra command failed: {e}"));
                deployment_manager.update_status(&deployment_id, DeploymentStatus::Failed);
                deployment_manager.add_log(&deployment_id, "info", "Deployment failed");
                return;
            }
        }
    }

    // All steps completed successfully
    deployment_manager.update_status(&deployment_id, DeploymentStatus::Success);
    deployment_manager.add_log(&deployment_id, "info", "Deployment completed successfully");
    info!("Deployment completed successfully for {}", project_name);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ProjectType;
    use crate::deploy::manager::DeploymentStatus;
    use crate::project_config::ProjectConfig;
    use std::collections::HashMap;
    use std::process::Command;
    use tempfile::TempDir;

    /// Create a working directory with a git repo already cloned
    fn create_workspace_with_git_repo(workspace_dir: &std::path::Path, _repo_url: &str) -> std::io::Result<()> {
        std::fs::create_dir_all(workspace_dir)?;

        // Clone from local file:// URL (faster, no network needed)
        // First create a bare repo to clone from
        let temp_bare = std::env::temp_dir().join(format!("test-bare-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&temp_bare)?;
        Command::new("git")
            .args(["init", "--bare"])
            .current_dir(&temp_bare)
            .output()?;

        // Clone from the bare repo
        let output = Command::new("git")
            .args(["clone", &temp_bare.to_string_lossy(), workspace_dir.to_str().unwrap()])
            .output();

        // Clean up temp bare repo
        let _ = std::fs::remove_dir_all(&temp_bare);

        if output.is_err() {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "git clone failed"));
        }

        // Configure git user for commits
        let _ = Command::new("git")
            .args(["config", "user.email", "test@test.com"])
            .current_dir(workspace_dir)
            .output();
        let _ = Command::new("git")
            .args(["config", "user.name", "Test"])
            .current_dir(workspace_dir)
            .output();

        // Create initial commit
        std::fs::write(workspace_dir.join("README.md"), "# Test Project")?;
        let _ = Command::new("git")
            .args(["add", "."])
            .current_dir(workspace_dir)
            .output();
        let _ = Command::new("git")
            .args(["commit", "-m", "Initial commit"])
            .current_dir(workspace_dir)
            .output();

        Ok(())
    }

    #[test]
    fn test_execute_deployment_signature_has_docker_compose_path() {
        // Verify execute_deployment function signature includes docker_compose_path parameter
        // This test ensures the function can be called with docker_compose_path argument
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let manager = DeploymentManager::new(&db_path, temp_dir.path().to_string_lossy().to_string()).unwrap();

        let project_name = "test-project";
        let workspace_dir = temp_dir.path().join("workspace").join(project_name);

        // Create local git repo to avoid network calls
        create_workspace_with_git_repo(&workspace_dir, "https://github.com/test/repo.git")
            .expect("Failed to create test git repo");

        let deployment = Deployment {
            id: "test-id".to_string(),
            project_name: project_name.to_string(),
            project: ProjectConfig {
                repo_url: "https://github.com/test/repo.git".to_string(),
                branch: "main".to_string(),
                project_type: ProjectType::Nodejs,
                docker_service: Some("node".to_string()),
                working_dir: None,
                install_command: None,
                build_command: None,
                extra_command: None,
                run_user: None,
                env: HashMap::new(),
            },
            status: DeploymentStatus::Pending,
            created_at: chrono::Utc::now(),
            started_at: None,
            finished_at: None,
        };

        let workspace_base = temp_dir.path().join("workspace").to_string_lossy().to_string();

        // Create a test runtime to call the async function
        let rt = tokio::runtime::Runtime::new().unwrap();

        // This test verifies the function signature accepts docker_compose_path
        // The actual execution uses local git repo
        rt.block_on(async {
            let _ = execute_deployment(
                deployment,
                &workspace_base,
                Some("./docker-compose.yaml"),  // docker_compose_path = Some
                Some(config::DockerComposeCommand::DockerCompose),  // docker_compose_command
                std::sync::Arc::new(manager),
            ).await;
        });
    }

    #[test]
    fn test_execute_deployment_with_docker_compose_path_none() {
        // Test that execute_deployment accepts docker_compose_path = None
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let manager = DeploymentManager::new(&db_path, temp_dir.path().to_string_lossy().to_string()).unwrap();

        let project_name = "test-project-2";
        let workspace_dir = temp_dir.path().join("workspace").join(project_name);

        // Create local git repo to avoid network calls
        create_workspace_with_git_repo(&workspace_dir, "https://github.com/test/repo.git")
            .expect("Failed to create test git repo");

        let deployment = Deployment {
            id: "test-id-2".to_string(),
            project_name: project_name.to_string(),
            project: ProjectConfig {
                repo_url: "https://github.com/test/repo.git".to_string(),
                branch: "main".to_string(),
                project_type: ProjectType::Nodejs,
                docker_service: None,
                working_dir: None,
                install_command: None,
                build_command: None,
                extra_command: None,
                run_user: None,
                env: HashMap::new(),
            },
            status: DeploymentStatus::Pending,
            created_at: chrono::Utc::now(),
            started_at: None,
            finished_at: None,
        };

        let workspace_base = temp_dir.path().join("workspace").to_string_lossy().to_string();

        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            // Call with docker_compose_path = None
            let _ = execute_deployment(
                deployment,
                &workspace_base,
                None,  // docker_compose_path = None
                None,  // docker_compose_command = None
                std::sync::Arc::new(manager),
            ).await;
        });
    }
}

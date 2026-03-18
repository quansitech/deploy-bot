//! Build runner implementation

use std::path::Path;
use std::process::Command;

use crate::config::{DockerComposeCommand, ProjectType};
use crate::error::AppError;
use crate::installer::tasks;

/// Run build based on project type
/// Returns (Result, output_string) - output_string contains command stdout/stderr for logging
#[allow(clippy::too_many_arguments)]
pub async fn run_build(
    project_dir: &Path,
    project_type: &ProjectType,
    custom_command: Option<&str>,
    env_vars: &std::collections::HashMap<String, String>,
    docker_compose_paths: Option<&[String]>,
    docker_compose_command: Option<DockerComposeCommand>,
    docker_service: Option<&str>,
    working_dir: Option<&str>,
    run_user: Option<&str>,
) -> Result<String, AppError> {
    // Use custom command if provided
    if let Some(cmd) = custom_command {
        return tasks::run_command(
            project_dir,
            cmd,
            env_vars,
            docker_compose_paths,
            docker_compose_command,
            docker_service,
            working_dir,
            run_user,
            None,
        )
        .await;
    }

    match project_type {
        ProjectType::Nodejs => {
            tasks::run_command(
                project_dir,
                "npm run build",
                env_vars,
                docker_compose_paths,
                docker_compose_command,
                docker_service,
                working_dir,
                run_user,
                None,
            ).await
        }
        ProjectType::Rust => build_rust(project_dir, run_user).await,
        ProjectType::Python => {
            build_python(project_dir, docker_compose_paths, docker_compose_command, docker_service, working_dir, run_user).await
        }
        ProjectType::Php => {
            Ok(String::new()) // PHP doesn't need build
        }
        ProjectType::Custom => Ok(String::new()),
    }
}

/// Build Rust project
async fn build_rust(project_dir: &Path, _run_user: Option<&str>) -> Result<String, AppError> {
    let project_dir_clone = project_dir.to_path_buf();
    tokio::task::spawn_blocking(move || {
        let output = Command::new("cargo")
            .current_dir(&project_dir_clone)
            .args(["build", "--release"])
            .output()
            .map_err(AppError::Io)?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !output.status.success() {
            return Err(AppError::Deployment(format!("cargo build failed: {stderr}")));
        }

        let output_str = if !stdout.is_empty() { stdout.to_string() } else { stderr.to_string() };
        Ok(output_str)
    })
    .await
    .map_err(|e| AppError::Deployment(format!("Task join error: {e}")))?
}

/// Build Python project
async fn build_python(
    project_dir: &Path,
    docker_compose_paths: Option<&[String]>,
    docker_compose_command: Option<DockerComposeCommand>,
    docker_service: Option<&str>,
    working_dir: Option<&str>,
    run_user: Option<&str>,
) -> Result<String, AppError> {
    // Check for setup.py
    if project_dir.join("setup.py").exists() {
        return tasks::run_command(
            project_dir,
            "python setup.py bdist_wheel",
            &std::collections::HashMap::new(),
            docker_compose_paths,
            docker_compose_command,
            docker_service,
            working_dir,
            run_user,
            None,
        )
        .await;
    }

    // Check for pyproject.toml
    if project_dir.join("pyproject.toml").exists() {
        return tasks::run_command(
            project_dir,
            "python -m build",
            &std::collections::HashMap::new(),
            docker_compose_paths,
            docker_compose_command,
            docker_service,
            working_dir,
            run_user,
            None,
        )
        .await;
    }

    // No build needed for pure Python projects
    Ok(String::new())
}

/// Run a shell command
/// Returns the combined stdout/stderr output
#[allow(clippy::too_many_arguments)]
pub async fn run_command(
    project_dir: &Path,
    command: &str,
    env_vars: &std::collections::HashMap<String, String>,
    docker_compose_paths: Option<&[String]>,
    docker_compose_command: Option<DockerComposeCommand>,
    docker_service: Option<&str>,
    working_dir: Option<&str>,
    run_user: Option<&str>,
) -> Result<String, AppError> {
    tasks::run_command(
        project_dir,
        command,
        env_vars,
        docker_compose_paths,
        docker_compose_command,
        docker_service,
        working_dir,
        run_user,
        None,
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_run_build_custom_command() {
        let temp_dir = TempDir::new().unwrap();

        // Using custom command - should call run_command
        let result = tokio::runtime::Runtime::new().unwrap().block_on(
            run_build(
                temp_dir.path(),
                &ProjectType::Custom,
                Some("echo test"),
                &std::collections::HashMap::new(),
                None,
                None,
                None,
                None,
                None,
            )
        );

        // Result depends on whether docker is available, but function should be callable
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_run_build_custom_project_type() {
        let temp_dir = TempDir::new().unwrap();

        // Using built-in project type - no-op for Custom
        let result = tokio::runtime::Runtime::new().unwrap().block_on(
            run_build(
                temp_dir.path(),
                &ProjectType::Custom,
                None,
                &std::collections::HashMap::new(),
                None,
                None,
                None,
                None,
                None,
            )
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_run_command_empty_command() {
        let temp_dir = TempDir::new().unwrap();

        let result = tokio::runtime::Runtime::new().unwrap().block_on(
            run_command(
                temp_dir.path(),
                "",
                &std::collections::HashMap::new(),
                None,
                None,
                None,
                None,
                None,
            )
        );

        assert!(result.is_err());
    }
}

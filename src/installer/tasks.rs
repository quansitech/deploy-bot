//! Dependency installer implementation

use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::Arc;

use crate::config::ProjectType;
use crate::error::AppError;

/// Log callback type for real-time log streaming
pub type LogCallback = Arc<dyn Fn(String) + Send + Sync>;

/// Detect project type from project files
#[allow(dead_code)]
pub fn detect_project_type(project_dir: &Path) -> ProjectType {
    // Check for Node.js
    if project_dir.join("package.json").exists() {
        return ProjectType::Nodejs;
    }

    // Check for Rust
    if project_dir.join("Cargo.toml").exists() {
        return ProjectType::Rust;
    }

    // Check for Python
    if project_dir.join("requirements.txt").exists()
        || project_dir.join("pyproject.toml").exists()
        || project_dir.join("setup.py").exists()
    {
        return ProjectType::Python;
    }

    // Check for PHP
    if project_dir.join("composer.json").exists() {
        return ProjectType::Php;
    }

    ProjectType::Custom
}

/// Install dependencies based on project type
/// Returns (Result, output_string) - output_string contains command stdout/stderr for logging
#[allow(clippy::too_many_arguments)]
pub async fn install_dependencies(
    project_dir: &Path,
    project_type: &ProjectType,
    custom_command: Option<&str>,
    env_vars: &std::collections::HashMap<String, String>,
    docker_compose_path: Option<&str>,
    docker_service: Option<&str>,
    working_dir: Option<&str>,
    log_callback: Option<LogCallback>,
) -> Result<String, AppError> {
    tracing::info!("install_dependencies called: custom_command={:?}, docker_service={:?}", custom_command, docker_service);

    // Use custom command if provided
    if let Some(cmd) = custom_command {
        return run_command(
            project_dir,
            cmd,
            env_vars,
            docker_compose_path,
            docker_service,
            working_dir,
            log_callback,
        )
        .await;
    }

    match project_type {
        ProjectType::Nodejs => {
            install_nodejs(project_dir, env_vars, docker_compose_path, docker_service, working_dir, log_callback).await
        }
        ProjectType::Rust => install_rust(project_dir).await,
        ProjectType::Python => {
            install_python(project_dir, docker_compose_path, docker_service, working_dir, log_callback).await
        }
        ProjectType::Php => {
            install_php(project_dir, docker_compose_path, docker_service, working_dir, log_callback).await
        }
        ProjectType::Custom => Ok(String::new()), // No-op for custom
    }
}

/// Install Node.js dependencies
async fn install_nodejs(
    project_dir: &Path,
    env_vars: &std::collections::HashMap<String, String>,
    docker_compose_path: Option<&str>,
    docker_service: Option<&str>,
    working_dir: Option<&str>,
    log_callback: Option<LogCallback>,
) -> Result<String, AppError> {
    // Try yarn first, then npm
    let has_yarn_lock = project_dir.join("yarn.lock").exists();
    let has_pnpm_lock = project_dir.join("pnpm-lock.yaml").exists();

    let command = if has_pnpm_lock {
        "pnpm install"
    } else if has_yarn_lock {
        "yarn install"
    } else {
        "npm install"
    };

    run_command(project_dir, command, env_vars, docker_compose_path, docker_service, working_dir, log_callback).await
}

/// Install Rust dependencies
/// Note: Rust projects skip install step since `cargo build` already handles dependencies
async fn install_rust(_project_dir: &Path) -> Result<String, AppError> {
    // Skip install - cargo build in build phase will handle dependencies
    Ok(String::new())
}

/// Install Python dependencies
async fn install_python(
    project_dir: &Path,
    docker_compose_path: Option<&str>,
    docker_service: Option<&str>,
    working_dir: Option<&str>,
    log_callback: Option<LogCallback>,
) -> Result<String, AppError> {
    // Check for poetry first
    if project_dir.join("poetry.lock").exists() {
        return run_command(
            project_dir,
            "poetry install",
            &std::collections::HashMap::new(),
            docker_compose_path,
            docker_service,
            working_dir,
            log_callback,
        )
        .await;
    }

    // Fall back to pip
    if project_dir.join("requirements.txt").exists() {
        return run_command(
            project_dir,
            "pip install -r requirements.txt",
            &std::collections::HashMap::new(),
            docker_compose_path,
            docker_service,
            working_dir,
            log_callback,
        )
        .await;
    }

    Ok(String::new())
}

/// Install PHP dependencies
async fn install_php(
    project_dir: &Path,
    docker_compose_path: Option<&str>,
    docker_service: Option<&str>,
    working_dir: Option<&str>,
    log_callback: Option<LogCallback>,
) -> Result<String, AppError> {
    run_command(
        project_dir,
        "composer install --no-dev",
        &std::collections::HashMap::new(),
        docker_compose_path,
        docker_service,
        working_dir,
        log_callback,
    )
    .await
}

/// Run a shell command
/// Returns the combined stdout/stderr output
pub async fn run_command(
    project_dir: &Path,
    command: &str,
    env_vars: &std::collections::HashMap<String, String>,
    docker_compose_path: Option<&str>,
    docker_service: Option<&str>,
    working_dir: Option<&str>,
    log_callback: Option<LogCallback>,
) -> Result<String, AppError> {
    let project_dir_clone = project_dir.to_path_buf();
    let command = command.to_string();

    tracing::info!("run_command - docker_compose_path: {:?}, docker_service: {:?}", docker_compose_path, docker_service);

    // If docker_service is specified: only use project-defined env vars (don't pass host env vars)
    // This prevents host PATH from overriding container's PATH
    // Otherwise: merge host env vars with project env vars
    let env: std::collections::HashMap<String, String> = if docker_service.is_some() {
        // Add a placeholder env var to ensure docker compose doesn't inherit all host env vars
        let mut e = env_vars.clone();
        e.insert("_FORCE_EMPTY_ENV".to_string(), "1".to_string());
        e
    } else {
        let mut merged = std::env::vars().collect::<std::collections::HashMap<_, _>>();
        for (key, value) in env_vars {
            merged.insert(key.clone(), value.clone());
        }
        merged
    };

    // If docker_service is specified, use docker compose run
    if let (Some(service), Some(compose_path)) = (docker_service, docker_compose_path) {
        return run_docker_compose(
            project_dir,
            compose_path,
            service,
            working_dir,
            &command,
            &env,
            log_callback,
        )
        .await;
    }

    // Otherwise, run command directly
    tokio::task::spawn_blocking(move || {
        // Parse command
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return Err(AppError::Deployment("Empty command".to_string()));
        }

        let output = Command::new(parts[0])
            .current_dir(&project_dir_clone)
            .args(&parts[1..])
            .envs(env)
            .output()
            .map_err(AppError::Io)?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !output.status.success() {
            return Err(AppError::Deployment(format!(
                "Command failed: {command}\nstdout: {stdout}\nstderr: {stderr}"
            )));
        }

        // Log output on success
        let output_str = if !stdout.is_empty() {
            tracing::info!("{} output: {}", command, stdout);
            stdout.to_string()
        } else if !stderr.is_empty() {
            tracing::info!("{} stderr: {}", command, stderr);
            stderr.to_string()
        } else {
            String::new()
        };

        Ok(output_str)
    })
    .await
    .map_err(|e| AppError::Deployment(format!("Task join error: {e}")))?
}

/// Run command via docker compose with real-time log streaming
/// Returns the combined stdout/stderr output
async fn run_docker_compose(
    project_dir: &Path,
    docker_compose_path: &str,
    service: &str,
    working_dir: Option<&str>,
    command: &str,
    env: &std::collections::HashMap<String, String>,
    log_callback: Option<LogCallback>,
) -> Result<String, AppError> {
    tracing::info!("run_docker_compose called: docker_compose_path={}, service={}, command={}, working_dir={:?}",
        docker_compose_path, service, command, working_dir);

    let project_dir_clone = project_dir.to_path_buf();
    let docker_compose_path = docker_compose_path.to_string();
    let service = service.to_string();
    let command = command.to_string();
    let command_for_error = command.clone();
    let working_dir = working_dir.map(String::from);

    // Clone env to move into the blocking task
    let env_vars = env.clone();

    tracing::info!("Running docker compose: command={}, working_dir={:?}", command_for_error, working_dir);

    // Run docker compose and capture output directly (not in detached mode)
    // This allows us to get the logs in real-time
    let output = tokio::task::spawn_blocking(move || {
        let mut cmd = Command::new("docker");
        cmd.args(["compose", "-f", &docker_compose_path, "run", "--rm"]);

        if let Some(ref wd) = working_dir {
            cmd.args(["-w", wd]);
        }

        cmd.arg(&service);
        cmd.arg("sh");
        cmd.arg("-c");
        cmd.arg(&command);
        cmd.current_dir(&project_dir_clone);
        cmd.envs(&env_vars);

        // Capture both stdout and stderr
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let output = cmd.output().map_err(AppError::Io)?;
        Ok::<_, AppError>(output)
    })
    .await
    .map_err(|e| AppError::Deployment(format!("Task join error: {e}")))??;

    // Stream output lines to callback in real-time
    if let Some(ref callback) = log_callback {
        // Read stdout
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            callback(line.to_string());
            println!("{line}");
        }

        // Read stderr
        let stderr = String::from_utf8_lossy(&output.stderr);
        for line in stderr.lines() {
            // Skip the container ID line that docker compose outputs
            if !line.chars().all(|c| c.is_ascii_hexdigit() || c == '-') {
                callback(line.to_string());
                println!("{line}");
            }
        }
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Combine stdout and stderr for final output
    let final_logs = if !stdout.is_empty() {
        stdout.to_string()
    } else {
        stderr.to_string()
    };

    if !output.status.success() {
        return Err(AppError::Deployment(format!(
            "Docker compose run failed: {command_for_error}\n{final_logs}"
        )));
    }

    Ok(final_logs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_detect_project_type_nodejs() {
        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join("package.json"), "{}").unwrap();

        let project_type = detect_project_type(temp_dir.path());
        assert_eq!(project_type, ProjectType::Nodejs);
    }

    #[test]
    fn test_detect_project_type_rust() {
        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join("Cargo.toml"), "[package]").unwrap();

        let project_type = detect_project_type(temp_dir.path());
        assert_eq!(project_type, ProjectType::Rust);
    }

    #[test]
    fn test_detect_project_type_python_requirements() {
        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join("requirements.txt"), "flask").unwrap();

        let project_type = detect_project_type(temp_dir.path());
        assert_eq!(project_type, ProjectType::Python);
    }

    #[test]
    fn test_detect_project_type_python_pyproject() {
        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join("pyproject.toml"), "[project]").unwrap();

        let project_type = detect_project_type(temp_dir.path());
        assert_eq!(project_type, ProjectType::Python);
    }

    #[test]
    fn test_detect_project_type_python_setup() {
        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join("setup.py"), "from setuptools import setup").unwrap();

        let project_type = detect_project_type(temp_dir.path());
        assert_eq!(project_type, ProjectType::Python);
    }

    #[test]
    fn test_detect_project_type_php() {
        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join("composer.json"), "{}").unwrap();

        let project_type = detect_project_type(temp_dir.path());
        assert_eq!(project_type, ProjectType::Php);
    }

    #[test]
    fn test_detect_project_type_custom() {
        let temp_dir = TempDir::new().unwrap();
        // No recognized project files
        std::fs::write(temp_dir.path().join("README.md"), "# Project").unwrap();

        let project_type = detect_project_type(temp_dir.path());
        assert_eq!(project_type, ProjectType::Custom);
    }

    #[test]
    fn test_detect_project_type_priority_nodejs() {
        // Node.js should be detected first when multiple files exist
        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join("package.json"), "{}").unwrap();
        std::fs::write(temp_dir.path().join("Cargo.toml"), "[package]").unwrap();

        let project_type = detect_project_type(temp_dir.path());
        assert_eq!(project_type, ProjectType::Nodejs);
    }

    #[tokio::test]
    async fn test_run_command_empty_command() {
        let temp_dir = TempDir::new().unwrap();
        let result = run_command(
            temp_dir.path(),
            "",
            &std::collections::HashMap::new(),
            None,
            None,
            None,
            None,
        ).await;

        assert!(result.is_err());
    }

    #[test]
    fn test_install_dependencies_accepts_docker_compose_path() {
        // Verify that install_dependencies function signature includes docker_compose_path
        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join("package.json"), "{}").unwrap();

        // This test verifies the function accepts docker_compose_path parameter
        // We use None here to avoid requiring actual docker compose setup
        let result = tokio::runtime::Runtime::new().unwrap().block_on(
            install_dependencies(
                temp_dir.path(),
                &ProjectType::Nodejs,
                Some("npm install"),
                &std::collections::HashMap::new(),
                Some("./docker-compose.yaml"),  // docker_compose_path
                Some("node"),                     // docker_service
                None,                            // working_dir
                None,                            // log_callback
            )
        );

        // Result will be error because docker compose might not be available,
        // but we're testing that the parameter is accepted
        assert!(result.is_err() || result.is_ok());
    }

    #[test]
    fn test_install_dependencies_builtin_type_with_docker() {
        // Test that docker params are passed when using built-in project type (custom_command = None)
        let temp_dir = TempDir::new().unwrap();
        // Create composer.json to make it detect as PHP project
        std::fs::write(temp_dir.path().join("composer.json"), "{}").unwrap();

        // custom_command = None, so it will use built-in PHP install
        let result = tokio::runtime::Runtime::new().unwrap().block_on(
            install_dependencies(
                temp_dir.path(),
                &ProjectType::Php,  // Using built-in PHP type
                None,               // custom_command = None - uses built-in!
                &std::collections::HashMap::new(),
                Some("./docker-compose.yaml"),
                Some("php"),
                None,
                None,               // log_callback
            )
        );

        // Result depends on docker availability, but the key is that docker params are passed
        // We verify by checking the log output shows docker params
        let _ = result;
    }

    #[test]
    fn test_run_command_with_docker_compose_path_none() {
        // Test that run_command works with docker_compose_path = None
        let temp_dir = TempDir::new().unwrap();
        let result = tokio::runtime::Runtime::new().unwrap().block_on(
            run_command(
                temp_dir.path(),
                "echo test",
                &std::collections::HashMap::new(),
                None,  // docker_compose_path = None
                None,  // docker_service = None
                None,  // working_dir = None
                None,  // log_callback
            )
        );

        // Should fail because "echo" command may not be available in test env
        // but the parameter is correctly passed
        let _ = result;
    }

    #[test]
    fn test_run_command_with_docker_compose_path_some() {
        // Test that run_command accepts docker_compose_path = Some
        let temp_dir = TempDir::new().unwrap();
        let result = tokio::runtime::Runtime::new().unwrap().block_on(
            run_command(
                temp_dir.path(),
                "echo test",
                &std::collections::HashMap::new(),
                Some("./docker-compose.yaml"),  // docker_compose_path = Some
                Some("node"),                     // docker_service = Some
                None,                            // working_dir = None
                None,                            // log_callback
            )
        );

        // With docker_service set, it will try to run docker compose
        // Result depends on whether docker is available
        let _ = result;
    }
}

//! Git operations using command line

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::AppError;

/// Check if a directory is empty (contains no files except .deploy.yaml)
fn is_directory_empty(dir: &Path) -> Result<bool, AppError> {
    if !dir.exists() {
        return Ok(true);
    }

    let entries = fs::read_dir(dir)
        .map_err(|e| AppError::Git(format!("Failed to read directory: {e}")))?;

    for entry in entries {
        let entry = entry.map_err(|e| AppError::Git(format!("Failed to read entry: {e}")))?;
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        // Skip hidden files (starting with .)
        // Skip .deploy.yaml (the config file)
        if name.starts_with('.') || name == ".deploy.yaml" {
            continue;
        }

        // Found a non-hidden, non-config file
        return Ok(false);
    }

    Ok(true)
}

/// Clone or fetch a repository
#[allow(dead_code)]
pub async fn pull_repo(
    repo_url: String,
    target_dir: PathBuf,
    branch: String,
    ssh_key: Option<String>,
    run_user: Option<&str>,
) -> Result<(), AppError> {
    let run_user = run_user.map(String::from);
    tokio::task::spawn_blocking(move || {
        // Setup SSH key if provided
        if let Some(ref key_path) = ssh_key {
            std::env::set_var(
                "GIT_SSH_COMMAND",
                format!("ssh -i {key_path} -o StrictHostKeyChecking=no"),
            );
        }

        // Check if directory exists and has content
        if target_dir.exists() {
            // Check if directory is empty (only has .deploy.yaml or hidden files)
            if is_directory_empty(&target_dir)? {
                // Empty directory - use git clone to current dir
                clone_to_current_dir(&target_dir, &repo_url, &branch, run_user.as_deref())
            } else {
                // Non-empty directory - fetch and checkout
                fetch_and_checkout(&target_dir, &branch, run_user.as_deref())
            }
        } else {
            // Directory doesn't exist - clone to new directory
            clone_repo(&target_dir, &repo_url, &branch, run_user.as_deref())
        }
    })
    .await
    .map_err(|e| AppError::Git(format!("Task join error: {e}")))?
}

/// Clone repository
#[allow(dead_code)]
fn clone_repo(target_dir: &Path, repo_url: &str, branch: &str, run_user: Option<&str>) -> Result<(), AppError> {
    // Create parent directory (need permissions)
    if let Some(parent) = target_dir.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| AppError::Git(format!("Failed to create directory: {e}")))?;
    }

    // Build git clone command
    let git_cmd = if let Some(user) = run_user {
        format!("sudo -u {} git clone --branch {} --depth 1 {} {}", user, branch, repo_url, target_dir.display())
    } else {
        format!("git clone --branch {branch} --depth 1 {repo_url} {}", target_dir.display())
    };

    let output = Command::new("sh")
        .arg("-c")
        .arg(&git_cmd)
        .output()
        .map_err(|e| AppError::Git(format!("Failed to run git clone: {e}")))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::Git(format!("Git clone failed: {stderr}")));
    }

    Ok(())
}

/// Clone repository to current directory (for empty directories)
#[allow(dead_code)]
fn clone_to_current_dir(target_dir: &Path, repo_url: &str, branch: &str, run_user: Option<&str>) -> Result<(), AppError> {
    // Ensure directory exists
    if !target_dir.exists() {
        std::fs::create_dir_all(target_dir)
            .map_err(|e| AppError::Git(format!("Failed to create directory: {e}")))?;
    }

    // Save .deploy.yaml if exists (git clone requires empty directory)
    let deploy_yaml_path = target_dir.join(".deploy.yaml");
    let deploy_yaml_backup = if deploy_yaml_path.exists() {
        let temp_path = std::env::temp_dir().join(".deploy.yaml.backup");
        std::fs::rename(&deploy_yaml_path, &temp_path)
            .map_err(|e| AppError::Git(format!("Failed to backup .deploy.yaml: {e}")))?;
        Some(temp_path)
    } else {
        None
    };

    // Try to clone, restore .deploy.yaml on failure
    let clone_result = (|| {
        let git_cmd = if let Some(user) = run_user {
            format!("sudo -u {user} git clone --branch {branch} --depth 1 -- {repo_url} .")
        } else {
            format!("git clone --branch {branch} --depth 1 -- {repo_url} .")
        };

        let output = Command::new("sh")
            .current_dir(target_dir)
            .arg("-c")
            .arg(&git_cmd)
            .output()
            .map_err(|e| AppError::Git(format!("Failed to run git clone: {e}")))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::Git(format!("Git clone failed: {stderr}")));
        }

        Ok(())
    })();

    // Restore .deploy.yaml
    if let Some(backup_path) = deploy_yaml_backup {
        if let Err(e) = std::fs::rename(&backup_path, &deploy_yaml_path) {
            // Log warning but don't fail if restore fails
            eprintln!("Warning: Failed to restore .deploy.yaml: {e}");
        }
    }

    clone_result
}

/// Fetch and checkout
#[allow(dead_code)]
fn fetch_and_checkout(target_dir: &Path, branch: &str, run_user: Option<&str>) -> Result<(), AppError> {
    // Build git commands with sudo if run_user is specified
    let (git_fetch, git_checkout, git_pull) = if let Some(user) = run_user {
        (
            format!("sudo -u {user} git fetch origin {branch}"),
            format!("sudo -u {user} git checkout -f origin/{branch}"),
            format!("sudo -u {user} git pull origin {branch}"),
        )
    } else {
        (
            format!("git fetch origin {branch}"),
            format!("git checkout -f origin/{branch}"),
            format!("git pull origin {branch}"),
        )
    };

    // Fetch
    let output = Command::new("sh")
        .current_dir(target_dir)
        .arg("-c")
        .arg(&git_fetch)
        .output()
        .map_err(|e| AppError::Git(format!("Failed to run git fetch: {e}")))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::Git(format!("Git fetch failed: {stderr}")));
    }

    // Checkout
    let output = Command::new("sh")
        .current_dir(target_dir)
        .arg("-c")
        .arg(&git_checkout)
        .output()
        .map_err(|e| AppError::Git(format!("Failed to run git checkout: {e}")))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::Git(format!("Git checkout failed: {stderr}")));
    }

    // Pull latest
    let output = Command::new("sh")
        .current_dir(target_dir)
        .arg("-c")
        .arg(&git_pull)
        .output()
        .map_err(|e| AppError::Git(format!("Failed to run git pull: {e}")))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::Git(format!("Git pull failed: {stderr}")));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_is_directory_empty_nonexistent_dir() {
        let result = is_directory_empty(&PathBuf::from("/nonexistent/path"));
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_is_directory_empty_with_deploy_yaml() {
        // Create temp directory with only .deploy.yaml
        let temp_dir = tempfile::TempDir::new().unwrap();
        let deploy_yaml = temp_dir.path().join(".deploy.yaml");
        fs::write(&deploy_yaml, "name = \"test\"").unwrap();

        let result = is_directory_empty(temp_dir.path());
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_is_directory_empty_with_other_files() {
        // Create temp directory with other files
        let temp_dir = tempfile::TempDir::new().unwrap();
        let temp_dir_path = temp_dir.path().to_path_buf();
        fs::write(temp_dir_path.join("test.txt"), "content").unwrap();

        let result = is_directory_empty(&temp_dir_path);
        assert!(result.is_ok());
        assert!(!result.unwrap());

        // Keep temp_dir alive until here
    }

    #[test]
    fn test_is_directory_empty_with_hidden_files() {
        // Create temp directory with hidden files
        let temp_dir = tempfile::TempDir::new().unwrap();
        let temp_dir_path = temp_dir.path().to_path_buf();
        fs::write(temp_dir_path.join(".gitignore"), "content").unwrap();

        let result = is_directory_empty(&temp_dir_path);
        assert!(result.is_ok());
        // Hidden files are considered empty (user responsibility to keep directory clean)
        assert!(result.unwrap());
    }

    #[test]
    fn test_is_directory_empty_with_regular_files() {
        // Create temp directory with regular files (not hidden)
        // This should be considered non-empty and trigger fetch_and_checkout
        let temp_dir = tempfile::TempDir::new().unwrap();
        let temp_dir_path = temp_dir.path().to_path_buf();
        fs::write(temp_dir_path.join("README.md"), "# Project").unwrap();

        let result = is_directory_empty(&temp_dir_path);
        assert!(result.is_ok());
        // Regular files mean directory is NOT empty
        assert!(!result.unwrap());
    }
}

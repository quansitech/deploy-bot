//! Self-update module

use axum::{
    extract::State,
    http::HeaderMap,
    Json,
};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;
use tokio::io::AsyncWriteExt;
use tracing::{error, info, warn};

use crate::error::{AppError, AppResult};
use crate::webhook::handler::WebhookAppState;

/// Payload from GitHub release webhook
#[derive(Debug, Deserialize, Serialize)]
pub struct ReleasePayload {
    pub tag_name: String,
    #[serde(rename = "browser_download_url")]
    pub download_url: String,
}

/// Response for self-update endpoint
#[derive(Debug, Serialize)]
pub struct UpdateResponse {
    pub message: String,
    pub updated: bool,
    pub version: Option<String>,
}

/// Current version info
#[derive(Debug, PartialEq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    /// Parse version from string like "v0.2.0"
    pub fn parse(version_str: &str) -> Option<Self> {
        // Remove leading 'v' if present
        let version_str = version_str.trim_start_matches('v');

        let parts: Vec<&str> = version_str.split('.').collect();
        if parts.len() != 3 {
            return None;
        }

        let major = parts[0].parse().ok()?;
        let minor = parts[1].parse().ok()?;
        let patch = parts[2].parse().ok()?;

        Some(Version { major, minor, patch })
    }

    /// Compare two versions: returns true if self > other
    pub fn gt(&self, other: &Version) -> bool {
        if self.major != other.major {
            return self.major > other.major;
        }
        if self.minor != other.minor {
            return self.minor > other.minor;
        }
        self.patch > other.patch
    }
}

/// Get current version from Cargo.toml
pub fn get_current_version() -> AppResult<Version> {
    // Read from Cargo.toml in the current executable's directory
    let exe_path = std::env::current_exe()
        .map_err(|e| AppError::Config(format!("Failed to get executable path: {e}")))?;

    let cargo_toml_path = exe_path
        .parent()
        .ok_or_else(|| AppError::Config("Failed to get parent directory".to_string()))?
        .join("Cargo.toml");

    // Also try the source directory as fallback
    let source_cargo_toml = PathBuf::from("Cargo.toml");

    let content = if cargo_toml_path.exists() {
        std::fs::read_to_string(&cargo_toml_path)
            .map_err(|e| AppError::Config(format!("Failed to read Cargo.toml: {e}")))?
    } else if source_cargo_toml.exists() {
        std::fs::read_to_string(&source_cargo_toml)
            .map_err(|e| AppError::Config(format!("Failed to read Cargo.toml: {e}")))?
    } else {
        return Err(AppError::Config("Cannot find Cargo.toml".to_string()));
    };

    // Parse version from Cargo.toml
    let version_str = content
        .lines()
        .find(|line| line.starts_with("version = "))
        .ok_or_else(|| AppError::Config("Version not found in Cargo.toml".to_string()))?
        .split('"')
        .nth(1)
        .ok_or_else(|| AppError::Config("Invalid version format".to_string()))?;

    Version::parse(version_str)
        .ok_or_else(|| AppError::Config(format!("Failed to parse version: {version_str}")))
}

/// Check if new version is greater than current version
pub fn is_newer_version(new_version: &str) -> AppResult<bool> {
    let current = get_current_version()?;
    let new = Version::parse(new_version)
        .ok_or_else(|| AppError::Config(format!("Invalid new version format: {new_version}")))?;

    Ok(new.gt(&current))
}

/// Download binary from URL
pub async fn download_binary(url: &str, version: &str) -> AppResult<PathBuf> {
    let temp_dir = std::env::temp_dir();
    let binary_path = temp_dir.join(format!("deploy-bot-{}", version.replace('v', "")));

    info!("Downloading new binary from {url} to {}", binary_path.display());

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| AppError::Config(format!("Failed to download binary: {e}")))?;

    if !response.status().is_success() {
        return Err(AppError::Config(format!(
            "Download failed with status: {}",
            response.status()
        )));
    }

    // Stream download to file
    let mut file = tokio::fs::File::create(&binary_path)
        .await
        .map_err(|e| AppError::Config(format!("Failed to create temp file: {e}")))?;

    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk
            .map_err(|e| AppError::Config(format!("Download error: {e}")))?;
        file.write_all(&chunk)
            .await
            .map_err(|e| AppError::Config(format!("Write error: {e}")))?;
    }

    file.flush()
        .await
        .map_err(|e| AppError::Config(format!("Flush error: {e}")))?;

    // Make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&binary_path)
            .map_err(|e| AppError::Config(format!("Failed to get permissions: {e}")))?
            .permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&binary_path, perms)
            .map_err(|e| AppError::Config(format!("Failed to set permissions: {e}")))?;
    }

    info!("Binary downloaded successfully to {}", binary_path.display());
    Ok(binary_path)
}

/// Execute update script
pub fn execute_update_script(script_path: &str, new_binary_path: &Path) -> AppResult<()> {
    info!(
        "Executing update script: {script_path} with binary {}",
        new_binary_path.display()
    );

    // Fork and execute the update script
    // The script will handle stopping the old process, replacing the binary, and starting the new one
    let output = Command::new(script_path)
        .arg(new_binary_path.to_str().unwrap_or(""))
        .output()
        .map_err(|e| AppError::Config(format!("Failed to execute update script: {e}")))?;

    if output.status.success() {
        info!("Update script executed successfully");
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("Update script failed: {stderr}");
        Err(AppError::Config(format!(
            "Update script failed: {stderr}"
        )))
    }
}

/// Handle self-update webhook
pub async fn handle_self_update(
    State(state): State<WebhookAppState>,
    headers: HeaderMap,
    Json(payload): Json<ReleasePayload>,
) -> AppResult<Json<UpdateResponse>> {
    info!(
        "Received self-update request for version {}",
        payload.tag_name
    );

    // Verify webhook secret if configured
    if state.config.server.is_update_webhook_secret_configured() {
        let secret = state
            .config
            .server
            .update_webhook_secret
            .as_ref()
            .unwrap();

        // Get secret from header
        let header_secret = headers
            .get("X-Update-Secret")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        match header_secret {
            Some(ref s) if s == secret => {
                info!("Webhook secret verified successfully");
            }
            Some(_) => {
                warn!("Invalid webhook secret provided");
                return Err(AppError::WebhookValidation(
                    "Invalid webhook secret".to_string(),
                ));
            }
            None => {
                warn!("No webhook secret provided");
                return Err(AppError::WebhookValidation(
                    "Webhook secret required but not provided".to_string(),
                ));
            }
        }
    } else {
        warn!("No webhook secret configured, skipping verification");
    }

    // Check if update_script is configured
    if !state.config.server.is_update_script_configured() {
        return Err(AppError::Config(
            "Update script not configured".to_string(),
        ));
    }

    // Check version
    let new_version_str = &payload.tag_name;
    let is_newer = is_newer_version(new_version_str)?;

    if !is_newer {
        let current_version = get_current_version()?;
        let msg = format!(
            "Current version ({}.{}.{}) is already up to date or newer than {new_version_str}",
            current_version.major, current_version.minor, current_version.patch
        );
        info!("{msg}");
        return Ok(Json(UpdateResponse {
            message: msg,
            updated: false,
            version: Some(new_version_str.to_string()),
        }));
    }

    info!("New version {new_version_str} is available, downloading...");

    // Download new binary
    let binary_path = download_binary(&payload.download_url, new_version_str).await?;

    // Execute update script
    let script_path = state
        .config
        .server
        .update_script
        .as_ref()
        .ok_or_else(|| AppError::Config("Update script not configured".to_string()))?;

    // Execute in a spawned thread to allow the current process to be stopped
    let script_path_owned = script_path.to_string();
    let binary_path_owned = binary_path.clone();

    // Spawn a thread to run the update (so we can return a response before the process is killed)
    std::thread::spawn(move || {
        if let Err(e) = execute_update_script(&script_path_owned, &binary_path_owned) {
            error!("Update failed: {e}");
        }
    });

    // Give a brief moment for the thread to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    Ok(Json(UpdateResponse {
        message: format!("Update to {new_version_str} initiated"),
        updated: true,
        version: Some(new_version_str.to_string()),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_parse() {
        assert_eq!(
            Version::parse("v0.2.0"),
            Some(Version {
                major: 0,
                minor: 2,
                patch: 0
            })
        );
        assert_eq!(
            Version::parse("1.2.3"),
            Some(Version {
                major: 1,
                minor: 2,
                patch: 3
            })
        );
        assert_eq!(Version::parse("invalid"), None);
        assert_eq!(Version::parse("1.2"), None);
    }

    #[test]
    fn test_version_compare() {
        let v1 = Version {
            major: 0,
            minor: 2,
            patch: 0,
        };
        let v2 = Version {
            major: 0,
            minor: 2,
            patch: 0,
        };
        let v3 = Version {
            major: 0,
            minor: 3,
            patch: 0,
        };
        let v4 = Version {
            major: 1,
            minor: 0,
            patch: 0,
        };

        assert!(!v1.gt(&v2)); // equal
        assert!(v3.gt(&v1)); // 0.3.0 > 0.2.0
        assert!(v4.gt(&v1)); // 1.0.0 > 0.2.0
        assert!(!v1.gt(&v3)); // 0.2.0 < 0.3.0
        assert!(!v1.gt(&v4)); // 0.2.0 < 1.0.0
    }
}

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
use tracing::{info, warn};

use crate::error::{AppError, AppResult};
use crate::webhook::handler::WebhookAppState;

/// Current version of the application (hardcoded for runtime access)
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

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

/// Get current version from hardcoded constant
pub fn get_current_version() -> AppResult<Version> {
    Version::parse(VERSION)
        .ok_or_else(|| AppError::Config(format!("Failed to parse version: {VERSION}")))
}

/// Check if new version is greater than current version
pub fn is_newer_version(new_version: &str) -> AppResult<bool> {
    let current = get_current_version()?;
    let new = Version::parse(new_version)
        .ok_or_else(|| AppError::Config(format!("Invalid new version format: {new_version}")))?;

    Ok(new.gt(&current))
}

/// Download binary from URL (supports .tar.gz and raw binary)
pub async fn download_binary(url: &str, version: &str, github_mirror: Option<&str>) -> AppResult<PathBuf> {
    let temp_dir = std::env::temp_dir();
    let is_tarball = url.ends_with(".tar.gz");

    // Apply GitHub mirror if configured and URL contains github.com
    let download_url = if let Some(mirror) = github_mirror {
        if url.contains("github.com") {
            let mirrored_url = format!("{mirror}{url}");
            info!("Applying GitHub mirror: {} -> {}", url, mirrored_url);
            mirrored_url
        } else {
            url.to_string()
        }
    } else {
        url.to_string()
    };

    let download_path = if is_tarball {
        temp_dir.join(format!("deploy-bot-{}.tar.gz", version.replace('v', "")))
    } else {
        temp_dir.join(format!("deploy-bot-{}", version.replace('v', "")))
    };

    info!("Downloading new binary from {download_url} to {}", download_path.display());

    let client = reqwest::Client::new();
    let response = client
        .get(&download_url)
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
    let mut file = tokio::fs::File::create(&download_path)
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

    // Extract tarball if needed
    let binary_path = if is_tarball {
        info!("Extracting tarball...");
        let output = std::process::Command::new("tar")
            .args(["-xzf", &download_path.to_string_lossy(), "-C", &temp_dir.to_string_lossy()])
            .output()
            .map_err(|e| AppError::Config(format!("Failed to extract tarball: {e}")))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::Config(format!("Failed to extract tarball: {stderr}")));
        }

        // Find the extracted binary (assuming it's named "deploy-bot")
        let extracted_binary = temp_dir.join("deploy-bot");
        if !extracted_binary.exists() {
            return Err(AppError::Config("Extracted binary not found".to_string()));
        }

        // Remove the tarball
        tokio::fs::remove_file(&download_path).await.ok();

        extracted_binary
    } else {
        download_path
    };

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

/// Get the payload file path for replay updates
fn get_payload_path() -> AppResult<PathBuf> {
    let exe_dir = std::env::current_exe()
        .map_err(|e| AppError::Config(format!("Failed to get executable path: {e}")))?
        .parent()
        .ok_or_else(|| AppError::Config("Failed to get executable directory".to_string()))?
        .to_path_buf();

    Ok(exe_dir.join(".deploy-last-payload").join("deploy-bot-last-update.json"))
}

/// Save the update payload to a file for replay
pub fn save_update_payload(payload: &ReleasePayload) -> AppResult<()> {
    let payload_path = get_payload_path()?;

    // Create directory if it doesn't exist
    if let Some(parent) = payload_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| AppError::Config(format!("Failed to create payload directory: {e}")))?;
    }

    // Serialize and save payload
    let json = serde_json::to_string_pretty(payload)
        .map_err(|e| AppError::Config(format!("Failed to serialize payload: {e}")))?;

    std::fs::write(&payload_path, json)
        .map_err(|e| AppError::Config(format!("Failed to write payload file: {e}")))?;

    info!("Update payload saved to {}", payload_path.display());
    Ok(())
}

/// Load the update payload from file for replay
pub fn load_update_payload() -> AppResult<ReleasePayload> {
    let payload_path = get_payload_path()?;

    if !payload_path.exists() {
        return Err(AppError::Config(
            "No update payload found. Please trigger an update webhook first.".to_string(),
        ));
    }

    let json = std::fs::read_to_string(&payload_path)
        .map_err(|e| AppError::Config(format!("Failed to read payload file: {e}")))?;

    serde_json::from_str(&json)
        .map_err(|e| AppError::Config(format!("Failed to parse payload file: {e}")))
}

/// Execute update script
pub fn execute_update_script(script_path: &str, new_binary_path: &Path) -> AppResult<()> {
    info!(
        "Executing update script: {script_path} with binary {}",
        new_binary_path.display()
    );

    // Check if systemd-run is available (for systemd-based systems)
    // and use it to escape the cgroup. Otherwise fall back to setsid.
    let use_systemd_run = Command::new("systemd-run")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    let _child = if use_systemd_run {
        // Use systemd-run to start the update script in a separate systemd service.
        // This is critical because:
        // 1. It escapes deploy-bot.service's cgroup, avoiding KillMode=control-group cleanup
        // 2. The script runs as a fully independent temporary service
        info!("Using systemd-run to start update script");
        Command::new("systemd-run")
            .args([
                "--unit=deploy-bot-updater",
                "--",
                "/bin/bash",
                script_path,
                new_binary_path.to_str().unwrap_or(""),
            ])
            .spawn()
    } else {
        // Fall back to setsid for non-systemd systems
        info!("systemd-run not available, using setsid");
        Command::new("setsid")
            .args([script_path, new_binary_path.to_str().unwrap_or("")])
            .spawn()
    }
    .map_err(|e| AppError::Config(format!("Failed to spawn update script: {e}")))?;

    info!("Update script started");
    Ok(())
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
        // Still save the payload even if version is not newer
        if let Err(e) = save_update_payload(&payload) {
            warn!("Failed to save update payload: {e}");
        }
        return Ok(Json(UpdateResponse {
            message: msg,
            updated: false,
            version: Some(new_version_str.to_string()),
        }));
    }

    // Save payload for replay before downloading
    if let Err(e) = save_update_payload(&payload) {
        warn!("Failed to save update payload: {e}");
    }

    info!("New version {new_version_str} is available, downloading...");

    // Get GitHub mirror configuration
    let github_mirror = state.config.server.github_mirror.as_deref();

    // Download new binary
    let binary_path = download_binary(&payload.download_url, new_version_str, github_mirror).await?;

    // Execute update script
    let script_path = state
        .config
        .server
        .update_script
        .as_ref()
        .ok_or_else(|| AppError::Config("Update script not configured".to_string()))?;

    // Log before spawning thread (logs will be lost after process is stopped)
    info!(
        "About to execute update script: {} with binary: {}",
        script_path,
        binary_path.display()
    );

    // Execute in a spawned thread to allow the current process to be stopped
    let script_path_owned = script_path.to_string();
    let binary_path_owned = binary_path.clone();

    // Spawn a thread to run the update (so we can return a response before the process is killed)
    // The script runs in a new session via setsid, so it continues even after parent exits
    std::thread::spawn(move || {
        if let Err(e) = execute_update_script(&script_path_owned, &binary_path_owned) {
            eprintln!("Update failed: {e}"); // Print to stderr as fallback
        }
    });

    // Return immediately without waiting - the spawned thread handles the update
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

    /// Helper function to test mirror URL transformation
    #[allow(dead_code)]
    fn apply_github_mirror(url: &str, github_mirror: Option<&str>) -> String {
        if let Some(mirror) = github_mirror {
            if url.contains("github.com") {
                return format!("{mirror}{url}");
            }
        }
        url.to_string()
    }

    #[test]
    fn test_github_mirror_applied_to_github_url() {
        let url = "https://github.com/owner/repo/releases/download/v1.0.0/deploy-bot";
        let mirror = "https://ghproxy.com/";
        let result = apply_github_mirror(url, Some(mirror));
        assert_eq!(
            result,
            "https://ghproxy.com/https://github.com/owner/repo/releases/download/v1.0.0/deploy-bot"
        );
    }

    #[test]
    fn test_github_mirror_not_applied_to_non_github_url() {
        let url = "https://other-cdn.com/file.tar.gz";
        let mirror = "https://ghproxy.com/";
        let result = apply_github_mirror(url, Some(mirror));
        assert_eq!(result, "https://other-cdn.com/file.tar.gz");
    }

    #[test]
    fn test_no_mirror_returns_original_url() {
        let url = "https://github.com/owner/repo/releases/download/v1.0.0/deploy-bot";
        let result = apply_github_mirror(url, None);
        assert_eq!(result, "https://github.com/owner/repo/releases/download/v1.0.0/deploy-bot");
    }

    #[test]
    fn test_github_mirror_none_value() {
        let url = "https://github.com/owner/repo/releases/download/v1.0.0/deploy-bot";
        let mirror: Option<&str> = None;
        let result = apply_github_mirror(url, mirror);
        assert_eq!(result, "https://github.com/owner/repo/releases/download/v1.0.0/deploy-bot");
    }
}

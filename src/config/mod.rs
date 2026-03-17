//! Configuration module

use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub github_secret: Option<String>,
    pub gitlab_token: Option<String>,
    pub codeup_token: Option<String>,
    pub workspace_dir: String,
    pub docker_compose_path: Option<String>,
    /// Detected docker compose command (None if docker_compose_path is not set)
    pub docker_compose_command: Option<DockerComposeCommand>,
    /// Path to the update script for self-update functionality
    pub update_script: Option<String>,
    /// Secret for self-update webhook verification
    pub update_webhook_secret: Option<String>,
    /// Comma-separated webhook URLs to notify on self-update (can be multiple)
    pub update_webhook_urls: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    Nodejs,
    Rust,
    Python,
    Php,
    Custom,
}

/// Docker Compose command type
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub enum DockerComposeCommand {
    /// docker compose (new version, Docker 19.03+)
    DockerCompose,
    /// docker-compose (legacy standalone command)
    DockerComposeLegacy,
}

impl DockerComposeCommand {
    /// Detect available docker compose command
    /// Returns None if docker_compose_path is not set
    pub fn detect(docker_compose_path: &Option<String>) -> Option<Self> {
        if docker_compose_path.is_none() {
            return None;
        }

        // Try docker compose first (new version)
        // Check output contains version number (not help text)
        let output = std::process::Command::new("docker")
            .args(["compose", "version"])
            .output();

        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            let combined = format!("{stdout}{stderr}");

            // Check if output contains version info (e.g., "v2.x.x" or "Docker Compose version v2")
            // NOT the help text which says "Usage: docker [OPTIONS] COMMAND"
            if !combined.contains("Usage:") && !combined.contains("docker --help")
                && (combined.contains("Compose") || combined.contains("v2") || combined.contains("v1"))
            {
                tracing::info!("Detected docker compose command: docker compose (output: {})", stdout.trim());
                return Some(DockerComposeCommand::DockerCompose);
            }
        }

        // Fallback to docker-compose (legacy version)
        let output = std::process::Command::new("docker-compose")
            .arg("--version")
            .output();

        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Check for version pattern like v1.x.x or v2.x.x
            let version_regex = regex::Regex::new(r"v\d+\.\d+\.\d+").ok();
            let has_version = version_regex.map(|r| r.is_match(&stdout)).unwrap_or(false);
            if output.status.success() && (stdout.contains("Compose") || has_version) {
                tracing::info!("Detected docker compose command: docker-compose (legacy)");
                return Some(DockerComposeCommand::DockerComposeLegacy);
            }
        }

        // Neither is available - log warning
        tracing::warn!("No docker compose command available (tried 'docker compose' and 'docker-compose')");
        None
    }
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectType::Nodejs => write!(f, "nodejs"),
            ProjectType::Rust => write!(f, "rust"),
            ProjectType::Python => write!(f, "python"),
            ProjectType::Php => write!(f, "php"),
            ProjectType::Custom => write!(f, "custom"),
        }
    }
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let mut config: Config = toml::from_str(&content)?;

        // Detect docker compose command if docker_compose_path is set
        config.server.docker_compose_command =
            DockerComposeCommand::detect(&config.server.docker_compose_path);

        Ok(config)
    }
}

impl ServerConfig {
    /// Check if self-update is configured
    pub fn is_update_script_configured(&self) -> bool {
        self.update_script.is_some()
    }

    /// Check if webhook secret is configured for self-update
    pub fn is_update_webhook_secret_configured(&self) -> bool {
        self.update_webhook_secret.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_config_load_success() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
[server]
host = "0.0.0.0"
port = 8080
workspace_dir = "./workspace"
docker_compose_path = "./docker-compose.yaml"
"#
        )
        .unwrap();
        file.flush().unwrap();

        let config = Config::load(file.path().to_str().unwrap()).unwrap();
        assert_eq!(config.server.host, "0.0.0.0");
        assert_eq!(config.server.port, 8080);
        assert_eq!(
            config.server.docker_compose_path,
            Some("./docker-compose.yaml".to_string())
        );
    }

    #[test]
    fn test_config_load_with_optional_fields() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
[server]
host = "127.0.0.1"
port = 9000
github_secret = "secret"
gitlab_token = "gitlab-token"
codeup_token = "codeup-token"
workspace_dir = "/var/workspace"
"#
        )
        .unwrap();
        file.flush().unwrap();

        let config = Config::load(file.path().to_str().unwrap()).unwrap();
        assert_eq!(config.server.host, "127.0.0.1");
        assert_eq!(config.server.port, 9000);
        assert_eq!(
            config.server.github_secret,
            Some("secret".to_string())
        );
        assert_eq!(
            config.server.gitlab_token,
            Some("gitlab-token".to_string())
        );
        assert_eq!(
            config.server.codeup_token,
            Some("codeup-token".to_string())
        );
        assert_eq!(config.server.docker_compose_path, None);
    }

    #[test]
    fn test_config_load_file_not_found() {
        let result = Config::load("/nonexistent/path/config.toml");
        assert!(result.is_err());
    }

    #[test]
    fn test_config_load_invalid_toml() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "invalid toml content = =").unwrap();
        file.flush().unwrap();

        let result = Config::load(file.path().to_str().unwrap());
        assert!(result.is_err());
    }
}

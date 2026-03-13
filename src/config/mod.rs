//! Configuration module

use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub webhook_token: String,
    pub github_secret: Option<String>,
    pub gitlab_token: Option<String>,
    pub codeup_token: Option<String>,
    pub log_dir: String,
    pub workspace_dir: String,
    pub docker_compose_path: Option<String>,
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
        let config: Config = toml::from_str(&content)?;
        Ok(config)
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
webhook_token = "test-token"
log_dir = "./logs"
workspace_dir = "./workspace"
docker_compose_path = "./docker-compose.yaml"
"#
        )
        .unwrap();
        file.flush().unwrap();

        let config = Config::load(file.path().to_str().unwrap()).unwrap();
        assert_eq!(config.server.host, "0.0.0.0");
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.server.webhook_token, "test-token");
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
webhook_token = "token"
github_secret = "secret"
gitlab_token = "gitlab-token"
codeup_token = "codeup-token"
log_dir = "/var/log"
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

//! Project configuration module
//! 配置下沉到各项目目录的 .deploy.yaml

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::config::{ProjectType, DockerComposePaths};

/// Docker 服务重启配置，支持字符串和数组两种格式
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(untagged)]
pub enum RestartService {
    #[default]
    None,
    Single(String),
    Multiple(Vec<String>),
}

impl RestartService {
    /// 获取需要重启的服务列表
    pub fn to_services(&self) -> Vec<String> {
        match self {
            RestartService::None => vec![],
            RestartService::Single(s) => vec![s.clone()],
            RestartService::Multiple(v) => v.clone(),
        }
    }
}

/// Deserialize an Option<String> treating empty strings as None
fn deserialize_empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.filter(|s| !s.trim().is_empty()))
}

/// Project-level configuration loaded from .deploy.yaml
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectConfig {
    #[serde(default)]
    pub repo_url: Option<String>,
    #[serde(default)]
    pub branch: Option<String>,
    pub project_type: ProjectType,
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub docker_service: Option<String>,
    pub working_dir: Option<String>,
    pub install_command: Option<String>,
    pub build_command: Option<String>,
    pub extra_command: Option<String>,
    /// Run commands as this user (e.g., "www-data", "nginx")
    #[serde(default)]
    pub run_user: Option<String>,
    #[serde(default)]
    pub env: HashMap<String, String>,
    /// Docker 服务重启配置，部署完成后串行重启指定的服务
    #[serde(default)]
    pub restart_service: RestartService,
    /// Docker compose 配置文件路径，支持字符串或数组格式
    /// .deploy.yaml 中的此配置会覆盖 config.yaml 的配置
    #[serde(default)]
    pub docker_compose_path: DockerComposePaths,
}

impl ProjectConfig {
    /// Load project config from .deploy.yaml file
    pub fn load_from_file(path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: ProjectConfig = toml::from_str(&content)?;
        Ok(config)
    }

    /// Validate project configuration
    /// For non-Custom types, repo_url and branch must be configured
    pub fn validate(&self) -> Result<(), String> {
        // Custom type doesn't require git configuration
        if self.project_type == ProjectType::Custom {
            return Ok(());
        }

        // All other types require repo_url and branch
        if self.repo_url.is_none() || self.repo_url.as_ref().map(|s| s.trim().is_empty()).unwrap_or(true) {
            return Err(format!(
                "Project type '{}' requires repo_url to be configured",
                self.project_type
            ));
        }

        if self.branch.is_none() || self.branch.as_ref().map(|s| s.trim().is_empty()).unwrap_or(true) {
            return Err(format!(
                "Project type '{}' requires branch to be configured",
                self.project_type
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_project_config_load_full() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
repo_url = "https://github.com/example/test.git"
branch = "main"
project_type = "php"
docker_service = "php"
working_dir = "/app"
install_command = "composer install"
build_command = "php artisan migrate"
run_user = "www-data"
env = {{ APP_ENV = "production", DB_HOST = "localhost" }}
"#
        )
        .unwrap();
        file.flush().unwrap();

        let config = ProjectConfig::load_from_file(file.path()).unwrap();
        assert_eq!(config.repo_url, Some("https://github.com/example/test.git".to_string()));
        assert_eq!(config.branch, Some("main".to_string()));
        assert_eq!(config.project_type, ProjectType::Php);
        assert_eq!(config.docker_service, Some("php".to_string()));
        assert_eq!(config.working_dir, Some("/app".to_string()));
        assert_eq!(config.install_command, Some("composer install".to_string()));
        assert_eq!(config.build_command, Some("php artisan migrate".to_string()));
        assert_eq!(config.extra_command, None);
        assert_eq!(config.run_user, Some("www-data".to_string()));
        assert_eq!(config.env.get("APP_ENV"), Some(&"production".to_string()));
        assert_eq!(config.env.get("DB_HOST"), Some(&"localhost".to_string()));
    }

    #[test]
    fn test_project_config_load_minimal() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
repo_url = "https://github.com/example/test.git"
branch = "main"
project_type = "nodejs"
"#
        )
        .unwrap();
        file.flush().unwrap();

        let config = ProjectConfig::load_from_file(file.path()).unwrap();
        assert_eq!(config.repo_url, Some("https://github.com/example/test.git".to_string()));
        assert_eq!(config.branch, Some("main".to_string()));
        assert_eq!(config.project_type, ProjectType::Nodejs);
        assert_eq!(config.docker_service, None);
        assert_eq!(config.working_dir, None);
        assert_eq!(config.install_command, None);
        assert_eq!(config.build_command, None);
        assert_eq!(config.extra_command, None);
        assert_eq!(config.run_user, None);
        assert!(config.env.is_empty());
    }

    #[test]
    fn test_project_config_load_all_project_types() {
        let types = vec![
            ("nodejs", ProjectType::Nodejs),
            ("rust", ProjectType::Rust),
            ("python", ProjectType::Python),
            ("php", ProjectType::Php),
            ("custom", ProjectType::Custom),
        ];

        for (type_str, expected) in types {
            let mut file = NamedTempFile::new().unwrap();
            writeln!(
                file,
                r#"
repo_url = "https://github.com/test/test.git"
branch = "main"
project_type = "{}"
"#,
                type_str
            )
            .unwrap();
            file.flush().unwrap();

            let config = ProjectConfig::load_from_file(file.path()).unwrap();
            assert_eq!(config.project_type, expected, "Failed for type: {}", type_str);
        }
    }

    #[test]
    fn test_project_config_docker_service_empty_string_is_none() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
repo_url = "https://github.com/example/test.git"
branch = "main"
project_type = "php"
docker_service = ""
"#
        )
        .unwrap();
        file.flush().unwrap();

        let config = ProjectConfig::load_from_file(file.path()).unwrap();
        assert_eq!(config.docker_service, None);
    }

    #[test]
    fn test_project_config_docker_service_whitespace_is_none() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
repo_url = "https://github.com/example/test.git"
branch = "main"
project_type = "php"
docker_service = "  "
"#
        )
        .unwrap();
        file.flush().unwrap();

        let config = ProjectConfig::load_from_file(file.path()).unwrap();
        assert_eq!(config.docker_service, None);
    }

    #[test]
    fn test_project_config_load_file_not_found() {
        let result = ProjectConfig::load_from_file(std::path::Path::new("/nonexistent/.deploy.yaml"));
        assert!(result.is_err());
    }

    #[test]
    fn test_project_config_load_no_spaces() {
        // Test TOML parsing without spaces around =
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
repo_url = "https://github.com/example/test.git"
branch = "main"
project_type = "php"
docker_service="workspace7.4"
working_dir="/var/www/zgq"
"#
        )
        .unwrap();
        file.flush().unwrap();

        let config = ProjectConfig::load_from_file(file.path()).unwrap();
        assert_eq!(config.docker_service, Some("workspace7.4".to_string()));
        assert_eq!(config.working_dir, Some("/var/www/zgq".to_string()));
    }

    #[test]
    fn test_project_config_load_invalid_toml() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "invalid toml = =").unwrap();
        file.flush().unwrap();

        let result = ProjectConfig::load_from_file(file.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_project_config_run_user_parsing() {
        // Test with run_user specified
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
repo_url = "https://github.com/test/test.git"
branch = "main"
project_type = "nodejs"
run_user = "nginx"
"#
        )
        .unwrap();
        file.flush().unwrap();

        let config = ProjectConfig::load_from_file(file.path()).unwrap();
        assert_eq!(config.run_user, Some("nginx".to_string()));
    }

    #[test]
    fn test_restart_service_single_string() {
        // Test with single string restart_service
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
repo_url = "https://github.com/test/test.git"
branch = "main"
project_type = "python"
restart_service = "web"
"#
        )
        .unwrap();
        file.flush().unwrap();

        let config = ProjectConfig::load_from_file(file.path()).unwrap();
        assert_eq!(config.restart_service.to_services(), vec!["web"]);
    }

    #[test]
    fn test_restart_service_multiple() {
        // Test with multiple services restart_service
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
repo_url = "https://github.com/test/test.git"
branch = "main"
project_type = "python"
restart_service = ["web", "worker"]
"#
        )
        .unwrap();
        file.flush().unwrap();

        let config = ProjectConfig::load_from_file(file.path()).unwrap();
        assert_eq!(config.restart_service.to_services(), vec!["web", "worker"]);
    }

    #[test]
    fn test_restart_service_not_configured() {
        // Test when restart_service is not configured
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
repo_url = "https://github.com/test/test.git"
branch = "main"
project_type = "python"
"#
        )
        .unwrap();
        file.flush().unwrap();

        let config = ProjectConfig::load_from_file(file.path()).unwrap();
        assert!(config.restart_service.to_services().is_empty());
    }

    #[test]
    fn test_project_config_git_type_parsing() {
        // Test parsing git project type
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
repo_url = "https://github.com/test/test.git"
branch = "main"
project_type = "git"
"#
        )
        .unwrap();
        file.flush().unwrap();

        let config = ProjectConfig::load_from_file(file.path()).unwrap();
        assert_eq!(config.project_type, ProjectType::Git);
    }

    #[test]
    fn test_validate_git_type_requires_repo_url() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
branch = "main"
project_type = "git"
"#
        )
        .unwrap();
        file.flush().unwrap();

        let config = ProjectConfig::load_from_file(file.path()).unwrap();
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("repo_url"));
    }

    #[test]
    fn test_validate_git_type_requires_branch() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
repo_url = "https://github.com/test/test.git"
project_type = "git"
"#
        )
        .unwrap();
        file.flush().unwrap();

        let config = ProjectConfig::load_from_file(file.path()).unwrap();
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("branch"));
    }

    #[test]
    fn test_validate_custom_type_no_repo_required() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
project_type = "custom"
"#
        )
        .unwrap();
        file.flush().unwrap();

        let config = ProjectConfig::load_from_file(file.path()).unwrap();
        let result = config.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_nodejs_type_requires_repo() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
project_type = "nodejs"
"#
        )
        .unwrap();
        file.flush().unwrap();

        let config = ProjectConfig::load_from_file(file.path()).unwrap();
        let result = config.validate();
        assert!(result.is_err());
    }
}

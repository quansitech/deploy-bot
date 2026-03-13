//! Webhook handler

use axum::{
    body::to_bytes,
    extract::{Path, State},
    http::HeaderMap,
    Json,
};
use serde_json::Value;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::info;

use crate::config::Config;
use crate::deploy::DeploymentManager;
use crate::error::{AppError, AppResult};
use crate::project_config::ProjectConfig;
use crate::webhook::middleware as webhook_middleware;

/// App state for webhook handler
#[derive(Clone)]
pub struct WebhookAppState {
    pub config: Arc<Config>,
    pub deployment_manager: Arc<DeploymentManager>,
}

const MAX_BODY_SIZE: usize = 10 * 1024 * 1024; // 10MB

pub async fn handle_webhook(
    Path(project_name): Path<String>,
    State(state): State<WebhookAppState>,
    request: axum::http::Request<axum::body::Body>,
) -> AppResult<Json<Value>> {
    info!("Received webhook for project: {}", project_name);

    // Step 0: 验证 webhook 请求
    let server_config = &state.config.server;

    // 提取 headers 先用于验证
    let headers = request.headers().clone();

    // 获取请求 body 用于验证和解析
    let body_bytes = to_bytes(request.into_body(), MAX_BODY_SIZE).await
        .map_err(|e| AppError::Config(format!("Failed to read request body: {e}")))?;
    let body = &body_bytes[..];

    // 验证 webhook 请求
    validate_webhook_request(&headers, body, server_config)?;

    let workspace_dir = &server_config.workspace_dir;

    // Step 1: 查找项目目录
    let project_dir = PathBuf::from(workspace_dir).join(&project_name);
    if !project_dir.exists() {
        return Err(AppError::ProjectNotFound(format!(
            "Project directory not found: {}",
            project_dir.display()
        )));
    }

    // Step 2: 查找 .deploy.yaml 配置文件
    let config_file = project_dir.join(".deploy.yaml");
    if !config_file.exists() {
        return Err(AppError::ProjectNotFound(format!(
            "Project not configured: {} not found",
            config_file.display()
        )));
    }

    // Step 3: 读取项目配置
    let project = ProjectConfig::load_from_file(&config_file)
        .map_err(|e| AppError::Config(format!("Failed to load project config: {e}")))?;

    info!("Project config loaded: repo_url={}, branch={}, project_type={}, docker_service={:?}, working_dir={:?}",
        project.repo_url, project.branch, project.project_type, project.docker_service, project.working_dir);

    // Step 4: 将部署任务加入队列
    let result = state.deployment_manager.queue_deployment(project_name, project);

    match result {
        Some(deployment_id) => Ok(Json(serde_json::json!({
            "message": "Deployment queued",
            "deployment_id": deployment_id,
        }))),
        None => Ok(Json(serde_json::json!({
            "message": "Deployment skipped - duplicate task exists",
        }))),
    }
}

/// Validate webhook request based on platform-specific headers
fn validate_webhook_request(
    headers: &HeaderMap,
    body: &[u8],
    server_config: &crate::config::ServerConfig,
) -> AppResult<()> {
    // 检查各个平台的 header

    // GitHub: X-Hub-Signature-256
    if let Some(signature) = headers.get("X-Hub-Signature-256") {
        if let Some(secret) = &server_config.github_secret {
            let signature_str = signature.to_str()
                .map_err(|_| AppError::WebhookValidation("Invalid signature header".to_string()))?;
            webhook_middleware::validate_github_signature(body, signature_str, secret)?;
            info!("GitHub webhook signature validated");
            return Ok(());
        }
    }

    // GitLab: X-Gitlab-Token
    if let Some(token) = headers.get("X-Gitlab-Token") {
        if let Some(secret) = &server_config.gitlab_token {
            let token_str = token.to_str()
                .map_err(|_| AppError::WebhookValidation("Invalid token header".to_string()))?;
            webhook_middleware::validate_gitlab_token(token_str, secret)?;
            info!("GitLab webhook token validated");
            return Ok(());
        }
    }

    // Codeup: X-Codeup-Token
    if let Some(token) = headers.get("X-Codeup-Token") {
        if let Some(secret) = &server_config.codeup_token {
            let token_str = token.to_str()
                .map_err(|_| AppError::WebhookValidation("Invalid token header".to_string()))?;
            webhook_middleware::validate_codeup_token(token_str, secret)?;
            info!("Codeup webhook token validated");
            return Ok(());
        }
    }

    // 如果没有配置任何平台的验证，或者请求不包含任何验证 header，则跳过验证
    // (向后兼容)
    info!("No webhook validation applied - no valid token/secret found");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_function_exists() {
        let _ = handle_webhook as fn(_, _, _) -> _;
    }
}

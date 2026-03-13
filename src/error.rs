//! Error handling module

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// Application error types
#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Git error: {0}")]
    Git(String),

    #[error("Project not found: {0}")]
    ProjectNotFound(String),

    #[error("Webhook validation failed: {0}")]
    WebhookValidation(String),

    #[error("Deployment error: {0}")]
    Deployment(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Config(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::Git(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::ProjectNotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::WebhookValidation(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Deployment(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::Io(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::Serialization(err) => (StatusCode::BAD_REQUEST, err.to_string()),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    #[test]
    fn test_app_error_config_status() {
        let error = AppError::Config("test error".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_app_error_git_status() {
        let error = AppError::Git("git error".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_app_error_project_not_found_status() {
        let error = AppError::ProjectNotFound("project not found".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_app_error_webhook_validation_status() {
        let error = AppError::WebhookValidation("invalid signature".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_app_error_deployment_status() {
        let error = AppError::Deployment("deployment failed".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_app_error_io_status() {
        let error = AppError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"));
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_app_error_serialization_status() {
        let error = AppError::Serialization(serde_json::from_str::<serde_json::Value>("invalid").unwrap_err());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_app_error_response_body() {
        // Test that error message is correctly formatted in the error display
        let error = AppError::Config("configuration error message".to_string());
        assert_eq!(error.to_string(), "Configuration error: configuration error message");

        let error2 = AppError::ProjectNotFound("myproject".to_string());
        assert_eq!(error2.to_string(), "Project not found: myproject");
    }
}

//! Web UI module for deployment management

use askama::Template;
use axum::{
    extract::{Path, State},
    response::{Html, Redirect},
};

use crate::deploy::manager::Deployment;
use crate::webhook::handler::WebhookAppState;

/// List page template
#[derive(Template)]
#[template(path = "list.html")]
struct ListTemplate<'a> {
    deployments: &'a [Deployment],
}

/// Detail page template
#[derive(Template)]
#[template(path = "detail.html")]
struct DetailTemplate<'a> {
    deployment: &'a Deployment,
    logs: &'a [crate::database::DeploymentLog],
    status_str: &'a str,
}

/// List all deployments
pub async fn list_deployments(
    State(state): State<WebhookAppState>,
) -> Html<String> {
    let deployments = state.deployment_manager.get_all_deployments();
    let template = ListTemplate { deployments: &deployments };
    Html(template.render().unwrap())
}

/// Show deployment detail
pub async fn show_deployment(
    Path(id): Path<String>,
    State(state): State<WebhookAppState>,
) -> Html<String> {
    match state.deployment_manager.get_deployment(&id) {
        Some(deployment) => {
            let logs = state.deployment_manager.get_logs(&id);
            let status_str = deployment.status.to_string();
            let template = DetailTemplate {
                deployment: &deployment,
                logs: &logs,
                status_str: &status_str,
            };
            Html(template.render().unwrap())
        }
        None => Html("<html><body><p>Deployment not found</p><a href='/'>Back</a></body></html>".to_string()),
    }
}

/// Delete a deployment (only pending)
pub async fn delete_deployment(
    Path(id): Path<String>,
    State(state): State<WebhookAppState>,
) -> Redirect {
    if state.deployment_manager.delete_deployment(&id) {
        Redirect::to("/")
    } else {
        Redirect::to(&format!("/deploy/{id}"))
    }
}

/// Retry a failed deployment
pub async fn retry_deployment(
    Path(id): Path<String>,
    State(state): State<WebhookAppState>,
) -> Redirect {
    if state.deployment_manager.retry_deployment(&id) {
        Redirect::to(&format!("/deploy/{id}"))
    } else {
        Redirect::to("/")
    }
}

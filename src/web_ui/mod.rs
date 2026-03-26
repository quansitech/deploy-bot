//! Web UI module for deployment management

use askama::Template;
use axum::{
    extract::{Path, Query, State},
    response::{Html, Redirect},
};

use crate::deploy::manager::Deployment;
use crate::webhook::handler::WebhookAppState;

const PAGE_SIZE: u32 = 20;

/// Query parameters for list page
#[derive(serde::Deserialize)]
pub struct ListQuery {
    pub page: Option<u32>,
}

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

/// List deployments with pagination
pub async fn list_deployments(
    State(state): State<WebhookAppState>,
    Query(query): Query<ListQuery>,
) -> Html<String> {
    let page = query.page.unwrap_or(1).max(1);
    let deployments = state.deployment_manager.get_deployments_paginated(page, PAGE_SIZE);
    let template = ListTemplate { deployments: &deployments };
    Html(template.render().unwrap())
}

/// API endpoint for infinite scroll - returns HTML fragment of deployment rows
pub async fn deployments_api(
    State(state): State<WebhookAppState>,
    Query(query): Query<ListQuery>,
) -> Html<String> {
    let page = query.page.unwrap_or(1).max(1);
    let deployments = state.deployment_manager.get_deployments_paginated(page, PAGE_SIZE);

    if deployments.is_empty() {
        return Html(String::new());
    }

    let html = deployments
        .iter()
        .map(|d| {
            let actions = match d.status.to_string().as_str() {
                "Pending" => format!(
                    r#"<a href="/deploy/{}">查看</a>
                <form method="POST" action="/deploy/{}/delete" style="display:inline;">
                    <button type="submit" class="btn-delete">删除</button>
                </form>"#,
                    d.id, d.id
                ),
                "Failed" => format!(
                    r#"<a href="/deploy/{}">查看</a>
                <form method="POST" action="/deploy/{}/retry" style="display:inline;">
                    <button type="submit" class="btn-retry">重试</button>
                </form>"#,
                    d.id, d.id
                ),
                _ => format!(r#"<a href="/deploy/{}">查看</a>"#, d.id),
            };
            format!(
                r#"<tr>
            <td><a href="/deploy/{}">{}</a></td>
            <td>{}</td>
            <td>{}</td>
            <td><span class="status {}">{}</span></td>
            <td>{}</td>
            <td>{}</td>
        </tr>"#,
                d.id,
                d.id,
                d.project_name,
                d.project.branch.as_deref().unwrap_or("N/A"),
                d.status,
                d.status,
                d.created_at_local(),
                actions
            )
        })
        .collect::<Vec<_>>()
        .join("");

    Html(html)
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

//! WebSocket module for real-time log streaming

#![allow(clippy::collapsible_if)]

use crate::deploy::manager::DeploymentManager;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::Response,
};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::broadcast;
use crate::webhook::handler::WebhookAppState;

/// WebSocket handler for deployment logs
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(deployment_id): Path<String>,
    State(state): State<WebhookAppState>,
) -> Response {
    let deployment_manager = state.deployment_manager.clone();

    // Check if deployment exists
    if deployment_manager.get_deployment(&deployment_id).is_none() {
        return ws.on_upgrade(|_| async {
            // Connection closed - deployment not found
        });
    }

    ws.on_upgrade(move |socket| handle_socket(socket, deployment_id, deployment_manager))
}

async fn handle_socket(
    socket: WebSocket,
    deployment_id: String,
    deployment_manager: Arc<DeploymentManager>,
) {
    let (mut sender, mut receiver) = socket.split();

    // Send existing logs first
    let logs = deployment_manager.get_logs(&deployment_id);
    for log in logs {
        let msg = serde_json::to_string(&log).unwrap_or_default();
        if sender.send(Message::Text(msg)).await.is_err() {
            return;
        }
    }

    // Subscribe to new log broadcasts
    let mut log_receiver = deployment_manager.subscribe_logs();

    // Send current deployment status
    if let Some(deployment) = deployment_manager.get_deployment(&deployment_id) {
        let status_msg = serde_json::json!({
            "type": "status",
            "status": deployment.status.to_string(),
        });
        let _ = sender.send(Message::Text(status_msg.to_string())).await;
    }

    // Loop to receive both WebSocket messages and new logs
    loop {
        tokio::select! {
            // Handle incoming messages from client
            msg = receiver.next() => {
                match msg {
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Err(_)) => break,
                    _ => {}
                }
            }
            // Handle new log broadcasts
            log_result = log_receiver.recv() => {
                match log_result {
                    Ok(msg) => {
                        // Only send if it matches this deployment
                        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&msg);
                        if let Ok(log) = parsed {
                            let dept_id = log.get("deployment_id").and_then(|v| v.as_str());
                            if dept_id == Some(&deployment_id) {
                                if sender.send(Message::Text(msg)).await.is_err() {
                                    break;
                                }
                            }
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => {}
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
        }
    }
}

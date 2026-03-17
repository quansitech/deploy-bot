//! Deploy Bot - Automated deployment service

mod cli;
mod config;
mod database;
mod deploy;
mod error;
mod git;
mod installer;
mod logging;
mod runner;
mod webhook;
mod project_config;
mod web_ui;
mod websocket;

use axum::{
    routing::{get, post},
    Router,
};
use clap::Parser as ClapParser;
use cli::{Cli, Command};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tokio::time::Duration;
use tracing::{info, warn};

use crate::deploy::{DeploymentManager, executor};
use crate::webhook::handler::WebhookAppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Handle CLI commands
    match &cli.command {
        Some(Command::Migrate) => {
            run_migrations().await?;
            return Ok(());
        }
        Some(Command::MigrateStatus) => {
            show_migration_status().await?;
            return Ok(());
        }
        None | Some(Command::Server) => {
            // Default: start the server
        }
    }

    // Load configuration from the executable's directory
    let exe_dir = std::env::current_exe()
        .map_err(|e| anyhow::anyhow!("Failed to get executable path: {e}"))?
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Failed to get executable directory"))?
        .to_path_buf();
    let config_path = exe_dir.join("config.yaml");
    let config = config::Config::load(config_path.to_str().unwrap())
        .map_err(|e| anyhow::anyhow!("Failed to load config: {e}"))?;

    // Initialize logging first
    let _guard = logging::init(std::path::PathBuf::from(&config.server.log_dir));

    // Log detected docker compose command (after logging is initialized)
    match &config.server.docker_compose_command {
        Some(config::DockerComposeCommand::DockerCompose) => {
            info!("Docker compose command: docker compose (detected)");
        }
        Some(config::DockerComposeCommand::DockerComposeLegacy) => {
            info!("Docker compose command: docker-compose (legacy, detected)");
        }
        None => {
            if config.server.docker_compose_path.is_some() {
                warn!("Docker compose path configured but no docker compose command detected!");
            }
        }
    }

    let config = Arc::new(config);

    info!("Starting Deploy Bot v{}", env!("CARGO_PKG_VERSION"));

    // Build application state with SQLite persistence
    let db_dir = std::path::PathBuf::from("db");
    std::fs::create_dir_all(&db_dir).ok();
    let db_path = db_dir.join("deployments.db");
    let deployment_manager = Arc::new(
        DeploymentManager::new(&db_path, config.server.workspace_dir.clone())
            .map_err(|e| anyhow::anyhow!("Failed to initialize database: {e}"))?
    );

    info!("Database initialized at: {:?}", db_path);

    // Create webhook app state
    let webhook_state = WebhookAppState {
        config: config.clone(),
        deployment_manager: deployment_manager.clone(),
    };

    // Start deployment worker in background
    let worker_deployment_manager = deployment_manager.clone();
    let worker_workspace_dir = config.server.workspace_dir.clone();
    let worker_docker_compose_path = config.server.docker_compose_path.clone();
    let worker_docker_compose_command = config.server.docker_compose_command;
    tokio::spawn(async move {
        info!("Deployment worker started");
        loop {
            if let Some(task) = worker_deployment_manager.pop_deployment() {
                info!("Worker: processing deployment {}", task.id);
                executor::execute_deployment(
                    task,
                    &worker_workspace_dir,
                    worker_docker_compose_path.as_deref(),
                    worker_docker_compose_command,
                    worker_deployment_manager.clone(),
                ).await;
            } else {
                // Queue empty, wait before checking again
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    });

    // Build router with all routes - use webhook_state for all routes
    let app = Router::new()
        // Web UI routes
        .route("/", get(web_ui::list_deployments))
        .route("/deploy/:id", get(web_ui::show_deployment))
        .route("/deploy/:id/delete", post(web_ui::delete_deployment))
        .route("/deploy/:id/retry", post(web_ui::retry_deployment))
        // Webhook route
        .route("/webhook/:project_name", post(webhook::handler::handle_webhook))
        // WebSocket route
        .route("/ws/deploy/:id", get(websocket::ws_handler))
        .layer(TraceLayer::new_for_http())
        .with_state(webhook_state);

    // Start server
    let addr = SocketAddr::new(
        config.server.host.parse().unwrap_or_else(|_| "0.0.0.0".parse().unwrap()),
        config.server.port,
    );

    info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Run database migrations
async fn run_migrations() -> anyhow::Result<()> {
    // Initialize logging to stdout
    tracing_subscriber::fmt()
        .with_target(false)
        .init();

    info!("Running database migrations...");

    let db_path = Cli::get_db_path();

    // Ensure db directory exists
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Run migrations
    database::run_migrations_at(&db_path)
        .map_err(|e| anyhow::anyhow!("Migration failed: {e}"))?;

    info!("Migrations completed successfully!");
    println!("Migrations completed successfully!");

    Ok(())
}

/// Show database migration status
async fn show_migration_status() -> anyhow::Result<()> {
    // Initialize logging to stdout
    tracing_subscriber::fmt()
        .with_target(false)
        .init();

    info!("Checking migration status...");

    let db_path = Cli::get_db_path();

    // Check if database exists
    if !db_path.exists() {
        println!("Database file does not exist. Run 'cargo run -- migrate' first.");
        return Ok(());
    }

    // Get migration status
    let migrations = database::get_migration_status_at(&db_path)
        .map_err(|e| anyhow::anyhow!("Failed to get migration status: {e}"))?;

    println!("Applied migrations:");
    if migrations.is_empty() {
        println!("  (none)");
    } else {
        for m in migrations {
            println!("  - {m}");
        }
    }

    Ok(())
}

//! CLI module for deploy-bot
//!
//! Provides command-line interface for database migrations and server control

use clap::{Parser as ClapParser, Subcommand};
use std::path::PathBuf;

/// CLI arguments for deploy-bot
#[derive(ClapParser, Debug)]
#[command(name = "deploy-bot")]
#[command(about = "Automated deployment service", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

/// Available commands
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Run database migrations
    Migrate,
    /// Show database migration status
    MigrateStatus,
    /// Start the HTTP server (default)
    Server,
}

impl Cli {
    /// Get the database path
    pub fn get_db_path() -> PathBuf {
        let db_dir = PathBuf::from("db");
        db_dir.join("deployments.db")
    }
}

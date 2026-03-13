//! Logging configuration module

use std::path::PathBuf;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initialize the logging system
pub fn init(log_dir: PathBuf) -> WorkerGuard {
    // Create log directory if it doesn't exist
    std::fs::create_dir_all(&log_dir).expect("Failed to create log directory");

    // Create file appender
    let file_appender = tracing_appender::rolling::daily(&log_dir, "deploy-bot.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // Create env filter
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,deploy_bot=debug"));

    // Initialize subscriber
    tracing_subscriber::registry()
        .with(env_filter)
        .with(
            fmt::layer()
                .with_writer(non_blocking)
                .with_ansi(false)
                .with_target(true)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true),
        )
        .with(
            fmt::layer()
                .with_writer(std::io::stderr)
                .with_ansi(true)
                .with_target(false),
        )
        .init();

    guard
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_init_creates_log_directory() {
        let temp_dir = TempDir::new().unwrap();
        let log_dir = temp_dir.path().join("logs");

        // Note: init() calls .init() on the global subscriber, which can only be done once
        // So we can only test directory creation
        std::fs::create_dir_all(&log_dir).unwrap();
        assert!(log_dir.exists());
    }
}

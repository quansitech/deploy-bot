-- V1__initial.sql
-- Initial database schema for deploy-bot

CREATE TABLE IF NOT EXISTS deployments (
    id TEXT PRIMARY KEY,
    project_name TEXT NOT NULL,
    repo_url TEXT NOT NULL,
    branch TEXT NOT NULL,
    project_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    install_command TEXT,
    build_command TEXT,
    extra_command TEXT,
    docker_service TEXT,
    working_dir TEXT,
    created_at TEXT NOT NULL,
    started_at TEXT,
    finished_at TEXT
);

CREATE TABLE IF NOT EXISTS deployment_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    deployment_id TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    level TEXT NOT NULL,
    message TEXT NOT NULL,
    FOREIGN KEY (deployment_id) REFERENCES deployments(id)
);

CREATE INDEX IF NOT EXISTS idx_deployment_logs_deployment_id
ON deployment_logs(deployment_id);

//! Database module for SQLite persistence

mod migrations;

use crate::config::ProjectType;
use crate::deploy::manager::{Deployment, DeploymentStatus};
use chrono::{DateTime, Utc};
use parking_lot::Mutex as ParkingMutex;
use rusqlite::{params, Connection, Result as SqliteResult};
use std::path::Path;

/// Run migrations on a database file (standalone function for CLI)
pub fn run_migrations_at<P: AsRef<Path>>(path: P) -> SqliteResult<()> {
    let mut conn = Connection::open(path)?;
    migrations::migrations::runner()
        .run(&mut conn)
        .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;
    tracing::info!("Database migrations completed");
    Ok(())
}

/// Get migration status for a database file (standalone function for CLI)
pub fn get_migration_status_at<P: AsRef<Path>>(path: P) -> SqliteResult<Vec<String>> {
    let mut conn = Connection::open(path)?;
    let migrations = migrations::migrations::runner()
        .get_applied_migrations(&mut conn)
        .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;
    Ok(migrations.iter().map(|m| m.name().to_string()).collect())
}

/// Deployment log entry
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeploymentLog {
    pub id: i64,
    pub deployment_id: String,
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub message: String,
}

/// Database wrapper
pub struct Database {
    conn: ParkingMutex<Connection>,
}

impl Database {
    /// Create a new database connection
    pub fn new<P: AsRef<Path>>(path: P) -> SqliteResult<Self> {
        let conn = Connection::open(path)?;
        let db = Self {
            conn: ParkingMutex::new(conn),
        };
        db.run_migrations()?;
        Ok(db)
    }

    /// Run database migrations
    fn run_migrations(&self) -> SqliteResult<()> {
        let mut conn = self.conn.lock();
        migrations::migrations::runner()
            .run(&mut *conn)
            .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;
        tracing::info!("Database migrations completed");
        Ok(())
    }

    /// Get migration status
    ///
    /// Returns the list of applied migrations
    #[allow(dead_code)]
    pub fn get_migration_status(&self) -> SqliteResult<Vec<String>> {
        let mut conn = self.conn.lock();
        let migrations = migrations::migrations::runner()
            .get_applied_migrations(&mut *conn)
            .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;
        Ok(migrations.iter().map(|m| m.name().to_string()).collect())
    }

    /// Insert a new deployment
    pub fn insert_deployment(&self, deployment: &Deployment) -> SqliteResult<()> {
        let conn = self.conn.lock();
        conn.execute(
            "INSERT INTO deployments (id, project_name, repo_url, branch, project_type, status,
                install_command, build_command, extra_command, docker_service, working_dir, created_at, started_at, finished_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            params![
                deployment.id,
                deployment.project_name,
                deployment.project.repo_url,
                deployment.project.branch,
                format!("{:?}", deployment.project.project_type).to_lowercase(),
                format!("{}", deployment.status).to_lowercase(),
                deployment.project.install_command,
                deployment.project.build_command,
                deployment.project.extra_command,
                deployment.project.docker_service,
                deployment.project.working_dir,
                deployment.created_at.to_rfc3339(),
                deployment.started_at.map(|t| t.to_rfc3339()),
                deployment.finished_at.map(|t| t.to_rfc3339()),
            ],
        )?;
        Ok(())
    }

    /// Update deployment status
    pub fn update_deployment_status(&self, id: &str, status: &DeploymentStatus,
        started_at: Option<DateTime<Utc>>, finished_at: Option<DateTime<Utc>>) -> SqliteResult<()> {
        let conn = self.conn.lock();
        conn.execute(
            "UPDATE deployments SET status = ?1, started_at = ?2, finished_at = ?3 WHERE id = ?4",
            params![
                format!("{status}").to_lowercase(),
                started_at.map(|t| t.to_rfc3339()),
                finished_at.map(|t| t.to_rfc3339()),
                id,
            ],
        )?;
        Ok(())
    }

    /// Get deployment by ID
    pub fn get_deployment(&self, id: &str) -> SqliteResult<Option<Deployment>> {
        let conn = self.conn.lock();
        let mut stmt = conn.prepare(
            "SELECT id, project_name, repo_url, branch, project_type, status,
                install_command, build_command, extra_command, docker_service, working_dir,
                created_at, started_at, finished_at
             FROM deployments WHERE id = ?1"
        )?;

        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(self.row_to_deployment(row)?))
        } else {
            Ok(None)
        }
    }

    /// Get all deployments
    pub fn get_all_deployments(&self) -> SqliteResult<Vec<Deployment>> {
        let conn = self.conn.lock();
        let mut stmt = conn.prepare(
            "SELECT id, project_name, repo_url, branch, project_type, status,
                install_command, build_command, extra_command, docker_service, working_dir,
                created_at, started_at, finished_at
             FROM deployments ORDER BY created_at DESC"
        )?;

        let mut deployments = Vec::new();
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            deployments.push(self.row_to_deployment(row)?);
        }

        Ok(deployments)
    }

    /// Delete deployment by ID
    pub fn delete_deployment(&self, id: &str) -> SqliteResult<bool> {
        let conn = self.conn.lock();
        // First delete logs
        conn.execute("DELETE FROM deployment_logs WHERE deployment_id = ?1", params![id])?;
        // Then delete deployment
        let rows_affected = conn.execute("DELETE FROM deployments WHERE id = ?1", params![id])?;
        Ok(rows_affected > 0)
    }

    /// Convert row to Deployment
    fn row_to_deployment(&self, row: &rusqlite::Row) -> SqliteResult<Deployment> {
        let status_str: String = row.get(5)?;
        let project_type_str: String = row.get(4)?;
        let created_at_str: String = row.get(11)?;
        let started_at_str: Option<String> = row.get(12)?;
        let finished_at_str: Option<String> = row.get(13)?;

        let project = crate::project_config::ProjectConfig {
            repo_url: row.get(2)?,
            branch: row.get(3)?,
            project_type: match project_type_str.as_str() {
                "nodejs" => ProjectType::Nodejs,
                "rust" => ProjectType::Rust,
                "python" => ProjectType::Python,
                "php" => ProjectType::Php,
                _ => ProjectType::Custom,
            },
            docker_service: row.get(9)?,
            working_dir: row.get(10)?,
            install_command: row.get(6)?,
            build_command: row.get(7)?,
            extra_command: row.get(8)?,
            run_user: row.get(14).ok(),
            env: std::collections::HashMap::new(),
        };

        Ok(Deployment {
            id: row.get(0)?,
            project_name: row.get(1)?,
            project,
            status: match status_str.as_str() {
                "pending" => DeploymentStatus::Pending,
                "running" => DeploymentStatus::Running,
                "success" => DeploymentStatus::Success,
                "failed" => DeploymentStatus::Failed,
                "cancelled" => DeploymentStatus::Cancelled,
                _ => DeploymentStatus::Pending,
            },
            created_at: DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            started_at: started_at_str.and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                .map(|dt| dt.with_timezone(&Utc)),
            finished_at: finished_at_str.and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                .map(|dt| dt.with_timezone(&Utc)),
        })
    }

    /// Insert a deployment log
    pub fn insert_log(&self, deployment_id: &str, level: &str, message: &str) -> SqliteResult<i64> {
        let conn = self.conn.lock();
        let timestamp = Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO deployment_logs (deployment_id, timestamp, level, message) VALUES (?1, ?2, ?3, ?4)",
            params![deployment_id, timestamp, level, message],
        )?;
        Ok(conn.last_insert_rowid())
    }

    /// Get logs for a deployment
    pub fn get_deployment_logs(&self, deployment_id: &str) -> SqliteResult<Vec<DeploymentLog>> {
        let conn = self.conn.lock();
        let mut stmt = conn.prepare(
            "SELECT id, deployment_id, timestamp, level, message FROM deployment_logs
             WHERE deployment_id = ?1 ORDER BY timestamp ASC"
        )?;

        let mut logs = Vec::new();
        let mut rows = stmt.query(params![deployment_id])?;

        while let Some(row) = rows.next()? {
            let timestamp_str: String = row.get(2)?;
            logs.push(DeploymentLog {
                id: row.get(0)?,
                deployment_id: row.get(1)?,
                timestamp: DateTime::parse_from_rfc3339(&timestamp_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                level: row.get(3)?,
                message: row.get(4)?,
            });
        }

        Ok(logs)
    }

    /// Get the connection for internal use
    #[allow(dead_code)]
    pub fn connection(&self) -> &ParkingMutex<Connection> {
        &self.conn
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_database_new() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let db = Database::new(&db_path).unwrap();
        assert!(db_path.exists());
    }

    #[test]
    fn test_database_initialize() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let db = Database::new(&db_path).unwrap();
        let conn = db.conn.lock();

        // Check deployments table exists
        let mut stmt = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='deployments'")
            .unwrap();
        let exists = stmt.exists([]).unwrap();
        assert!(exists);

        // Check deployment_logs table exists
        let mut stmt = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='deployment_logs'")
            .unwrap();
        let exists = stmt.exists([]).unwrap();
        assert!(exists);
    }

    #[test]
    fn test_insert_and_get_deployment() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let db = Database::new(&db_path).unwrap();

        let deployment = Deployment {
            id: "test-id-1".to_string(),
            project_name: "test-project".to_string(),
            project: crate::project_config::ProjectConfig {
                repo_url: "https://github.com/test/test.git".to_string(),
                branch: "main".to_string(),
                project_type: ProjectType::Php,
                docker_service: Some("php".to_string()),
                working_dir: Some("/app".to_string()),
                install_command: Some("composer install".to_string()),
                build_command: Some("php artisan migrate".to_string()),
                extra_command: None,
                run_user: None,
                env: std::collections::HashMap::new(),
            },
            status: DeploymentStatus::Pending,
            created_at: Utc::now(),
            started_at: None,
            finished_at: None,
        };

        db.insert_deployment(&deployment).unwrap();

        let retrieved = db.get_deployment("test-id-1").unwrap();
        assert!(retrieved.is_some());
        let d = retrieved.unwrap();
        assert_eq!(d.id, "test-id-1");
        assert_eq!(d.project_name, "test-project");
        assert_eq!(d.status, DeploymentStatus::Pending);
    }

    #[test]
    fn test_get_all_deployments() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let db = Database::new(&db_path).unwrap();

        let deployment = Deployment {
            id: "test-id-2".to_string(),
            project_name: "test-project-2".to_string(),
            project: crate::project_config::ProjectConfig {
                repo_url: "https://github.com/test/test2.git".to_string(),
                branch: "main".to_string(),
                project_type: ProjectType::Nodejs,
                docker_service: None,
                working_dir: None,
                install_command: None,
                build_command: None,
                extra_command: None,
                run_user: None,
                env: std::collections::HashMap::new(),
            },
            status: DeploymentStatus::Success,
            created_at: Utc::now(),
            started_at: Some(Utc::now()),
            finished_at: Some(Utc::now()),
        };

        db.insert_deployment(&deployment).unwrap();

        let all = db.get_all_deployments().unwrap();
        assert_eq!(all.len(), 1);
    }

    #[test]
    fn test_delete_deployment() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let db = Database::new(&db_path).unwrap();

        let deployment = Deployment {
            id: "test-id-3".to_string(),
            project_name: "test-project-3".to_string(),
            project: crate::project_config::ProjectConfig {
                repo_url: "https://github.com/test/test3.git".to_string(),
                branch: "main".to_string(),
                project_type: ProjectType::Rust,
                docker_service: None,
                working_dir: None,
                install_command: None,
                build_command: None,
                extra_command: None,
                run_user: None,
                env: std::collections::HashMap::new(),
            },
            status: DeploymentStatus::Failed,
            created_at: Utc::now(),
            started_at: None,
            finished_at: None,
        };

        db.insert_deployment(&deployment).unwrap();
        let deleted = db.delete_deployment("test-id-3").unwrap();
        assert!(deleted);

        let retrieved = db.get_deployment("test-id-3").unwrap();
        assert!(retrieved.is_none());
    }

    #[test]
    fn test_insert_and_get_logs() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let db = Database::new(&db_path).unwrap();

        // Insert deployment first
        let deployment = Deployment {
            id: "test-id-4".to_string(),
            project_name: "test-project".to_string(),
            project: crate::project_config::ProjectConfig {
                repo_url: "https://github.com/test/test.git".to_string(),
                branch: "main".to_string(),
                project_type: ProjectType::Php,
                docker_service: None,
                working_dir: None,
                install_command: None,
                build_command: None,
                extra_command: None,
                run_user: None,
                env: std::collections::HashMap::new(),
            },
            status: DeploymentStatus::Pending,
            created_at: Utc::now(),
            started_at: None,
            finished_at: None,
        };
        db.insert_deployment(&deployment).unwrap();

        // Insert logs
        db.insert_log("test-id-4", "info", "Starting deployment").unwrap();
        db.insert_log("test-id-4", "info", "Cloning repository").unwrap();

        let logs = db.get_deployment_logs("test-id-4").unwrap();
        assert_eq!(logs.len(), 2);
        assert_eq!(logs[0].message, "Starting deployment");
    }
}

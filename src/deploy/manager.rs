//! Deployment manager

use crate::database::Database;
use crate::project_config::ProjectConfig;
use chrono::{DateTime, Utc};
use parking_lot::Mutex;
use std::collections::VecDeque;
use std::fmt;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;

/// Deployment status
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DeploymentStatus {
    Pending,
    Running,
    Success,
    Failed,
    Cancelled,
}

impl fmt::Display for DeploymentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeploymentStatus::Pending => write!(f, "Pending"),
            DeploymentStatus::Running => write!(f, "Running"),
            DeploymentStatus::Success => write!(f, "Success"),
            DeploymentStatus::Failed => write!(f, "Failed"),
            DeploymentStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}

/// Deployment task
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Deployment {
    pub id: String,
    pub project_name: String,
    pub project: ProjectConfig,
    pub status: DeploymentStatus,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
}

impl Deployment {
    /// Get started_at as local time string (UTC+8)
    pub fn started_at_local(&self) -> Option<String> {
        self.started_at.map(|t| {
            let local = t + chrono::Duration::hours(8);
            local.format("%Y-%m-%d %H:%M:%S").to_string()
        })
    }

    /// Get finished_at as local time string (UTC+8)
    pub fn finished_at_local(&self) -> Option<String> {
        self.finished_at.map(|t| {
            let local = t + chrono::Duration::hours(8);
            local.format("%Y-%m-%d %H:%M:%S").to_string()
        })
    }

    /// Get created_at as local time string (UTC+8)
    pub fn created_at_local(&self) -> String {
        let local = self.created_at + chrono::Duration::hours(8);
        local.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

/// Deployment manager
pub struct DeploymentManager {
    queue: Arc<Mutex<VecDeque<Deployment>>>,
    db: Arc<Database>,
    log_sender: broadcast::Sender<String>,
}

impl DeploymentManager {
    /// Create a new deployment manager with SQLite persistence
    pub fn new<P: AsRef<Path>>(db_path: P) -> anyhow::Result<Self> {
        let db = Arc::new(Database::new(db_path)?);
        let (log_sender, _) = broadcast::channel(1000);
        Ok(Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            db,
            log_sender,
        })
    }

    /// Get a receiver for log updates
    pub fn subscribe_logs(&self) -> broadcast::Receiver<String> {
        self.log_sender.subscribe()
    }

    /// Queue a new deployment
    /// Returns Some(id) if successfully queued, None if duplicate task exists
    pub fn queue_deployment(&self, project_name: String, project: ProjectConfig) -> Option<String> {
        let mut queue = self.queue.lock();

        // Check for duplicate: same project_name + branch with Pending or Running status
        let branch = &project.branch;
        let is_duplicate = queue.iter().any(|d| {
            d.project_name == project_name
            && d.project.branch == *branch
            && (d.status == DeploymentStatus::Pending || d.status == DeploymentStatus::Running)
        });

        if is_duplicate {
            return None;
        }

        let id = Uuid::new_v4().to_string();
        let deployment = Deployment {
            id: id.clone(),
            project_name: project_name.clone(),
            project: project.clone(),
            status: DeploymentStatus::Pending,
            created_at: Utc::now(),
            started_at: None,
            finished_at: None,
        };

        // Persist to database
        if let Err(e) = self.db.insert_deployment(&deployment) {
            tracing::error!("Failed to persist deployment: {}", e);
        }

        queue.push_back(deployment);
        Some(id)
    }

    /// Get deployment status
    pub fn get_deployment(&self, id: &str) -> Option<Deployment> {
        // First check in-memory queue
        let queue = self.queue.lock();
        if let Some(deployment) = queue.iter().find(|d| d.id == id) {
            return Some(deployment.clone());
        }
        drop(queue);

        // Fall back to database
        self.db.get_deployment(id).ok().flatten()
    }

    /// Pop the next pending deployment from the queue
    pub fn pop_deployment(&self) -> Option<Deployment> {
        let mut queue = self.queue.lock();
        // Find the first pending deployment
        let index = queue.iter().position(|d| d.status == DeploymentStatus::Pending);
        if let Some(idx) = index {
            queue.remove(idx)
        } else {
            None
        }
    }

    /// Update deployment status
    pub fn update_status(&self, id: &str, status: DeploymentStatus) -> bool {
        // Update in-memory queue
        let mut queue = self.queue.lock();
        let started_at;
        let finished_at;

        if let Some(deployment) = queue.iter_mut().find(|d| d.id == id) {
            deployment.status = status.clone();
            match status {
                DeploymentStatus::Running => {
                    deployment.started_at = Some(Utc::now());
                    started_at = Some(Utc::now());
                    finished_at = None;
                }
                DeploymentStatus::Success | DeploymentStatus::Failed | DeploymentStatus::Cancelled => {
                    deployment.finished_at = Some(Utc::now());
                    started_at = deployment.started_at;
                    finished_at = Some(Utc::now());
                }
                _ => {
                    started_at = None;
                    finished_at = None;
                }
            }
            drop(queue);

            // Persist to database
            if let Err(e) = self.db.update_deployment_status(id, &status, started_at, finished_at) {
                tracing::error!("Failed to update deployment status in DB: {}", e);
            }

            // Broadcast status change to WebSocket subscribers
            let status_msg = serde_json::json!({
                "type": "status",
                "status": status.to_string(),
                "deployment_id": id,
            });
            let _ = self.log_sender.send(status_msg.to_string());

            true
        } else {
            drop(queue);
            // Deployment not in queue (already popped), get from database and update based on status
            let (db_started_at, db_finished_at) = if let Ok(Some(existing)) = self.db.get_deployment(id) {
                (existing.started_at, existing.finished_at)
            } else {
                (None, None)
            };

            // Update timestamps based on status
            match status {
                DeploymentStatus::Running => {
                    // If already running, keep existing started_at; otherwise set now
                    started_at = db_started_at.or_else(|| Some(Utc::now()));
                    finished_at = None;
                }
                DeploymentStatus::Success | DeploymentStatus::Failed | DeploymentStatus::Cancelled => {
                    started_at = db_started_at;
                    finished_at = Some(Utc::now());
                }
                _ => {
                    started_at = db_started_at;
                    finished_at = db_finished_at;
                }
            }

            let result = self.db.update_deployment_status(id, &status, started_at, finished_at);

            if result.is_ok() {
                // Broadcast status change to WebSocket subscribers
                let status_msg = serde_json::json!({
                    "type": "status",
                    "status": status.to_string(),
                    "deployment_id": id,
                });
                let _ = self.log_sender.send(status_msg.to_string());
            }

            result.is_ok()
        }
    }

    /// Cancel a deployment
    #[allow(dead_code)]
    pub fn cancel_deployment(&self, id: &str) -> bool {
        let mut queue = self.queue.lock();
        if let Some(deployment) = queue.iter_mut().find(|d| d.id == id) {
            // Only pending deployments can be cancelled
            if deployment.status == DeploymentStatus::Pending {
                deployment.status = DeploymentStatus::Cancelled;
                deployment.finished_at = Some(Utc::now());
                let finished_at = deployment.finished_at;
                let started_at = deployment.started_at;
                drop(queue);

                // Persist to database
                let _ = self.db.update_deployment_status(id, &DeploymentStatus::Cancelled, started_at, finished_at);
                return true;
            }
        }
        false
    }

    /// Get all deployments
    pub fn get_all_deployments(&self) -> Vec<Deployment> {
        // First check in-memory queue
        let queue = self.queue.lock();
        let in_memory: Vec<Deployment> = queue.iter().cloned().collect();
        drop(queue);

        // Get from database
        let from_db = self.db.get_all_deployments().unwrap_or_default();

        // Merge, preferring in-memory (more current) and removing duplicates
        let mut all: Vec<Deployment> = from_db;
        for m in in_memory {
            if !all.iter().any(|d| d.id == m.id) {
                all.push(m);
            }
        }

        // Sort by created_at descending
        all.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        all
    }

    /// Delete a deployment (only pending)
    pub fn delete_deployment(&self, id: &str) -> bool {
        // Check in-memory first
        let mut queue = self.queue.lock();
        if let Some(idx) = queue.iter().position(|d| d.id == id && d.status == DeploymentStatus::Pending) {
            queue.remove(idx);
            drop(queue);
            // Also delete from database
            let _ = self.db.delete_deployment(id);
            return true;
        }
        drop(queue);

        // Check database
        if let Ok(Some(deployment)) = self.db.get_deployment(id) {
            if deployment.status == DeploymentStatus::Pending {
                return self.db.delete_deployment(id).unwrap_or(false);
            }
        }
        false
    }

    /// Retry a failed deployment
    pub fn retry_deployment(&self, id: &str) -> bool {
        // Check in-memory
        let mut queue = self.queue.lock();
        if let Some(deployment) = queue.iter_mut().find(|d| d.id == id && d.status == DeploymentStatus::Failed) {
            deployment.status = DeploymentStatus::Pending;
            deployment.started_at = None;
            deployment.finished_at = None;
            let _dep = deployment.clone();
            drop(queue);

            // Update database
            let _ = self.db.update_deployment_status(id, &DeploymentStatus::Pending, None, None);
            return true;
        }
        drop(queue);

        // Check database
        if let Ok(Some(deployment)) = self.db.get_deployment(id) {
            if deployment.status == DeploymentStatus::Failed {
                // Re-add to queue
                let mut queue = self.queue.lock();
                let new_deployment = Deployment {
                    id: deployment.id.clone(),
                    project_name: deployment.project_name.clone(),
                    project: deployment.project,
                    status: DeploymentStatus::Pending,
                    created_at: Utc::now(),
                    started_at: None,
                    finished_at: None,
                };
                queue.push_back(new_deployment);
                drop(queue);

                // Update database
                let _ = self.db.update_deployment_status(id, &DeploymentStatus::Pending, None, None);
                return true;
            }
        }
        false
    }

    /// Add log for a deployment
    pub fn add_log(&self, deployment_id: &str, level: &str, message: &str) {
        // Insert to database
        if let Err(e) = self.db.insert_log(deployment_id, level, message) {
            tracing::error!("Failed to insert log: {}", e);
        }

        // Broadcast to WebSocket subscribers
        let log_json = serde_json::json!({
            "deployment_id": deployment_id,
            "level": level,
            "message": message,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });
        let _ = self.log_sender.send(log_json.to_string());
    }

    /// Get logs for a deployment
    pub fn get_logs(&self, deployment_id: &str) -> Vec<crate::database::DeploymentLog> {
        self.db.get_deployment_logs(deployment_id).unwrap_or_default()
    }

    /// Get database reference
    #[allow(dead_code)]
    pub fn database(&self) -> &Arc<Database> {
        &self.db
    }
}

impl Default for DeploymentManager {
    fn default() -> Self {
        // This will create a default in-memory manager (not persisted)
        // Use DeploymentManager::new() for persistent storage
        let (log_sender, _) = broadcast::channel(1000);
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            db: Arc::new(Database::new(":memory:").expect("Failed to create in-memory DB")),
            log_sender,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ProjectType;
    use std::collections::HashMap;
    use tempfile::TempDir;

    fn create_test_project_config() -> ProjectConfig {
        ProjectConfig {
            repo_url: "https://github.com/test/test.git".to_string(),
            branch: "main".to_string(),
            project_type: ProjectType::Php,
            docker_service: Some("php".to_string()),
            working_dir: Some("/app".to_string()),
            install_command: Some("composer install".to_string()),
            build_command: Some("php artisan migrate".to_string()),
            extra_command: None,
            env: HashMap::new(),
        }
    }

    #[test]
    fn test_deployment_manager_new() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let manager = DeploymentManager::new(&db_path).unwrap();
        let queue = manager.queue.lock();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_queue_deployment() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let manager = DeploymentManager::new(&db_path).unwrap();
        let project = create_test_project_config();

        let id = manager.queue_deployment("test-project".to_string(), project);

        assert!(id.is_some());
        assert!(!id.unwrap().is_empty());
        let queue = manager.queue.lock();
        assert_eq!(queue.len(), 1);
    }

    #[test]
    fn test_get_deployment_found() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let manager = DeploymentManager::new(&db_path).unwrap();
        let project = create_test_project_config();

        let id = manager.queue_deployment("test-project".to_string(), project).unwrap();
        let deployment = manager.get_deployment(&id);

        assert!(deployment.is_some());
        let d = deployment.unwrap();
        assert_eq!(d.project_name, "test-project");
        assert_eq!(d.status, DeploymentStatus::Pending);
    }

    #[test]
    fn test_delete_deployment_pending() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let manager = DeploymentManager::new(&db_path).unwrap();
        let project = create_test_project_config();

        let id = manager.queue_deployment("test-project".to_string(), project).unwrap();
        let deleted = manager.delete_deployment(&id);

        assert!(deleted);
        assert!(manager.get_deployment(&id).is_none());
    }

    #[test]
    fn test_delete_deployment_running_fails() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let manager = DeploymentManager::new(&db_path).unwrap();
        let project = create_test_project_config();

        let id = manager.queue_deployment("test-project".to_string(), project).unwrap();
        manager.update_status(&id, DeploymentStatus::Running);

        let deleted = manager.delete_deployment(&id);
        assert!(!deleted);
    }

    #[test]
    fn test_retry_deployment_failed() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let manager = DeploymentManager::new(&db_path).unwrap();
        let project = create_test_project_config();

        let id = manager.queue_deployment("test-project".to_string(), project).unwrap();
        manager.update_status(&id, DeploymentStatus::Failed);

        let retried = manager.retry_deployment(&id);
        assert!(retried);

        let deployment = manager.get_deployment(&id).unwrap();
        assert_eq!(deployment.status, DeploymentStatus::Pending);
    }

    #[test]
    fn test_add_and_get_logs() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let manager = DeploymentManager::new(&db_path).unwrap();
        let project = create_test_project_config();

        let id = manager.queue_deployment("test-project".to_string(), project).unwrap();
        manager.add_log(&id, "info", "Starting deployment");
        manager.add_log(&id, "info", "Cloning repository");

        let logs = manager.get_logs(&id);
        assert_eq!(logs.len(), 2);
        assert_eq!(logs[0].message, "Starting deployment");
    }
}

//! Repository Traits
//!
//! Abstraction traits for process instance persistence.

use crate::engine::instance::ProcessInstance;
use async_trait::async_trait;
use std::sync::Arc;

/// Repository Trait
///
/// Trait for process instance repositories.
/// This abstraction allows for different storage backends (memory, database, etc.).
#[async_trait]
pub trait Repository: Send + Sync {
    /// Save a process instance
    async fn save(&self, instance: Arc<ProcessInstance>) -> Result<(), RepositoryError>;

    /// Get a process instance by ID
    async fn get(&self, instance_id: &str) -> Result<Option<Arc<ProcessInstance>>, RepositoryError>;

    /// Delete a process instance
    async fn delete(&self, instance_id: &str) -> Result<(), RepositoryError>;

    /// List all process instance IDs
    async fn list_ids(&self) -> Result<Vec<String>, RepositoryError>;

    /// Check if a process instance exists
    async fn exists(&self, instance_id: &str) -> Result<bool, RepositoryError> {
        Ok(self.get(instance_id).await?.is_some())
    }
}

/// Repository Error
#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Repository operation failed: {0}")]
    OperationFailed(String),
    #[error("Instance not found: {0}")]
    NotFound(String),
    #[error("Storage error: {0}")]
    StorageError(String),
}


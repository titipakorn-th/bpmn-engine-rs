//! Memory Repository
//!
//! In-memory implementation of the repository for process instances.
//!
//! This is the initial implementation that stores process instances in memory.
//! Designed to be replaced with a database-backed implementation in the future.

use crate::engine::instance::ProcessInstance;
use crate::repository::{Repository, RepositoryError};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Memory Repository
///
/// In-memory storage for process instances.
#[derive(Debug, Clone)]
pub struct MemoryRepository {
    instances: Arc<RwLock<HashMap<String, Arc<ProcessInstance>>>>,
}

impl MemoryRepository {
    /// Create a new memory repository
    pub fn new() -> Self {
        Self {
            instances: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl Repository for MemoryRepository {
    async fn save(&self, instance: Arc<ProcessInstance>) -> Result<(), RepositoryError> {
        let mut instances = self.instances.write().await;
        instances.insert(instance.id().to_string(), instance);
        Ok(())
    }

    async fn get(&self, instance_id: &str) -> Result<Option<Arc<ProcessInstance>>, RepositoryError> {
        let instances = self.instances.read().await;
        Ok(instances.get(instance_id).cloned())
    }

    async fn delete(&self, instance_id: &str) -> Result<(), RepositoryError> {
        let mut instances = self.instances.write().await;
        instances.remove(instance_id);
        Ok(())
    }

    async fn list_ids(&self) -> Result<Vec<String>, RepositoryError> {
        let instances = self.instances.read().await;
        Ok(instances.keys().cloned().collect())
    }
}

impl Default for MemoryRepository {
    fn default() -> Self {
        Self::new()
    }
}


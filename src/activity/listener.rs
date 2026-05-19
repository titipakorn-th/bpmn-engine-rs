//! Activity Listeners
//!
//! Listener and hook mechanisms for process execution events.

use crate::activity::ActivityResult;
use crate::engine::ExecutionContext;
use async_trait::async_trait;
use std::sync::Arc;

/// Process Execution Listener
///
/// Listener for process execution events.
#[async_trait]
pub trait ProcessListener: Send + Sync + std::fmt::Debug {
    /// Called when a process instance starts
    async fn on_process_start(&self, context: &ExecutionContext) -> Result<(), ListenerError>;

    /// Called when a process instance completes
    async fn on_process_complete(&self, context: &ExecutionContext) -> Result<(), ListenerError>;

    /// Called when a process instance fails
    async fn on_process_fail(&self, context: &ExecutionContext, error: &str) -> Result<(), ListenerError>;

    /// Called before an activity executes
    async fn on_activity_start(
        &self,
        context: &ExecutionContext,
        activity_id: &str,
    ) -> Result<(), ListenerError>;

    /// Called after an activity executes
    async fn on_activity_complete(
        &self,
        context: &ExecutionContext,
        activity_id: &str,
        result: &ActivityResult,
    ) -> Result<(), ListenerError>;

    /// Called when an activity fails
    async fn on_activity_fail(
        &self,
        context: &ExecutionContext,
        activity_id: &str,
        error: &str,
    ) -> Result<(), ListenerError>;
}

/// Listener Error
#[derive(Debug, thiserror::Error)]
pub enum ListenerError {
    #[error("Listener execution failed: {0}")]
    ExecutionFailed(String),
}

/// Listener Registry
///
/// Registry for managing process listeners.
#[derive(Clone, Debug)]
pub struct ListenerRegistry {
    listeners: Arc<tokio::sync::RwLock<Vec<Arc<dyn ProcessListener>>>>,
}

impl ListenerRegistry {
    /// Create a new listener registry
    pub fn new() -> Self {
        Self {
            listeners: Arc::new(tokio::sync::RwLock::new(Vec::new())),
        }
    }

    /// Register a listener
    pub async fn register(&self, listener: Arc<dyn ProcessListener>) {
        let mut listeners = self.listeners.write().await;
        listeners.push(listener);
    }

    /// Get all registered listeners
    pub async fn get_listeners(&self) -> Vec<Arc<dyn ProcessListener>> {
        let listeners = self.listeners.read().await;
        listeners.clone()
    }
}

impl Default for ListenerRegistry {
    fn default() -> Self {
        Self::new()
    }
}


//! Process Instance
//!
//! Process instance management for BPMN processes.

use crate::engine::context::ExecutionContext;
use crate::model::ProcessDefinition;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Process Instance
///
/// Represents a running instance of a BPMN process.
#[derive(Debug, Clone)]
pub struct ProcessInstance {
    /// Instance ID
    pub id: String,
    /// Process definition
    pub definition: Arc<ProcessDefinition>,
    /// Execution context
    pub context: Arc<RwLock<ExecutionContext>>,
}

impl ProcessInstance {
    /// Create a new process instance
    pub fn new(definition: Arc<ProcessDefinition>, instance_id: String) -> Self {
        let context = ExecutionContext::new((*definition).clone(), instance_id.clone());
        Self {
            id: instance_id,
            definition,
            context: Arc::new(RwLock::new(context)),
        }
    }

    /// Get the instance ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get the process definition
    pub fn definition(&self) -> &Arc<ProcessDefinition> {
        &self.definition
    }

    /// Get a read lock on the execution context
    pub async fn context(&self) -> tokio::sync::RwLockReadGuard<'_, ExecutionContext> {
        self.context.read().await
    }

    /// Get a write lock on the execution context
    pub async fn context_mut(&self) -> tokio::sync::RwLockWriteGuard<'_, ExecutionContext> {
        self.context.write().await
    }
}


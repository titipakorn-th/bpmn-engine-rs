//! Task Elements
//!
//! Implementation of BPMN task elements with Activity/Capability traits.

use crate::activity::{Activity, ActivityError, ActivityResult};
use crate::capability::{Capability, CapabilityError, CapabilityResult, CapabilityProvider};
use crate::engine::ExecutionContext;
use crate::model::{ServiceTask, UserTask, ScriptTask, ManualTask};
use async_trait::async_trait;
use std::collections::HashMap;

/// Service Task Activity
///
/// Implements Activity trait for ServiceTask elements.
pub struct ServiceTaskActivity {
    task: ServiceTask,
}

impl ServiceTaskActivity {
    pub fn new(task: ServiceTask) -> Self {
        Self { task }
    }
}

#[async_trait]
impl Activity for ServiceTaskActivity {
    async fn execute(&self, context: &mut ExecutionContext) -> Result<ActivityResult, ActivityError> {
        // TODO: Implement service task execution
        // For now, just complete immediately
        Ok(ActivityResult::Completed { output_variables: None })
    }

    fn id(&self) -> &str {
        &self.task.base.id
    }

    fn name(&self) -> Option<&str> {
        self.task.base.name.as_deref()
    }
}

/// User Task Activity
///
/// Implements Activity trait for UserTask elements.
pub struct UserTaskActivity {
    task: UserTask,
}

impl UserTaskActivity {
    pub fn new(task: UserTask) -> Self {
        Self { task }
    }
}

#[async_trait]
impl Activity for UserTaskActivity {
    async fn execute(&self, context: &mut ExecutionContext) -> Result<ActivityResult, ActivityError> {
        // User tasks wait for user input
        Ok(ActivityResult::Waiting {
            reason: format!("User task '{}' waiting for user input", self.task.base.id),
        })
    }

    fn id(&self) -> &str {
        &self.task.base.id
    }

    fn name(&self) -> Option<&str> {
        self.task.base.name.as_deref()
    }
}

/// Script Task Activity
///
/// Implements Activity trait for ScriptTask elements.
pub struct ScriptTaskActivity {
    task: ScriptTask,
}

impl ScriptTaskActivity {
    pub fn new(task: ScriptTask) -> Self {
        Self { task }
    }
}

#[async_trait]
impl Activity for ScriptTaskActivity {
    async fn execute(&self, context: &mut ExecutionContext) -> Result<ActivityResult, ActivityError> {
        // TODO: Implement script execution
        // For now, just complete immediately
        Ok(ActivityResult::Completed { output_variables: None })
    }

    fn id(&self) -> &str {
        &self.task.base.id
    }

    fn name(&self) -> Option<&str> {
        self.task.base.name.as_deref()
    }
}

/// Manual Task Activity
///
/// Implements Activity trait for ManualTask elements.
pub struct ManualTaskActivity {
    task: ManualTask,
}

impl ManualTaskActivity {
    pub fn new(task: ManualTask) -> Self {
        Self { task }
    }
}

#[async_trait]
impl Activity for ManualTaskActivity {
    async fn execute(&self, context: &mut ExecutionContext) -> Result<ActivityResult, ActivityError> {
        // Manual tasks are completed immediately (they represent manual work outside the system)
        Ok(ActivityResult::Completed { output_variables: None })
    }

    fn id(&self) -> &str {
        &self.task.base.id
    }

    fn name(&self) -> Option<&str> {
        self.task.base.name.as_deref()
    }
}

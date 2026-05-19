//! Task Elements
//!
//! Implementation of BPMN task elements with Activity/Capability traits.

use crate::activity::{Activity, ActivityError, ActivityResult};
use crate::capability::Capability;
use crate::engine::ExecutionContext;
use crate::model::{ServiceTask, UserTask, ScriptTask, ManualTask, CallActivity};
use async_trait::async_trait;

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
        // Handle multi-instance loop characteristics
        if let Some(mi) = &self.task.loop_characteristics {
            let count = mi.loop_cardinality.unwrap_or(1);
            context.set_variable(
                format!("_mi_count_{}", self.id()),
                serde_json::json!(count)
            );
            context.set_variable(
                format!("_mi_completed_{}", self.id()),
                serde_json::json!(0)
            );
            context.set_variable(
                format!("_mi_active_{}", self.id()),
                serde_json::json!(0)
            );
        }
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
        // Handle multi-instance loop characteristics
        if let Some(mi) = &self.task.loop_characteristics {
            let count = mi.loop_cardinality.unwrap_or(1);
            context.set_variable(
                format!("_mi_count_{}", self.id()),
                serde_json::json!(count)
            );
            context.set_variable(
                format!("_mi_completed_{}", self.id()),
                serde_json::json!(0)
            );
            context.set_variable(
                format!("_mi_active_{}", self.id()),
                serde_json::json!(0)
            );
        }
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
        // Handle multi-instance loop characteristics
        if let Some(mi) = &self.task.loop_characteristics {
            let count = mi.loop_cardinality.unwrap_or(1);
            context.set_variable(
                format!("_mi_count_{}", self.id()),
                serde_json::json!(count)
            );
            context.set_variable(
                format!("_mi_completed_{}", self.id()),
                serde_json::json!(0)
            );
            context.set_variable(
                format!("_mi_active_{}", self.id()),
                serde_json::json!(0)
            );
        }
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
    async fn execute(&self, _context: &mut ExecutionContext) -> Result<ActivityResult, ActivityError> {
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

/// Call Activity Task
///
/// Implements Activity trait for CallActivity elements.
/// Call Activities reference external subprocesses.
pub struct CallActivityTask {
    task: CallActivity,
}

impl CallActivityTask {
    pub fn new(task: CallActivity) -> Self {
        Self { task }
    }
}

#[async_trait]
impl Activity for CallActivityTask {
    async fn execute(&self, _context: &mut ExecutionContext) -> Result<ActivityResult, ActivityError> {
        match &self.task.called_element {
            Some(_called_id) => {
                // Call activities signal the platform to invoke a subprocess.
                // For now, complete immediately - the platform handles subprocess invocation.
                // The called_element attribute specifies which process to invoke.
                Ok(ActivityResult::Completed { output_variables: None })
            }
            None => {
                // No called element specified - complete immediately
                Ok(ActivityResult::Completed { output_variables: None })
            }
        }
    }

    fn id(&self) -> &str {
        &self.task.base.id
    }

    fn name(&self) -> Option<&str> {
        self.task.base.name.as_deref()
    }
}

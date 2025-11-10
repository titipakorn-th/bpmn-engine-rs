//! Activity Traits
//!
//! Activity trait definitions for BPMN elements that can be executed.
//!
//! Activities represent executable units in a BPMN process.
//! Each activity implements the Activity trait to define its execution behavior.

use crate::engine::ExecutionContext;
use crate::model::ProcessElement;
use async_trait::async_trait;
use std::sync::Arc;

/// Activity Trait
///
/// Represents an executable activity in a BPMN process.
/// Activities can be tasks, gateways, events, or other executable elements.
#[async_trait]
pub trait Activity: Send + Sync {
    /// Execute the activity
    ///
    /// # Arguments
    /// * `context` - Execution context containing process state, variables, etc.
    ///
    /// # Returns
    /// * `Ok(ActivityResult)` - Activity execution result
    /// * `Err(ActivityError)` - Activity execution error
    async fn execute(&self, context: &mut ExecutionContext) -> Result<ActivityResult, ActivityError>;

    /// Get the activity ID
    fn id(&self) -> &str;

    /// Get the activity name
    fn name(&self) -> Option<&str>;
}

/// Activity Result
///
/// Result of executing an activity.
#[derive(Debug, Clone)]
pub enum ActivityResult {
    /// Activity completed successfully
    Completed {
        /// Output variables (if any)
        output_variables: Option<std::collections::HashMap<String, serde_json::Value>>,
    },
    /// Activity is waiting (e.g., user task waiting for user input)
    Waiting {
        /// Wait reason
        reason: String,
    },
    /// Activity needs to continue to next element(s)
    Continue {
        /// Next element IDs to execute
        next_elements: Vec<String>,
    },
}

/// Activity Error
///
/// Error that occurred during activity execution.
#[derive(Debug, Clone, thiserror::Error)]
pub enum ActivityError {
    #[error("Activity execution failed: {0}")]
    ExecutionFailed(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Activity not found: {0}")]
    NotFound(String),
    #[error("Condition evaluation failed: {0}")]
    ConditionEvaluationFailed(String),
}

/// Activity Factory
///
/// Factory for creating Activity instances from ProcessElement.
pub trait ActivityFactory: Send + Sync + std::fmt::Debug {
    /// Create an Activity from a ProcessElement
    fn create_activity(&self, element: &ProcessElement) -> Result<Arc<dyn Activity>, ActivityError>;
}


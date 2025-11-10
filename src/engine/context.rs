//! Execution Context
//!
//! Execution context for BPMN process execution.
//!
//! The execution context holds the state of a process instance during execution,
//! including variables, current element, and execution history.

use crate::model::ProcessDefinition;
use std::collections::HashMap;

/// Execution Context
///
/// Context for executing a BPMN process instance.
/// Contains process state, variables, and execution information.
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    /// Process definition
    pub process_definition: ProcessDefinition,
    /// Process instance ID
    pub instance_id: String,
    /// Current element IDs being executed
    pub current_elements: Vec<String>,
    /// Process variables
    pub variables: HashMap<String, serde_json::Value>,
    /// Execution history
    pub execution_history: Vec<ExecutionStep>,
    /// Process state
    pub state: ProcessInstanceState,
}

impl ExecutionContext {
    /// Create a new execution context
    pub fn new(process_definition: ProcessDefinition, instance_id: String) -> Self {
        Self {
            process_definition,
            instance_id,
            current_elements: Vec::new(),
            variables: HashMap::new(),
            execution_history: Vec::new(),
            state: ProcessInstanceState::Active,
        }
    }

    /// Set a variable
    pub fn set_variable(&mut self, name: String, value: serde_json::Value) {
        self.variables.insert(name, value);
    }

    /// Get a variable
    pub fn get_variable(&self, name: &str) -> Option<&serde_json::Value> {
        self.variables.get(name)
    }

    /// Add execution step to history
    pub fn add_execution_step(&mut self, step: ExecutionStep) {
        self.execution_history.push(step);
    }

    /// Set current elements
    pub fn set_current_elements(&mut self, element_ids: Vec<String>) {
        self.current_elements = element_ids;
    }

    /// Clear current elements
    pub fn clear_current_elements(&mut self) {
        self.current_elements.clear();
    }
}

/// Process Instance State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessInstanceState {
    /// Process is active and executing
    Active,
    /// Process is completed
    Completed,
    /// Process is terminated
    Terminated,
    /// Process is suspended
    Suspended,
    /// Process execution failed
    Failed,
}

/// Execution Step
///
/// Represents a step in the execution history.
#[derive(Debug, Clone)]
pub struct ExecutionStep {
    /// Element ID that was executed
    pub element_id: String,
    /// Timestamp of execution
    pub timestamp: std::time::SystemTime,
    /// Execution result
    pub result: ExecutionStepResult,
}

/// Execution Step Result
#[derive(Debug, Clone)]
pub enum ExecutionStepResult {
    /// Step completed successfully
    Completed,
    /// Step failed
    Failed(String),
    /// Step is waiting
    Waiting(String),
}


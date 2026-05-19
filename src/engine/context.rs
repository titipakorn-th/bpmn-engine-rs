//! Execution Context
//!
//! Execution context for BPMN process execution.
//!
//! The execution context holds the state of a process instance during execution,
//! including variables, current element, and execution history.

use crate::model::ProcessDefinition;
use std::collections::{HashMap, HashSet};

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
    /// Token counter for parallel gateway joins
    /// Key: gateway_id, Value: set of source element IDs that have arrived
    pub incoming_tokens: HashMap<String, HashSet<String>>,
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
            incoming_tokens: HashMap::new(),
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

    /// Add an incoming token to a gateway
    pub fn add_incoming_token(&mut self, gateway_id: String, source_id: String) {
        self.incoming_tokens
            .entry(gateway_id)
            .or_insert_with(HashSet::new)
            .insert(source_id);
    }

    /// Get tokens for a gateway
    pub fn get_incoming_tokens(&self, gateway_id: &str) -> HashSet<String> {
        self.incoming_tokens
            .get(gateway_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Clear tokens for a gateway (after join completes)
    pub fn clear_incoming_tokens(&mut self, gateway_id: &str) {
        self.incoming_tokens.remove(gateway_id);
    }

    /// Check if all tokens have arrived for a gateway
    pub fn all_tokens_arrived(&self, gateway_id: &str, required_count: usize) -> bool {
        self.incoming_tokens
            .get(gateway_id)
            .map(|tokens| tokens.len() >= required_count)
            .unwrap_or(false)
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

/// Gateway Direction (BPMN 2.0)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GatewayDirection {
    /// Gateway has diverging output (1 in → N out) — AND/XOR split
    Diverging,
    /// Gateway has converging input (N in → 1 out) — AND join
    Converging,
    /// Both converging and diverging (join-then-split)
    Mixed,
    /// Unable to determine direction
    Unknown,
}

impl Default for GatewayDirection {
    fn default() -> Self {
        GatewayDirection::Unknown
    }
}

/// Detect gateway direction from graph structure
pub fn detect_gateway_direction(
    gateway_id: &str,
    definition: &ProcessDefinition,
) -> GatewayDirection {
    let incoming = definition.get_incoming_flows(gateway_id).len();
    let outgoing = definition.get_outgoing_flows(gateway_id).len();

    match (incoming, outgoing) {
        (1, n) if n > 1 => GatewayDirection::Diverging,
        (n, 1) if n > 1 => GatewayDirection::Converging,
        (n, m) if n > 1 && m > 1 => GatewayDirection::Mixed,
        _ => GatewayDirection::Unknown,
    }
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


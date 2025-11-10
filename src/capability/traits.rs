//! Capability Traits
//!
//! Capability trait definitions for BPMN elements that define what they can do.
//!
//! Capabilities represent the abilities or features that BPMN elements provide.
//! This allows for a flexible, extensible design where elements can be composed
//! based on their capabilities.

use crate::engine::ExecutionContext;
use async_trait::async_trait;
use std::collections::HashMap;

/// Capability Trait
///
/// Represents a capability that a BPMN element can provide.
/// Capabilities define what an element can do, allowing for flexible composition.
#[async_trait]
pub trait Capability: Send + Sync {
    /// Get the capability name
    fn name(&self) -> &str;

    /// Check if this capability is available in the given context
    fn is_available(&self, context: &ExecutionContext) -> bool;

    /// Execute the capability
    ///
    /// # Arguments
    /// * `context` - Execution context
    /// * `parameters` - Capability-specific parameters
    ///
    /// # Returns
    /// * `Ok(CapabilityResult)` - Capability execution result
    /// * `Err(CapabilityError)` - Capability execution error
    async fn execute(
        &self,
        context: &mut ExecutionContext,
        parameters: &HashMap<String, serde_json::Value>,
    ) -> Result<CapabilityResult, CapabilityError>;
}

/// Capability Result
///
/// Result of executing a capability.
#[derive(Debug, Clone)]
pub struct CapabilityResult {
    /// Output data
    pub output: HashMap<String, serde_json::Value>,
    /// Whether the capability execution was successful
    pub success: bool,
}

/// Capability Error
///
/// Error that occurred during capability execution.
#[derive(Debug, thiserror::Error)]
pub enum CapabilityError {
    #[error("Capability execution failed: {0}")]
    ExecutionFailed(String),
    #[error("Capability not available: {0}")]
    NotAvailable(String),
    #[error("Invalid parameters: {0}")]
    InvalidParameters(String),
}

/// Capability Provider
///
/// Trait for elements that provide capabilities.
pub trait CapabilityProvider: Send + Sync {
    /// Get all capabilities provided by this element
    fn get_capabilities(&self) -> Vec<Box<dyn Capability>>;
}


//! Gateway Elements
//!
//! Implementation of BPMN gateway elements with Activity/Capability traits.

use crate::activity::{Activity, ActivityError, ActivityResult};
use crate::capability::{Capability, CapabilityError, CapabilityResult, CapabilityProvider};
use crate::engine::ExecutionContext;
use crate::model::{ExclusiveGateway, ParallelGateway, InclusiveGateway};
use async_trait::async_trait;
use std::collections::HashMap;

/// Exclusive Gateway Activity
///
/// Implements Activity trait for ExclusiveGateway elements.
/// Evaluates conditions on outgoing flows and selects one path.
pub struct ExclusiveGatewayActivity {
    gateway: ExclusiveGateway,
}

impl ExclusiveGatewayActivity {
    pub fn new(gateway: ExclusiveGateway) -> Self {
        Self { gateway }
    }
}

#[async_trait]
impl Activity for ExclusiveGatewayActivity {
    async fn execute(&self, context: &mut ExecutionContext) -> Result<ActivityResult, ActivityError> {
        let definition = &context.process_definition;
        let outgoing_flows = definition.get_outgoing_flows(&self.gateway.base.id);

        // Evaluate conditions on outgoing flows
        let mut selected_flow: Option<String> = None;

        for flow in outgoing_flows {
            // Check if this is the default flow
            if Some(&flow.id) == self.gateway.default_flow.as_ref() {
                // Default flow is selected if no other condition matches
                if selected_flow.is_none() {
                    selected_flow = Some(flow.target_ref.clone());
                }
                continue;
            }

            // Evaluate condition if present
            if let Some(condition) = &flow.condition_expression {
                // TODO: Implement condition evaluation
                // For now, assume first condition that exists is true
                if selected_flow.is_none() {
                    selected_flow = Some(flow.target_ref.clone());
                    break;
                }
            } else {
                // Flow without condition is always taken (if no other condition matched)
                if selected_flow.is_none() {
                    selected_flow = Some(flow.target_ref.clone());
                }
            }
        }

        match selected_flow {
            Some(target) => Ok(ActivityResult::Continue {
                next_elements: vec![target],
            }),
            None => Err(ActivityError::ExecutionFailed(
                "No outgoing flow selected from exclusive gateway".to_string(),
            )),
        }
    }

    fn id(&self) -> &str {
        &self.gateway.base.id
    }

    fn name(&self) -> Option<&str> {
        self.gateway.base.name.as_deref()
    }
}

/// Parallel Gateway Activity
///
/// Implements Activity trait for ParallelGateway elements.
/// Takes all outgoing flows (splitting) or waits for all incoming flows (joining).
pub struct ParallelGatewayActivity {
    gateway: ParallelGateway,
}

impl ParallelGatewayActivity {
    pub fn new(gateway: ParallelGateway) -> Self {
        Self { gateway }
    }
}

#[async_trait]
impl Activity for ParallelGatewayActivity {
    async fn execute(&self, context: &mut ExecutionContext) -> Result<ActivityResult, ActivityError> {
        let definition = &context.process_definition;
        let incoming_flows = definition.get_incoming_flows(&self.gateway.base.id);
        let outgoing_flows = definition.get_outgoing_flows(&self.gateway.base.id);

        // If there are incoming flows, this is a join gateway
        // For now, we assume this is a split (outgoing flows)
        if !outgoing_flows.is_empty() {
            let next_elements: Vec<String> = outgoing_flows
                .iter()
                .map(|flow| flow.target_ref.clone())
                .collect();
            Ok(ActivityResult::Continue { next_elements })
        } else {
            // Join gateway - wait for all incoming tokens
            // TODO: Implement proper token synchronization
            Ok(ActivityResult::Completed { output_variables: None })
        }
    }

    fn id(&self) -> &str {
        &self.gateway.base.id
    }

    fn name(&self) -> Option<&str> {
        self.gateway.base.name.as_deref()
    }
}

/// Inclusive Gateway Activity
///
/// Implements Activity trait for InclusiveGateway elements.
/// Takes flows where conditions evaluate to true.
pub struct InclusiveGatewayActivity {
    gateway: InclusiveGateway,
}

impl InclusiveGatewayActivity {
    pub fn new(gateway: InclusiveGateway) -> Self {
        Self { gateway }
    }
}

#[async_trait]
impl Activity for InclusiveGatewayActivity {
    async fn execute(&self, context: &mut ExecutionContext) -> Result<ActivityResult, ActivityError> {
        let definition = &context.process_definition;
        let outgoing_flows = definition.get_outgoing_flows(&self.gateway.base.id);

        // Evaluate conditions and select all flows where condition is true
        let mut selected_targets = Vec::new();

        for flow in outgoing_flows {
            // Check if this is the default flow
            if Some(&flow.id) == self.gateway.default_flow.as_ref() {
                // Default flow is selected if no other condition matches
                if selected_targets.is_empty() {
                    selected_targets.push(flow.target_ref.clone());
                }
                continue;
            }

            // Evaluate condition if present
            if let Some(condition) = &flow.condition_expression {
                // TODO: Implement condition evaluation
                // For now, assume condition is true
                selected_targets.push(flow.target_ref.clone());
            } else {
                // Flow without condition is always taken
                selected_targets.push(flow.target_ref.clone());
            }
        }

        if selected_targets.is_empty() {
            Err(ActivityError::ExecutionFailed(
                "No outgoing flow selected from inclusive gateway".to_string(),
            ))
        } else {
            Ok(ActivityResult::Continue {
                next_elements: selected_targets,
            })
        }
    }

    fn id(&self) -> &str {
        &self.gateway.base.id
    }

    fn name(&self) -> Option<&str> {
        self.gateway.base.name.as_deref()
    }
}

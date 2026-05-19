//! Gateway Elements
//!
//! Implementation of BPMN gateway elements with Activity/Capability traits.

use crate::activity::{Activity, ActivityError, ActivityResult};
use crate::capability::Capability;
use crate::engine::{ExecutionContext, evaluator::evaluate_condition};
use crate::model::{ExclusiveGateway, ParallelGateway, InclusiveGateway};
use async_trait::async_trait;

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
        let mut selected_flow: Option<&crate::model::SequenceFlow> = None;

        for flow in &outgoing_flows {
            // Check if this is the default flow
            if Some(&flow.id) == self.gateway.default_flow.as_ref() {
                // Default flow is selected if no other condition matches
                if selected_flow.is_none() {
                    selected_flow = Some(flow);
                }
                continue;
            }

            // Evaluate condition if present
            if let Some(ref condition) = flow.condition_expression {
                if evaluate_condition(condition, &serde_json::json!(context.variables)) {
                    selected_flow = Some(flow);
                    break; // First match wins for exclusive gateway
                }
            }
            // Flows without conditions are handled as fallback
        }

        // If no condition matched, use first unconditional flow
        if selected_flow.is_none() {
            selected_flow = outgoing_flows.iter().find(|f| f.condition_expression.is_none()).map(|v| &**v);
        }

        // Last resort: first flow
        if selected_flow.is_none() {
            selected_flow = outgoing_flows.first().map(|v| &**v);
        }

        match selected_flow {
            Some(flow) => Ok(ActivityResult::Continue {
                next_elements: vec![flow.target_ref.clone()],
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
        let _incoming_flows = definition.get_incoming_flows(&self.gateway.base.id);
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

        // Evaluate conditions and select ALL flows where condition is true
        let mut selected_targets = Vec::new();
        let mut default_flow_id: Option<String> = None;

        for flow in &outgoing_flows {
            // Track default flow for fallback
            if Some(&flow.id) == self.gateway.default_flow.as_ref() {
                default_flow_id = Some(flow.id.clone());
                continue;
            }

            // Evaluate condition if present
            if let Some(ref condition) = flow.condition_expression {
                if evaluate_condition(condition, &serde_json::json!(context.variables)) {
                    selected_targets.push(flow.target_ref.clone());
                }
            } else {
                // Flow without condition is always taken in inclusive gateway
                selected_targets.push(flow.target_ref.clone());
            }
        }

        // If nothing matched, use default flow
        if selected_targets.is_empty() {
            if let Some(default_id) = default_flow_id {
                if let Some(flow) = outgoing_flows.iter().find(|f| f.id == default_id) {
                    selected_targets.push(flow.target_ref.clone());
                }
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

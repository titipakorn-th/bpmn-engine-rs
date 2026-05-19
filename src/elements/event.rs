//! Event Elements
//!
//! Implementation of BPMN event elements with Activity/Capability traits.

use crate::activity::{Activity, ActivityError, ActivityResult};
use crate::capability::Capability;
use crate::engine::context::ProcessInstanceState;
use crate::engine::ExecutionContext;
use crate::model::{StartEvent, EndEvent, IntermediateCatchEvent, IntermediateThrowEvent};
use async_trait::async_trait;

/// Start Event Activity
///
/// Implements Activity trait for StartEvent elements.
pub struct StartEventActivity {
    event: StartEvent,
}

impl StartEventActivity {
    pub fn new(event: StartEvent) -> Self {
        Self { event }
    }
}

#[async_trait]
impl Activity for StartEventActivity {
    async fn execute(&self, context: &mut ExecutionContext) -> Result<ActivityResult, ActivityError> {
        // Start events immediately continue to next elements
        let definition = &context.process_definition;
        let outgoing_flows = definition.get_outgoing_flows(&self.event.base.id);
        let next_elements: Vec<String> = outgoing_flows
            .iter()
            .map(|flow| flow.target_ref.clone())
            .collect();

        Ok(ActivityResult::Continue { next_elements })
    }

    fn id(&self) -> &str {
        &self.event.base.id
    }

    fn name(&self) -> Option<&str> {
        self.event.base.name.as_deref()
    }
}

/// End Event Activity
///
/// Implements Activity trait for EndEvent elements.
pub struct EndEventActivity {
    event: EndEvent,
}

impl EndEventActivity {
    pub fn new(event: EndEvent) -> Self {
        Self { event }
    }
}

#[async_trait]
impl Activity for EndEventActivity {
    async fn execute(&self, context: &mut ExecutionContext) -> Result<ActivityResult, ActivityError> {
        // End events complete the process
        context.state = ProcessInstanceState::Completed;
        Ok(ActivityResult::Completed { output_variables: None })
    }

    fn id(&self) -> &str {
        &self.event.base.id
    }

    fn name(&self) -> Option<&str> {
        self.event.base.name.as_deref()
    }
}

/// Intermediate Catch Event Activity
///
/// Implements Activity trait for IntermediateCatchEvent elements.
pub struct IntermediateCatchEventActivity {
    event: IntermediateCatchEvent,
}

impl IntermediateCatchEventActivity {
    pub fn new(event: IntermediateCatchEvent) -> Self {
        Self { event }
    }
}

#[async_trait]
impl Activity for IntermediateCatchEventActivity {
    async fn execute(&self, _context: &mut ExecutionContext) -> Result<ActivityResult, ActivityError> {
        // Intermediate catch events wait for the event to occur
        // TODO: Implement event waiting based on event definition
        Ok(ActivityResult::Waiting {
            reason: format!(
                "Intermediate catch event '{}' waiting for event",
                self.event.base.id
            ),
        })
    }

    fn id(&self) -> &str {
        &self.event.base.id
    }

    fn name(&self) -> Option<&str> {
        self.event.base.name.as_deref()
    }
}

/// Intermediate Throw Event Activity
///
/// Implements Activity trait for IntermediateThrowEvent elements.
pub struct IntermediateThrowEventActivity {
    event: IntermediateThrowEvent,
}

impl IntermediateThrowEventActivity {
    pub fn new(event: IntermediateThrowEvent) -> Self {
        Self { event }
    }
}

#[async_trait]
impl Activity for IntermediateThrowEventActivity {
    async fn execute(&self, context: &mut ExecutionContext) -> Result<ActivityResult, ActivityError> {
        // Intermediate throw events immediately throw the event and continue
        // TODO: Implement event throwing based on event definition
        let definition = &context.process_definition;
        let outgoing_flows = definition.get_outgoing_flows(&self.event.base.id);
        let next_elements: Vec<String> = outgoing_flows
            .iter()
            .map(|flow| flow.target_ref.clone())
            .collect();

        Ok(ActivityResult::Continue { next_elements })
    }

    fn id(&self) -> &str {
        &self.event.base.id
    }

    fn name(&self) -> Option<&str> {
        self.event.base.name.as_deref()
    }
}

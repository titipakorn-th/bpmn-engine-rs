//! Process Executor
//!
//! Core execution engine for BPMN processes.

use crate::activity::{Activity, ActivityError, ActivityFactory, ActivityResult, DefaultActivityFactory};
use crate::engine::context::ProcessInstanceState;
use crate::engine::instance::ProcessInstance;
use crate::model::ProcessDefinition;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Engine
///
/// Main BPMN execution engine.
#[derive(Debug)]
pub struct Engine {
    /// Process instances (in-memory storage)
    instances: Arc<RwLock<HashMap<String, Arc<ProcessInstance>>>>,
    /// Activity factory
    activity_factory: Arc<dyn ActivityFactory>,
}

impl Engine {
    /// Create a new engine
    pub fn new() -> Self {
        Self {
            instances: Arc::new(RwLock::new(HashMap::new())),
            activity_factory: Arc::new(DefaultActivityFactory::new()),
        }
    }

    /// Create a new engine with custom activity factory
    pub fn with_activity_factory(factory: Arc<dyn ActivityFactory>) -> Self {
        Self {
            instances: Arc::new(RwLock::new(HashMap::new())),
            activity_factory: factory,
        }
    }

    /// Start a new process instance
    ///
    /// # Arguments
    /// * `definition` - Process definition to execute
    /// * `initial_variables` - Initial process variables
    ///
    /// # Returns
    /// * `Ok(ProcessInstance)` - Created process instance
    /// * `Err(EngineError)` - Engine error
    pub async fn start_process(
        &self,
        definition: ProcessDefinition,
        initial_variables: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<Arc<ProcessInstance>, EngineError> {
        let instance_id = format!("instance_{}", uuid::Uuid::new_v4());
        let definition = Arc::new(definition);
        let instance = ProcessInstance::new(definition, instance_id.clone());

        // Set initial variables
        {
            let mut context = instance.context_mut().await;
            if let Some(vars) = initial_variables {
                for (name, value) in vars {
                    context.set_variable(name, value);
                }
            }
        }

        // Store instance
        {
            let mut instances = self.instances.write().await;
            instances.insert(instance_id.clone(), Arc::new(instance.clone()));
        }

        // Start execution
        self.execute_process(Arc::new(instance.clone())).await?;

        Ok(Arc::new(instance))
    }

    /// Execute a process instance
    ///
    /// This is the main execution loop that processes the BPMN process.
    async fn execute_process(&self, instance: Arc<ProcessInstance>) -> Result<(), EngineError> {
        let definition = instance.definition();
        
        // Find start events
        let start_events: Vec<String> = definition
            .elements
            .values()
            .filter_map(|elem| {
                match elem {
                    crate::model::ProcessElement::StartEvent(_) => Some(elem.id().to_string()),
                    _ => None,
                }
            })
            .collect();

        if start_events.is_empty() {
            return Err(EngineError::NoStartEvent);
        }

        // Set current elements to start events
        {
            let mut context = instance.context_mut().await;
            context.set_current_elements(start_events);
        }

        // Execute process loop
        loop {
            let should_continue = {
                let mut context = instance.context_mut().await;
                
                if context.state != ProcessInstanceState::Active {
                    break;
                }

                let current_elements = context.current_elements.clone();
                context.clear_current_elements();

                // Process each current element
                for element_id in current_elements {
                    // Get element from definition
                    let element = match definition.get_element(&element_id) {
                        Some(e) => e,
                        None => {
                            context.state = ProcessInstanceState::Failed;
                            return Err(EngineError::ElementNotFound(element_id));
                        }
                    };

                    // Create activity from element
                    let activity = match self.activity_factory.create_activity(element) {
                        Ok(a) => a,
                        Err(e) => {
                            context.state = ProcessInstanceState::Failed;
                            return Err(EngineError::ActivityExecutionError(e));
                        }
                    };

                    // Execute activity
                    let activity_result = match activity.execute(&mut context).await {
                        Ok(result) => result,
                        Err(e) => {
                            context.state = ProcessInstanceState::Failed;
                            return Err(EngineError::ActivityExecutionError(e));
                        }
                    };

                    // Record execution step
                    let step_result = match &activity_result {
                        ActivityResult::Completed { .. } => {
                            crate::engine::context::ExecutionStepResult::Completed
                        }
                        ActivityResult::Waiting { reason } => {
                            crate::engine::context::ExecutionStepResult::Waiting(reason.clone())
                        }
                        ActivityResult::Continue { .. } => {
                            crate::engine::context::ExecutionStepResult::Completed
                        }
                    };
                    context.add_execution_step(crate::engine::context::ExecutionStep {
                        element_id: element_id.clone(),
                        timestamp: std::time::SystemTime::now(),
                        result: step_result,
                    });

                    // Handle activity result
                    match activity_result {
                        ActivityResult::Completed { .. } => {
                            // Get outgoing flows
                            let outgoing_flows = definition.get_outgoing_flows(&element_id);
                            let mut next_elements = Vec::new();

                            for flow in outgoing_flows {
                                // Check condition if present
                                if let Some(_condition) = &flow.condition_expression {
                                    // TODO: Evaluate condition
                                    // For now, assume condition is true
                                }
                                next_elements.push(flow.target_ref.clone());
                            }

                            // Check if this is an end event
                            match element {
                                crate::model::ProcessElement::EndEvent(_) => {
                                    context.state = ProcessInstanceState::Completed;
                                    return Ok(());
                                }
                                _ => {
                                    context.current_elements.extend(next_elements);
                                }
                            }
                        }
                        ActivityResult::Waiting { .. } => {
                            // Process is waiting, pause execution
                            break;
                        }
                        ActivityResult::Continue { next_elements } => {
                            // Check if any next element is an end event
                            let mut has_end_event = false;
                            for next_id in &next_elements {
                                if let Some(next_elem) = definition.get_element(next_id) {
                                    if matches!(next_elem, crate::model::ProcessElement::EndEvent(_)) {
                                        has_end_event = true;
                                        break;
                                    }
                                }
                            }

                            if has_end_event {
                                context.state = ProcessInstanceState::Completed;
                                return Ok(());
                            }

                            context.current_elements.extend(next_elements);
                        }
                    }
                }

                !context.current_elements.is_empty()
            };

            if !should_continue {
                break;
            }
        }

        Ok(())
    }

    /// Get a process instance by ID
    pub async fn get_instance(&self, instance_id: &str) -> Option<Arc<ProcessInstance>> {
        let instances = self.instances.read().await;
        instances.get(instance_id).cloned()
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

/// Engine Builder
///
/// Builder for creating Engine instances with custom configuration.
#[derive(Debug, Default)]
pub struct EngineBuilder {
    // Future: Add configuration options
}

impl EngineBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the engine
    pub fn build(self) -> Engine {
        Engine::new()
    }
}

/// Engine Error
#[derive(Debug, thiserror::Error)]
pub enum EngineError {
    #[error("No start event found in process")]
    NoStartEvent,
    #[error("Element not found: {0}")]
    ElementNotFound(String),
    #[error("Activity execution error: {0}")]
    ActivityExecutionError(#[from] ActivityError),
    #[error("Process execution failed: {0}")]
    ExecutionFailed(String),
}


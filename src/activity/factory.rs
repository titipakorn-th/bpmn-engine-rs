//! Activity Factory
//!
//! Factory for creating Activity instances from ProcessElement.

use crate::activity::{Activity, ActivityError, ActivityFactory};
use crate::elements::event::{
    EndEventActivity, IntermediateCatchEventActivity, IntermediateThrowEventActivity,
    StartEventActivity,
};
use crate::elements::gateway::{
    ExclusiveGatewayActivity, InclusiveGatewayActivity, ParallelGatewayActivity,
};
use crate::elements::task::{
    CallActivityTask, ManualTaskActivity, ScriptTaskActivity, ServiceTaskActivity, UserTaskActivity,
};
use crate::model::ProcessElement;
use std::sync::Arc;

/// Default Activity Factory
///
/// Creates Activity instances from ProcessElement.
#[derive(Debug)]
pub struct DefaultActivityFactory;

impl DefaultActivityFactory {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DefaultActivityFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl ActivityFactory for DefaultActivityFactory {
    fn create_activity(&self, element: &ProcessElement) -> Result<Arc<dyn Activity>, ActivityError> {
        match element {
            ProcessElement::StartEvent(e) => {
                Ok(Arc::new(StartEventActivity::new(e.clone())) as Arc<dyn Activity>)
            }
            ProcessElement::EndEvent(e) => {
                Ok(Arc::new(EndEventActivity::new(e.clone())) as Arc<dyn Activity>)
            }
            ProcessElement::IntermediateCatchEvent(e) => Ok(Arc::new(IntermediateCatchEventActivity::new(
                e.clone(),
            )) as Arc<dyn Activity>),
            ProcessElement::IntermediateThrowEvent(e) => Ok(Arc::new(IntermediateThrowEventActivity::new(
                e.clone(),
            )) as Arc<dyn Activity>),
            ProcessElement::ServiceTask(e) => {
                Ok(Arc::new(ServiceTaskActivity::new(e.clone())) as Arc<dyn Activity>)
            }
            ProcessElement::UserTask(e) => {
                Ok(Arc::new(UserTaskActivity::new(e.clone())) as Arc<dyn Activity>)
            }
            ProcessElement::ScriptTask(e) => {
                Ok(Arc::new(ScriptTaskActivity::new(e.clone())) as Arc<dyn Activity>)
            }
            ProcessElement::ManualTask(e) => {
                Ok(Arc::new(ManualTaskActivity::new(e.clone())) as Arc<dyn Activity>)
            }
            ProcessElement::ExclusiveGateway(e) => {
                Ok(Arc::new(ExclusiveGatewayActivity::new(e.clone())) as Arc<dyn Activity>)
            }
            ProcessElement::ParallelGateway(e) => {
                Ok(Arc::new(ParallelGatewayActivity::new(e.clone())) as Arc<dyn Activity>)
            }
            ProcessElement::InclusiveGateway(e) => {
                Ok(Arc::new(InclusiveGatewayActivity::new(e.clone())) as Arc<dyn Activity>)
            }
            ProcessElement::DataObject(_e) => {
                // DataObjects are typically referenced, not executed directly
                // Return an error as they shouldn't be instantiated as activities
                Err(ActivityError::InvalidElement("DataObject is not executable".to_string()))
            }
            ProcessElement::DataInput(_e) => {
                Err(ActivityError::InvalidElement("DataInput is not executable".to_string()))
            }
            ProcessElement::DataOutput(_e) => {
                Err(ActivityError::InvalidElement("DataOutput is not executable".to_string()))
            }
            ProcessElement::DataObjectReference(_e) => {
                Err(ActivityError::InvalidElement("DataObjectReference is not executable".to_string()))
            }
            ProcessElement::CallActivity(e) => {
                Ok(Arc::new(CallActivityTask::new(e.clone())) as Arc<dyn Activity>)
            }
        }
    }
}


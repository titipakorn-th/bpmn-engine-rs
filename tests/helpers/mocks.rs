//! Mock Implementations
//!
//! Mock implementations of traits for testing.

use bpmn_engine::activity::{Activity, ActivityError, ActivityResult, ProcessListener, ListenerError};
use bpmn_engine::engine::ExecutionContext;
use async_trait::async_trait;
use std::sync::Arc;

/// Mock Activity for testing
#[derive(Debug)]
pub struct MockActivity {
    pub id: String,
    pub name: Option<String>,
    pub execute_result: Arc<std::sync::Mutex<Result<ActivityResult, ActivityError>>>,
}

impl MockActivity {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: None,
            execute_result: Arc::new(std::sync::Mutex::new(
                Ok(ActivityResult::Completed { output_variables: None })
            )),
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_result(mut self, result: Result<ActivityResult, ActivityError>) -> Self {
        *self.execute_result.lock().unwrap() = result;
        self
    }
}

#[async_trait]
impl Activity for MockActivity {
    async fn execute(&self, _context: &mut ExecutionContext) -> Result<ActivityResult, ActivityError> {
        self.execute_result.lock().unwrap().clone()
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

/// Mock ProcessListener for testing
#[derive(Debug, Clone)]
pub struct MockProcessListener {
    pub on_process_start_called: Arc<std::sync::Mutex<bool>>,
    pub on_process_complete_called: Arc<std::sync::Mutex<bool>>,
    pub on_process_fail_called: Arc<std::sync::Mutex<bool>>,
    pub on_activity_start_called: Arc<std::sync::Mutex<Vec<String>>>,
    pub on_activity_complete_called: Arc<std::sync::Mutex<Vec<String>>>,
    pub on_activity_fail_called: Arc<std::sync::Mutex<Vec<String>>>,
}

impl MockProcessListener {
    pub fn new() -> Self {
        Self {
            on_process_start_called: Arc::new(std::sync::Mutex::new(false)),
            on_process_complete_called: Arc::new(std::sync::Mutex::new(false)),
            on_process_fail_called: Arc::new(std::sync::Mutex::new(false)),
            on_activity_start_called: Arc::new(std::sync::Mutex::new(Vec::new())),
            on_activity_complete_called: Arc::new(std::sync::Mutex::new(Vec::new())),
            on_activity_fail_called: Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }
}

impl Default for MockProcessListener {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ProcessListener for MockProcessListener {
    async fn on_process_start(&self, _context: &ExecutionContext) -> Result<(), ListenerError> {
        *self.on_process_start_called.lock().unwrap() = true;
        Ok(())
    }

    async fn on_process_complete(&self, _context: &ExecutionContext) -> Result<(), ListenerError> {
        *self.on_process_complete_called.lock().unwrap() = true;
        Ok(())
    }

    async fn on_process_fail(&self, _context: &ExecutionContext, _error: &str) -> Result<(), ListenerError> {
        *self.on_process_fail_called.lock().unwrap() = true;
        Ok(())
    }

    async fn on_activity_start(
        &self,
        _context: &ExecutionContext,
        activity_id: &str,
    ) -> Result<(), ListenerError> {
        self.on_activity_start_called.lock().unwrap().push(activity_id.to_string());
        Ok(())
    }

    async fn on_activity_complete(
        &self,
        _context: &ExecutionContext,
        activity_id: &str,
        _result: &ActivityResult,
    ) -> Result<(), ListenerError> {
        self.on_activity_complete_called.lock().unwrap().push(activity_id.to_string());
        Ok(())
    }

    async fn on_activity_fail(
        &self,
        _context: &ExecutionContext,
        activity_id: &str,
        _error: &str,
    ) -> Result<(), ListenerError> {
        self.on_activity_fail_called.lock().unwrap().push(activity_id.to_string());
        Ok(())
    }
}

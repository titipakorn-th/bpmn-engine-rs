//! Mock Process Listener
//!
//! Mock implementation of ProcessListener for testing.

use bpmn_engine::activity::{ActivityResult, ProcessListener};
use bpmn_engine::engine::ExecutionContext;
use std::sync::{Arc, RwLock};

/// Mock Process Listener
///
/// A mock implementation of [`ProcessListener`] that records all events
/// to an internal vector for inspection in tests. Also maintains backward-compatible
/// per-event boolean/vector tracking fields.
#[derive(Debug, Clone)]
pub struct MockProcessListener {
    /// Unified event log for new-style inspection
    pub events: Arc<RwLock<Vec<String>>>,
    /// Backward-compatible: tracks on_process_start calls
    pub on_process_start_called: Arc<std::sync::Mutex<bool>>,
    /// Backward-compatible: tracks on_process_complete calls
    pub on_process_complete_called: Arc<std::sync::Mutex<bool>>,
    /// Backward-compatible: tracks on_process_fail calls
    pub on_process_fail_called: Arc<std::sync::Mutex<bool>>,
    /// Backward-compatible: tracks activity start calls
    pub on_activity_start_called: Arc<std::sync::Mutex<Vec<String>>>,
    /// Backward-compatible: tracks activity complete calls
    pub on_activity_complete_called: Arc<std::sync::Mutex<Vec<String>>>,
    /// Backward-compatible: tracks activity fail calls
    pub on_activity_fail_called: Arc<std::sync::Mutex<Vec<String>>>,
}

impl MockProcessListener {
    /// Create a new MockProcessListener
    pub fn new() -> Self {
        Self {
            events: Arc::new(RwLock::new(Vec::new())),
            on_process_start_called: Arc::new(std::sync::Mutex::new(false)),
            on_process_complete_called: Arc::new(std::sync::Mutex::new(false)),
            on_process_fail_called: Arc::new(std::sync::Mutex::new(false)),
            on_activity_start_called: Arc::new(std::sync::Mutex::new(Vec::new())),
            on_activity_complete_called: Arc::new(std::sync::Mutex::new(Vec::new())),
            on_activity_fail_called: Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }

    /// Get all recorded events
    ///
    /// Returns a clone of the events vector.
    pub fn get_events(&self) -> Vec<String> {
        self.events.read().unwrap().clone()
    }
}

impl Default for MockProcessListener {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl ProcessListener for MockProcessListener {
    async fn on_process_start(&self, _context: &ExecutionContext) -> Result<(), bpmn_engine::activity::ListenerError> {
        self.events.write().unwrap().push("process_start".to_string());
        *self.on_process_start_called.lock().unwrap() = true;
        Ok(())
    }

    async fn on_process_complete(&self, _context: &ExecutionContext) -> Result<(), bpmn_engine::activity::ListenerError> {
        self.events.write().unwrap().push("process_complete".to_string());
        *self.on_process_complete_called.lock().unwrap() = true;
        Ok(())
    }

    async fn on_process_fail(&self, _context: &ExecutionContext, _error: &str) -> Result<(), bpmn_engine::activity::ListenerError> {
        self.events.write().unwrap().push("process_fail".to_string());
        *self.on_process_fail_called.lock().unwrap() = true;
        Ok(())
    }

    async fn on_activity_start(&self, _context: &ExecutionContext, activity_id: &str) -> Result<(), bpmn_engine::activity::ListenerError> {
        self.events.write().unwrap().push(format!("activity_start:{}", activity_id));
        self.on_activity_start_called.lock().unwrap().push(activity_id.to_string());
        Ok(())
    }

    async fn on_activity_complete(&self, _context: &ExecutionContext, activity_id: &str, _result: &ActivityResult) -> Result<(), bpmn_engine::activity::ListenerError> {
        self.events.write().unwrap().push(format!("activity_complete:{}", activity_id));
        self.on_activity_complete_called.lock().unwrap().push(activity_id.to_string());
        Ok(())
    }

    async fn on_activity_fail(&self, _context: &ExecutionContext, activity_id: &str, _error: &str) -> Result<(), bpmn_engine::activity::ListenerError> {
        self.events.write().unwrap().push(format!("activity_fail:{}", activity_id));
        self.on_activity_fail_called.lock().unwrap().push(activity_id.to_string());
        Ok(())
    }
}
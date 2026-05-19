//! Unit tests for ProcessListener

use bpmn_engine::activity::{ListenerRegistry, ProcessListener, ListenerError};
use bpmn_engine::engine::ExecutionContext;
use bpmn_engine::model::ProcessDefinition;
use test_log::test;

#[tokio::test]
async fn test_listener_registry_register() {
    let registry = ListenerRegistry::new();
    
    // Create a mock listener
    let listener = std::sync::Arc::new(crate::helpers::mock_listener::MockProcessListener::new());
    
    registry.register(listener.clone()).await;
    
    let listeners = registry.get_listeners().await;
    assert_eq!(listeners.len(), 1);
}

#[tokio::test]
async fn test_listener_registry_multiple_listeners() {
    let registry = ListenerRegistry::new();
    
    let listener1 = std::sync::Arc::new(crate::helpers::mock_listener::MockProcessListener::new());
    let listener2 = std::sync::Arc::new(crate::helpers::mock_listener::MockProcessListener::new());
    
    registry.register(listener1.clone()).await;
    registry.register(listener2.clone()).await;
    
    let listeners = registry.get_listeners().await;
    assert_eq!(listeners.len(), 2);
}

#[tokio::test]
async fn test_mock_listener_on_process_start() {
    let listener = crate::helpers::mock_listener::MockProcessListener::new();
    let definition = ProcessDefinition::from_json(r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [],
        "variables": {}
    }
    "#).unwrap();
    let context = ExecutionContext::new(definition, "instance1".to_string());
    
    let result = listener.on_process_start(&context).await;
    assert!(result.is_ok());
    assert!(*listener.on_process_start_called.lock().unwrap());
}

#[tokio::test]
async fn test_mock_listener_on_process_complete() {
    let listener = crate::helpers::mock_listener::MockProcessListener::new();
    let definition = ProcessDefinition::from_json(r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [],
        "variables": {}
    }
    "#).unwrap();
    let context = ExecutionContext::new(definition, "instance1".to_string());
    
    let result = listener.on_process_complete(&context).await;
    assert!(result.is_ok());
    assert!(*listener.on_process_complete_called.lock().unwrap());
}

#[tokio::test]
async fn test_mock_listener_on_activity_start() {
    let listener = crate::helpers::mock_listener::MockProcessListener::new();
    let definition = ProcessDefinition::from_json(r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [],
        "variables": {}
    }
    "#).unwrap();
    let context = ExecutionContext::new(definition, "instance1".to_string());
    
    let result = listener.on_activity_start(&context, "activity1").await;
    assert!(result.is_ok());
    let called = listener.on_activity_start_called.lock().unwrap();
    assert_eq!(called.len(), 1);
    assert_eq!(called[0], "activity1");
}


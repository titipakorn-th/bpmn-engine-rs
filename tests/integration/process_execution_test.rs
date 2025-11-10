//! Integration tests for process execution

use bpmn_engine::{Engine, ProcessDefinition};
use test_log::test;
use crate::helpers::fixtures::*;

#[tokio::test]
async fn test_simple_process_execution() {
    let engine = Engine::new();
    let definition = ProcessDefinition::from_json(SIMPLE_PROCESS_JSON).unwrap();
    
    let instance = engine.start_process(definition, None).await.unwrap();
    
    let context = instance.context().await;
    assert_eq!(context.state, bpmn_engine::engine::context::ProcessInstanceState::Completed);
    assert!(!context.execution_history.is_empty());
}

#[tokio::test]
async fn test_process_with_initial_variables() {
    let engine = Engine::new();
    let definition = ProcessDefinition::from_json(SIMPLE_PROCESS_JSON).unwrap();
    
    let mut initial_variables = std::collections::HashMap::new();
    initial_variables.insert("test_var".to_string(), serde_json::json!("test_value"));
    
    let instance = engine.start_process(definition, Some(initial_variables)).await.unwrap();
    
    let context = instance.context().await;
    assert_eq!(context.get_variable("test_var"), Some(&serde_json::json!("test_value")));
}

#[tokio::test]
async fn test_process_no_start_event_error() {
    let engine = Engine::new();
    let definition = ProcessDefinition::from_json(INVALID_PROCESS_NO_START_JSON).unwrap();
    
    let result = engine.start_process(definition, None).await;
    assert!(result.is_err());
    
    match result.unwrap_err() {
        bpmn_engine::EngineError::NoStartEvent => {}
        _ => panic!("Expected NoStartEvent error"),
    }
}

#[tokio::test]
async fn test_get_instance() {
    let engine = Engine::new();
    let definition = ProcessDefinition::from_json(SIMPLE_PROCESS_JSON).unwrap();
    
    let instance = engine.start_process(definition, None).await.unwrap();
    let instance_id = instance.id().to_string();
    
    let retrieved = engine.get_instance(&instance_id).await;
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id(), instance_id);
}


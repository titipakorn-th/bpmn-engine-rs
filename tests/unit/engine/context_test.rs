//! Unit tests for ExecutionContext

use bpmn_engine::engine::context::{ExecutionContext, ProcessInstanceState, ExecutionStep, ExecutionStepResult};
use bpmn_engine::model::ProcessDefinition;
use test_log::test;

#[test]
fn test_execution_context_new() {
    let definition = ProcessDefinition::from_json(r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [],
        "variables": {}
    }
    "#).unwrap();
    
    let context = ExecutionContext::new(definition, "instance1".to_string());
    assert_eq!(context.instance_id, "instance1");
    assert_eq!(context.state, ProcessInstanceState::Active);
    assert!(context.current_elements.is_empty());
    assert!(context.variables.is_empty());
}

#[test]
fn test_execution_context_set_get_variable() {
    let definition = ProcessDefinition::from_json(r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [],
        "variables": {}
    }
    "#).unwrap();
    
    let mut context = ExecutionContext::new(definition, "instance1".to_string());
    context.set_variable("test_var".to_string(), serde_json::json!("test_value"));
    
    assert_eq!(context.get_variable("test_var"), Some(&serde_json::json!("test_value")));
    assert_eq!(context.get_variable("nonexistent"), None);
}

#[test]
fn test_execution_context_add_execution_step() {
    let definition = ProcessDefinition::from_json(r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [],
        "variables": {}
    }
    "#).unwrap();
    
    let mut context = ExecutionContext::new(definition, "instance1".to_string());
    context.add_execution_step(ExecutionStep {
        element_id: "task1".to_string(),
        timestamp: std::time::SystemTime::now(),
        result: ExecutionStepResult::Completed,
    });
    
    assert_eq!(context.execution_history.len(), 1);
    assert_eq!(context.execution_history[0].element_id, "task1");
}

#[test]
fn test_execution_context_set_current_elements() {
    let definition = ProcessDefinition::from_json(r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [],
        "variables": {}
    }
    "#).unwrap();
    
    let mut context = ExecutionContext::new(definition, "instance1".to_string());
    context.set_current_elements(vec!["task1".to_string(), "task2".to_string()]);
    
    assert_eq!(context.current_elements.len(), 2);
    assert_eq!(context.current_elements[0], "task1");
    
    context.clear_current_elements();
    assert!(context.current_elements.is_empty());
}


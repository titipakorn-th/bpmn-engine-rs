//! Unit tests for ExecutionContext

use bpmn_engine::engine::context::{ExecutionContext, ProcessInstanceState, ExecutionStep, ExecutionStepResult, GatewayDirection, detect_gateway_direction};
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

// Token tracking tests

#[test]
fn test_token_tracking_add_and_check() {
    let definition = ProcessDefinition::from_json(r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [],
        "variables": {}
    }
    "#).unwrap();

    let mut context = ExecutionContext::new(definition, "instance1".to_string());

    // Add tokens for a gateway
    context.add_incoming_token("gateway1".to_string(), "branch1".to_string());
    context.add_incoming_token("gateway1".to_string(), "branch2".to_string());

    let tokens = context.get_incoming_tokens("gateway1");
    assert_eq!(tokens.len(), 2);
    assert!(tokens.contains("branch1"));
    assert!(tokens.contains("branch2"));
}

#[test]
fn test_all_tokens_arrived_true() {
    let definition = ProcessDefinition::from_json(r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [],
        "variables": {}
    }
    "#).unwrap();

    let mut context = ExecutionContext::new(definition, "instance1".to_string());

    // Add 2 tokens, check for 2 required
    context.add_incoming_token("gateway1".to_string(), "branch1".to_string());
    context.add_incoming_token("gateway1".to_string(), "branch2".to_string());

    assert!(context.all_tokens_arrived("gateway1", 2));
}

#[test]
fn test_all_tokens_arrived_false() {
    let definition = ProcessDefinition::from_json(r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [],
        "variables": {}
    }
    "#).unwrap();

    let mut context = ExecutionContext::new(definition, "instance1".to_string());

    // Add 1 token, check for 2 required
    context.add_incoming_token("gateway1".to_string(), "branch1".to_string());

    assert!(!context.all_tokens_arrived("gateway1", 2));
}

#[test]
fn test_clear_incoming_tokens() {
    let definition = ProcessDefinition::from_json(r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [],
        "variables": {}
    }
    "#).unwrap();

    let mut context = ExecutionContext::new(definition, "instance1".to_string());

    context.add_incoming_token("gateway1".to_string(), "branch1".to_string());
    assert!(!context.get_incoming_tokens("gateway1").is_empty());

    context.clear_incoming_tokens("gateway1");
    assert!(context.get_incoming_tokens("gateway1").is_empty());
}

#[test]
fn test_all_tokens_arrived_no_tokens() {
    let definition = ProcessDefinition::from_json(r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [],
        "variables": {}
    }
    "#).unwrap();

    let context = ExecutionContext::new(definition, "instance1".to_string());

    // No tokens added, should return false
    assert!(!context.all_tokens_arrived("gateway1", 1));
}

// GatewayDirection tests

#[test]
fn test_gateway_direction_default() {
    let direction = GatewayDirection::default();
    assert_eq!(direction, GatewayDirection::Unknown);
}

#[test]
fn test_gateway_direction_diverging() {
    let definition = ProcessDefinition::from_json(r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [
            {
                "type": "startEvent",
                "id": "start"
            },
            {
                "type": "parallelGateway",
                "id": "gateway1"
            },
            {
                "type": "serviceTask",
                "id": "task1"
            },
            {
                "type": "serviceTask",
                "id": "task2"
            },
            {
                "type": "sequenceFlow",
                "id": "flow_start",
                "sourceRef": "start",
                "targetRef": "gateway1"
            },
            {
                "type": "sequenceFlow",
                "id": "flow1",
                "sourceRef": "gateway1",
                "targetRef": "task1"
            },
            {
                "type": "sequenceFlow",
                "id": "flow2",
                "sourceRef": "gateway1",
                "targetRef": "task2"
            }
        ],
        "variables": {}
    }
    "#).unwrap();

    let direction = detect_gateway_direction("gateway1", &definition);
    assert_eq!(direction, GatewayDirection::Diverging);
}

#[test]
fn test_gateway_direction_converging() {
    let definition = ProcessDefinition::from_json(r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [
            {
                "type": "serviceTask",
                "id": "task1"
            },
            {
                "type": "serviceTask",
                "id": "task2"
            },
            {
                "type": "parallelGateway",
                "id": "gateway1"
            },
            {
                "type": "endEvent",
                "id": "end"
            },
            {
                "type": "sequenceFlow",
                "id": "flow1",
                "sourceRef": "task1",
                "targetRef": "gateway1"
            },
            {
                "type": "sequenceFlow",
                "id": "flow2",
                "sourceRef": "task2",
                "targetRef": "gateway1"
            },
            {
                "type": "sequenceFlow",
                "id": "flow_end",
                "sourceRef": "gateway1",
                "targetRef": "end"
            }
        ],
        "variables": {}
    }
    "#).unwrap();

    let direction = detect_gateway_direction("gateway1", &definition);
    assert_eq!(direction, GatewayDirection::Converging);
}

#[test]
fn test_gateway_direction_mixed() {
    let definition = ProcessDefinition::from_json(r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [
            {
                "type": "serviceTask",
                "id": "task1"
            },
            {
                "type": "serviceTask",
                "id": "task2"
            },
            {
                "type": "parallelGateway",
                "id": "gateway1"
            },
            {
                "type": "serviceTask",
                "id": "task3"
            },
            {
                "type": "serviceTask",
                "id": "task4"
            },
            {
                "type": "sequenceFlow",
                "id": "flow1",
                "sourceRef": "task1",
                "targetRef": "gateway1"
            },
            {
                "type": "sequenceFlow",
                "id": "flow2",
                "sourceRef": "task2",
                "targetRef": "gateway1"
            },
            {
                "type": "sequenceFlow",
                "id": "flow3",
                "sourceRef": "gateway1",
                "targetRef": "task3"
            },
            {
                "type": "sequenceFlow",
                "id": "flow4",
                "sourceRef": "gateway1",
                "targetRef": "task4"
            }
        ],
        "variables": {}
    }
    "#).unwrap();

    let direction = detect_gateway_direction("gateway1", &definition);
    assert_eq!(direction, GatewayDirection::Mixed);
}

#[test]
fn test_gateway_direction_unknown() {
    let definition = ProcessDefinition::from_json(r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [
            {
                "type": "parallelGateway",
                "id": "gateway1"
            },
            {
                "type": "serviceTask",
                "id": "task1"
            },
            {
                "type": "sequenceFlow",
                "id": "flow1",
                "sourceRef": "gateway1",
                "targetRef": "task1"
            }
        ],
        "variables": {}
    }
    "#).unwrap();

    // Only 1 incoming, 1 outgoing = Unknown
    let direction = detect_gateway_direction("gateway1", &definition);
    assert_eq!(direction, GatewayDirection::Unknown);
}



//! Unit tests for BPMN JSON model

use bpmn_engine::model::json::*;
use serde_json;

#[test]
fn test_start_event_deserialize() {
    let json = r#"
    {
        "type": "startEvent",
        "id": "start1",
        "name": "Start Event"
    }
    "#;

    let element: BpmnJsonElement = serde_json::from_str(json).unwrap();
    match element {
        BpmnJsonElement::StartEvent(event) => {
            assert_eq!(event.base.id, "start1");
            assert_eq!(event.base.name, Some("Start Event".to_string()));
        }
        _ => panic!("Expected StartEvent"),
    }
}

#[test]
fn test_end_event_deserialize() {
    let json = r#"
    {
        "type": "endEvent",
        "id": "end1",
        "name": "End Event"
    }
    "#;

    let element: BpmnJsonElement = serde_json::from_str(json).unwrap();
    match element {
        BpmnJsonElement::EndEvent(event) => {
            assert_eq!(event.base.id, "end1");
            assert_eq!(event.base.name, Some("End Event".to_string()));
        }
        _ => panic!("Expected EndEvent"),
    }
}

#[test]
fn test_service_task_deserialize() {
    let json = r#"
    {
        "type": "serviceTask",
        "id": "task1",
        "name": "Service Task",
        "implementation": "webService"
    }
    "#;

    let element: BpmnJsonElement = serde_json::from_str(json).unwrap();
    match element {
        BpmnJsonElement::ServiceTask(task) => {
            assert_eq!(task.base.id, "task1");
            assert_eq!(task.base.name, Some("Service Task".to_string()));
            assert_eq!(task.implementation, Some("webService".to_string()));
        }
        _ => panic!("Expected ServiceTask"),
    }
}

#[test]
fn test_user_task_deserialize() {
    let json = r#"
    {
        "type": "userTask",
        "id": "userTask1",
        "name": "User Task",
        "formKey": "form123"
    }
    "#;

    let element: BpmnJsonElement = serde_json::from_str(json).unwrap();
    match element {
        BpmnJsonElement::UserTask(task) => {
            assert_eq!(task.base.id, "userTask1");
            assert_eq!(task.base.name, Some("User Task".to_string()));
            assert_eq!(task.form_key, Some("form123".to_string()));
        }
        _ => panic!("Expected UserTask"),
    }
}

#[test]
fn test_script_task_deserialize() {
    let json = r#"
    {
        "type": "scriptTask",
        "id": "scriptTask1",
        "name": "Script Task",
        "scriptFormat": "javascript",
        "script": "console.log('test');"
    }
    "#;

    let element: BpmnJsonElement = serde_json::from_str(json).unwrap();
    match element {
        BpmnJsonElement::ScriptTask(task) => {
            assert_eq!(task.base.id, "scriptTask1");
            assert_eq!(task.script_format, Some("javascript".to_string()));
            assert_eq!(task.script, Some("console.log('test');".to_string()));
        }
        _ => panic!("Expected ScriptTask"),
    }
}

#[test]
fn test_exclusive_gateway_deserialize() {
    let json = r#"
    {
        "type": "exclusiveGateway",
        "id": "gateway1",
        "name": "Exclusive Gateway",
        "defaultFlow": "flow2"
    }
    "#;

    let element: BpmnJsonElement = serde_json::from_str(json).unwrap();
    match element {
        BpmnJsonElement::ExclusiveGateway(gateway) => {
            assert_eq!(gateway.base.id, "gateway1");
            assert_eq!(gateway.default_flow, Some("flow2".to_string()));
        }
        _ => panic!("Expected ExclusiveGateway"),
    }
}

#[test]
fn test_parallel_gateway_deserialize() {
    let json = r#"
    {
        "type": "parallelGateway",
        "id": "gateway1",
        "name": "Parallel Gateway"
    }
    "#;

    let element: BpmnJsonElement = serde_json::from_str(json).unwrap();
    match element {
        BpmnJsonElement::ParallelGateway(gateway) => {
            assert_eq!(gateway.base.id, "gateway1");
        }
        _ => panic!("Expected ParallelGateway"),
    }
}

#[test]
fn test_sequence_flow_deserialize() {
    let json = r#"
    {
        "type": "sequenceFlow",
        "id": "flow1",
        "name": "Flow 1",
        "sourceRef": "start1",
        "targetRef": "task1",
        "conditionExpression": {
            "language": "javascript",
            "body": "true"
        }
    }
    "#;

    let element: BpmnJsonElement = serde_json::from_str(json).unwrap();
    match element {
        BpmnJsonElement::SequenceFlow(flow) => {
            assert_eq!(flow.base.id, "flow1");
            assert_eq!(flow.source_ref, "start1");
            assert_eq!(flow.target_ref, "task1");
            assert!(flow.condition_expression.is_some());
            if let Some(cond) = flow.condition_expression {
                assert_eq!(cond.language, Some("javascript".to_string()));
                assert_eq!(cond.body, "true");
            }
        }
        _ => panic!("Expected SequenceFlow"),
    }
}

#[test]
fn test_event_definition_message() {
    // Enum internal fields use snake_case
    let json = r#"
    {
        "type": "message",
        "message_ref": "msg1"
    }
    "#;

    let def: BpmnJsonEventDefinition = serde_json::from_str(json).unwrap();
    match def {
        BpmnJsonEventDefinition::Message { message_ref } => {
            assert_eq!(message_ref, Some("msg1".to_string()));
        }
        _ => panic!("Expected Message event definition, got {:?}", def),
    }
}

#[test]
fn test_event_definition_timer() {
    // Enum internal fields use snake_case
    let json = r#"
    {
        "type": "timer",
        "time_definition": "PT1H"
    }
    "#;

    let def: BpmnJsonEventDefinition = serde_json::from_str(json).unwrap();
    match def {
        BpmnJsonEventDefinition::Timer { time_definition } => {
            assert_eq!(time_definition, Some("PT1H".to_string()));
        }
        _ => panic!("Expected Timer event definition, got {:?}", def),
    }
}

#[test]
fn test_process_deserialize() {
    let json = r#"
    {
        "id": "process1",
        "name": "Test Process",
        "processType": "process",
        "isExecutable": true,
        "elements": [
            {
                "type": "startEvent",
                "id": "start1"
            }
        ],
        "variables": {}
    }
    "#;

    let process: BpmnJsonProcess = serde_json::from_str(json).unwrap();
    assert_eq!(process.id, "process1");
    assert_eq!(process.name, Some("Test Process".to_string()));
    assert_eq!(process.process_type, "process");
    assert!(process.is_executable);
    assert_eq!(process.elements.len(), 1);
}

#[test]
fn test_process_serialize() {
    let process = BpmnJsonProcess {
        id: "process1".to_string(),
        name: Some("Test Process".to_string()),
        process_type: "process".to_string(),
        is_executable: true,
        elements: vec![],
        variables: std::collections::HashMap::new(),
    };

    let json = serde_json::to_string(&process).unwrap();
    assert!(json.contains("process1"));
    assert!(json.contains("Test Process"));
}

#[test]
fn test_invalid_json_error() {
    let json = r#"
    {
        "id": "process1",
        "name": "Test Process"
        // Missing comma
        "elements": []
    }
    "#;

    let result: Result<BpmnJsonProcess, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_optional_fields() {
    let json = r#"
    {
        "type": "startEvent",
        "id": "start1"
    }
    "#;

    let element: BpmnJsonElement = serde_json::from_str(json).unwrap();
    match element {
        BpmnJsonElement::StartEvent(event) => {
            assert_eq!(event.base.id, "start1");
            assert_eq!(event.base.name, None);
            assert!(event.event_definition.is_none());
        }
        _ => panic!("Expected StartEvent"),
    }
}


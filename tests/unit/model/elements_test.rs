//! Unit tests for BPMN elements model

use bpmn_engine::model::elements::*;
use bpmn_engine::model::json::*;
use test_log::test;

#[test]
fn test_process_element_from_json_start_event() {
    let json_elem = BpmnJsonElement::StartEvent(BpmnJsonStartEvent {
        base: BpmnJsonElementBase {
            id: "start1".to_string(),
            name: Some("Start".to_string()),
            documentation: None,
        },
        event_definition: None,
    });

    let elem = ProcessElement::from_json_element(json_elem).unwrap();
    match elem {
        ProcessElement::StartEvent(e) => {
            assert_eq!(e.base.id, "start1");
            assert_eq!(e.base.name, Some("Start".to_string()));
        }
        _ => panic!("Expected StartEvent"),
    }
}

#[test]
fn test_process_element_from_json_end_event() {
    let json_elem = BpmnJsonElement::EndEvent(BpmnJsonEndEvent {
        base: BpmnJsonElementBase {
            id: "end1".to_string(),
            name: Some("End".to_string()),
            documentation: None,
        },
        event_definition: None,
    });

    let elem = ProcessElement::from_json_element(json_elem).unwrap();
    match elem {
        ProcessElement::EndEvent(e) => {
            assert_eq!(e.base.id, "end1");
        }
        _ => panic!("Expected EndEvent"),
    }
}

#[test]
fn test_process_element_from_json_service_task() {
    let json_elem = BpmnJsonElement::ServiceTask(BpmnJsonServiceTask {
        base: BpmnJsonElementBase {
            id: "task1".to_string(),
            name: Some("Task".to_string()),
            documentation: None,
        },
        implementation: Some("webService".to_string()),
        operation_ref: None,
        io_mapping: BpmnJsonIoMapping::default(),
        loop_characteristics: None,
    });

    let elem = ProcessElement::from_json_element(json_elem).unwrap();
    match elem {
        ProcessElement::ServiceTask(t) => {
            assert_eq!(t.base.id, "task1");
            assert_eq!(t.implementation, Some("webService".to_string()));
        }
        _ => panic!("Expected ServiceTask"),
    }
}

#[test]
fn test_process_element_id() {
    let json_elem = BpmnJsonElement::StartEvent(BpmnJsonStartEvent {
        base: BpmnJsonElementBase {
            id: "start1".to_string(),
            name: None,
            documentation: None,
        },
        event_definition: None,
    });

    let elem = ProcessElement::from_json_element(json_elem).unwrap();
    assert_eq!(elem.id(), "start1");
}

#[test]
fn test_process_definition_from_json() {
    let json = r#"
    {
        "id": "process1",
        "name": "Test Process",
        "isExecutable": true,
        "elements": [
            {
                "type": "startEvent",
                "id": "start1"
            },
            {
                "type": "endEvent",
                "id": "end1"
            },
            {
                "type": "sequenceFlow",
                "id": "flow1",
                "sourceRef": "start1",
                "targetRef": "end1"
            }
        ],
        "variables": {}
    }
    "#;

    let definition = ProcessDefinition::from_json(json).unwrap();
    assert_eq!(definition.id, "process1");
    assert_eq!(definition.name, Some("Test Process".to_string()));
    assert_eq!(definition.elements.len(), 2); // Start and End events
    assert_eq!(definition.flows.len(), 1); // One sequence flow
}

#[test]
fn test_process_definition_get_element() {
    let json = r#"
    {
        "id": "process1",
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

    let definition = ProcessDefinition::from_json(json).unwrap();
    let element = definition.get_element("start1");
    assert!(element.is_some());
    assert_eq!(element.unwrap().id(), "start1");
}

#[test]
fn test_process_definition_get_flow() {
    let json = r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [
            {
                "type": "startEvent",
                "id": "start1"
            },
            {
                "type": "endEvent",
                "id": "end1"
            },
            {
                "type": "sequenceFlow",
                "id": "flow1",
                "sourceRef": "start1",
                "targetRef": "end1"
            }
        ],
        "variables": {}
    }
    "#;

    let definition = ProcessDefinition::from_json(json).unwrap();
    let flow = definition.get_flow("flow1");
    assert!(flow.is_some());
    assert_eq!(flow.unwrap().id, "flow1");
    assert_eq!(flow.unwrap().source_ref, "start1");
    assert_eq!(flow.unwrap().target_ref, "end1");
}

#[test]
fn test_process_definition_get_outgoing_flows() {
    let json = r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [
            {
                "type": "startEvent",
                "id": "start1"
            },
            {
                "type": "endEvent",
                "id": "end1"
            },
            {
                "type": "sequenceFlow",
                "id": "flow1",
                "sourceRef": "start1",
                "targetRef": "end1"
            }
        ],
        "variables": {}
    }
    "#;

    let definition = ProcessDefinition::from_json(json).unwrap();
    let flows = definition.get_outgoing_flows("start1");
    assert_eq!(flows.len(), 1);
    assert_eq!(flows[0].id, "flow1");
}

#[test]
fn test_process_definition_get_incoming_flows() {
    let json = r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [
            {
                "type": "startEvent",
                "id": "start1"
            },
            {
                "type": "endEvent",
                "id": "end1"
            },
            {
                "type": "sequenceFlow",
                "id": "flow1",
                "sourceRef": "start1",
                "targetRef": "end1"
            }
        ],
        "variables": {}
    }
    "#;

    let definition = ProcessDefinition::from_json(json).unwrap();
    let flows = definition.get_incoming_flows("end1");
    assert_eq!(flows.len(), 1);
    assert_eq!(flows[0].id, "flow1");
}

#[test]
fn test_process_definition_from_json_error_sequence_flow() {
    let json_elem = BpmnJsonElement::SequenceFlow(BpmnJsonSequenceFlow {
        base: BpmnJsonElementBase {
            id: "flow1".to_string(),
            name: None,
            documentation: None,
        },
        source_ref: "start1".to_string(),
        target_ref: "end1".to_string(),
        condition_expression: None,
    });

    let result = ProcessElement::from_json_element(json_elem);
    assert!(result.is_err());
}

#[test]
fn test_event_definition_from_json() {
    let json_def = BpmnJsonEventDefinition::Message {
        message_ref: Some("msg1".to_string()),
    };

    let def = EventDefinition::from_json(json_def);
    match def {
        EventDefinition::Message { message_ref } => {
            assert_eq!(message_ref, Some("msg1".to_string()));
        }
        _ => panic!("Expected Message event definition"),
    }
}

#[test]
fn test_condition_expression_from_json() {
    let json_cond = BpmnJsonConditionExpression {
        language: Some("javascript".to_string()),
        body: "true".to_string(),
    };

    let cond = ConditionExpression::from_json(json_cond);
    assert_eq!(cond.language, Some("javascript".to_string()));
    assert_eq!(cond.body, "true");
}

#[test]
fn test_variable_from_json() {
    let json_var = BpmnJsonVariable {
        name: "var1".to_string(),
        variable_type: Some("string".to_string()),
        default_value: Some(serde_json::json!("test")),
    };

    let var = Variable::from_json(json_var);
    assert_eq!(var.name, "var1");
    assert_eq!(var.variable_type, Some("string".to_string()));
    assert_eq!(var.default_value, Some(serde_json::json!("test")));
}


//! Escalation event parsing and execution unit tests

use bpmn_engine::activity::{DefaultActivityFactory, ActivityFactory};
use bpmn_engine::engine::ExecutionContext;
use bpmn_engine::model::elements::*;
use bpmn_engine::model::ProcessDefinition;
use bpmn_engine::model::ProcessElement;
use test_log::test;
use tokio_test;

#[test]
fn test_parse_escalation_throw_event() {
    let xml = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL">
            <process id="Process_1" isExecutable="true">
                <intermediateThrowEvent id="escalate1" name="Escalate">
                    <escalationEventDefinition escalationRef="pmo_intervention"/>
                </intermediateThrowEvent>
            </process>
        </definitions>
    "#;

    let result = ProcessDefinition::from_xml(xml);
    assert!(result.is_ok(), "Failed to parse XML: {:?}", result.err());

    let process = result.unwrap();
    let element = process.elements.get("escalate1");

    match element {
        Some(ProcessElement::IntermediateThrowEvent(event)) => {
            assert_eq!(event.base.name, Some("Escalate".to_string()));

            match &event.event_definition {
                Some(EventDefinition::Escalation { escalation_ref }) => {
                    assert_eq!(escalation_ref.as_deref(), Some("pmo_intervention"));
                }
                _ => panic!("Expected Escalation event definition"),
            }
        }
        _ => panic!("Expected IntermediateThrowEvent"),
    }
}

#[test]
fn test_parse_escalation_catch_event() {
    let xml = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL">
            <process id="Process_1" isExecutable="true">
                <intermediateCatchEvent id="escalate_catch" name="Catch Escalation">
                    <escalationEventDefinition escalationRef="pmo_intervention"/>
                </intermediateCatchEvent>
            </process>
        </definitions>
    "#;

    let result = ProcessDefinition::from_xml(xml);
    assert!(result.is_ok(), "Failed to parse XML: {:?}", result.err());

    let process = result.unwrap();
    let element = process.elements.get("escalate_catch");

    match element {
        Some(ProcessElement::IntermediateCatchEvent(event)) => {
            assert_eq!(event.base.name, Some("Catch Escalation".to_string()));

            match &event.event_definition {
                Some(EventDefinition::Escalation { escalation_ref }) => {
                    assert_eq!(escalation_ref.as_deref(), Some("pmo_intervention"));
                }
                _ => panic!("Expected Escalation event definition"),
            }
        }
        _ => panic!("Expected IntermediateCatchEvent"),
    }
}

#[test]
fn test_parse_escalation_event_with_bpmn2_namespace() {
    let xml = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL">
            <process id="Process_1" isExecutable="true">
                <bpmn2:intermediateThrowEvent id="esc1" name="Escalation Event">
                    <bpmn2:escalationEventDefinition escalationRef="esc_signal"/>
                </bpmn2:intermediateThrowEvent>
            </process>
        </definitions>
    "#;

    let result = ProcessDefinition::from_xml(xml);
    assert!(result.is_ok(), "Failed to parse XML with bpmn2 namespace: {:?}", result.err());

    let process = result.unwrap();
    let element = process.elements.get("esc1");

    match element {
        Some(ProcessElement::IntermediateThrowEvent(event)) => {
            assert_eq!(event.base.name, Some("Escalation Event".to_string()));

            match &event.event_definition {
                Some(EventDefinition::Escalation { escalation_ref }) => {
                    assert_eq!(escalation_ref.as_deref(), Some("esc_signal"));
                }
                _ => panic!("Expected Escalation event definition"),
            }
        }
        _ => panic!("Expected IntermediateThrowEvent"),
    }
}

#[test]
fn test_parse_escalation_event_without_escalation_ref() {
    let xml = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL">
            <process id="Process_1" isExecutable="true">
                <intermediateThrowEvent id="esc1">
                    <escalationEventDefinition/>
                </intermediateThrowEvent>
            </process>
        </definitions>
    "#;

    let result = ProcessDefinition::from_xml(xml);
    assert!(result.is_ok(), "Failed to parse XML: {:?}", result.err());

    let process = result.unwrap();
    let element = process.elements.get("esc1");

    match element {
        Some(ProcessElement::IntermediateThrowEvent(event)) => {
            match &event.event_definition {
                Some(EventDefinition::Escalation { escalation_ref }) => {
                    assert!(escalation_ref.is_none());
                }
                _ => panic!("Expected Escalation event definition"),
            }
        }
        _ => panic!("Expected IntermediateThrowEvent"),
    }
}

#[tokio::test]
async fn test_escalation_catch_event_activity() {
    let factory = DefaultActivityFactory::new();
    let event = ProcessElement::IntermediateCatchEvent(IntermediateCatchEvent {
        base: ElementBase {
            id: "catchEscalation1".to_string(),
            name: Some("Catch Escalation Event".to_string()),
            documentation: None,
            extension_elements: None,
        },
        event_definition: Some(EventDefinition::Escalation {
            escalation_ref: Some("pmo_intervention".to_string()),
        }),
    });

    let activity = factory.create_activity(&event).unwrap();
    let mut definition = ProcessDefinition::from_json(r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [],
        "variables": {}
    }
    "#).unwrap();
    let mut context = ExecutionContext::new(definition.clone(), "instance1".to_string());

    let result = activity.execute(&mut context).await.unwrap();
    match result {
        bpmn_engine::activity::ActivityResult::Waiting { reason } => {
            assert!(reason.contains("Escalation"));
            // Check that escalation info was stored in context
            let escalation_key = "_escalation_catchEscalation1";
            assert!(context.variables.contains_key(escalation_key));
        }
        _ => panic!("Expected Waiting result for escalation catch event"),
    }
}

#[tokio::test]
async fn test_escalation_throw_event_activity() {
    let factory = DefaultActivityFactory::new();
    let event = ProcessElement::IntermediateThrowEvent(IntermediateThrowEvent {
        base: ElementBase {
            id: "throwEscalation1".to_string(),
            name: Some("Throw Escalation Event".to_string()),
            documentation: None,
            extension_elements: None,
        },
        event_definition: Some(EventDefinition::Escalation {
            escalation_ref: Some("pmo_intervention".to_string()),
        }),
    });

    let activity = factory.create_activity(&event).unwrap();
    let mut definition = ProcessDefinition::from_json(r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [
            {
                "type": "serviceTask",
                "id": "task1"
            },
            {
                "type": "sequenceFlow",
                "id": "flow1",
                "sourceRef": "throwEscalation1",
                "targetRef": "task1"
            }
        ],
        "variables": {}
    }
    "#).unwrap();
    let mut context = ExecutionContext::new(definition.clone(), "instance1".to_string());

    let result = activity.execute(&mut context).await.unwrap();
    match result {
        bpmn_engine::activity::ActivityResult::Continue { next_elements } => {
            // Check that escalation was stored in context
            let escalation_key = "_escalation_throwEscalation1";
            assert!(context.variables.contains_key(escalation_key));
        }
        _ => panic!("Expected Continue result for escalation throw event"),
    }
}

#[tokio::test]
async fn test_escalation_throw_and_catch_flow() {
    // Test a complete flow with escalation throw and catch
    let xml = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL">
            <process id="Process_1" isExecutable="true">
                <startEvent id="start1"/>
                <intermediateThrowEvent id="throw_esc">
                    <escalationEventDefinition escalationRef="pmo_intervention"/>
                </intermediateThrowEvent>
                <intermediateCatchEvent id="catch_esc">
                    <escalationEventDefinition escalationRef="pmo_intervention"/>
                </intermediateCatchEvent>
                <endEvent id="end1"/>
                <sequenceFlow id="flow1" sourceRef="start1" targetRef="throw_esc"/>
                <sequenceFlow id="flow2" sourceRef="throw_esc" targetRef="catch_esc"/>
                <sequenceFlow id="flow3" sourceRef="catch_esc" targetRef="end1"/>
            </process>
        </definitions>
    "#;

    let result = ProcessDefinition::from_xml(xml);
    assert!(result.is_ok(), "Failed to parse XML: {:?}", result.err());

    let process = result.unwrap();

    // Verify throw event
    let throw_element = process.elements.get("throw_esc");
    match throw_element {
        Some(ProcessElement::IntermediateThrowEvent(event)) => {
            match &event.event_definition {
                Some(EventDefinition::Escalation { escalation_ref }) => {
                    assert_eq!(escalation_ref.as_deref(), Some("pmo_intervention"));
                }
                _ => panic!("Expected Escalation event definition for throw event"),
            }
        }
        _ => panic!("Expected IntermediateThrowEvent for throw_esc"),
    }

    // Verify catch event
    let catch_element = process.elements.get("catch_esc");
    match catch_element {
        Some(ProcessElement::IntermediateCatchEvent(event)) => {
            match &event.event_definition {
                Some(EventDefinition::Escalation { escalation_ref }) => {
                    assert_eq!(escalation_ref.as_deref(), Some("pmo_intervention"));
                }
                _ => panic!("Expected Escalation event definition for catch event"),
            }
        }
        _ => panic!("Expected IntermediateCatchEvent for catch_esc"),
    }
}
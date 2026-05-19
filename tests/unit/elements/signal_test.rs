//! Signal event parsing and execution unit tests

use bpmn_engine::activity::{DefaultActivityFactory, ActivityFactory};
use bpmn_engine::engine::ExecutionContext;
use bpmn_engine::model::elements::*;
use bpmn_engine::model::ProcessDefinition;
use bpmn_engine::model::ProcessElement;

#[test]
fn test_signal_definition_clone() {
    let signal = SignalDefinition {
        name: Some("Approval Signal".to_string()),
        structure_ref: Some("approvalStructure".to_string()),
    };
    let cloned = signal.clone();
    assert_eq!(signal.name, cloned.name);
    assert_eq!(signal.structure_ref, cloned.structure_ref);
}

#[test]
fn test_signal_definition_debug() {
    let signal = SignalDefinition {
        name: Some("Test Signal".to_string()),
        structure_ref: None,
    };
    let debug_str = format!("{:?}", signal);
    assert!(debug_str.contains("Test Signal"));
}

#[test]
fn test_parse_signal_throw_event() {
    let xml = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL">
            <process id="Process_1" isExecutable="true">
                <intermediateThrowEvent id="signal1" name="Throw Signal">
                    <signalEventDefinition signalRef="approvalSignal"/>
                </intermediateThrowEvent>
            </process>
        </definitions>
    "#;

    let result = ProcessDefinition::from_xml(xml);
    assert!(result.is_ok(), "Failed to parse XML: {:?}", result.err());

    let process = result.unwrap();
    let element = process.elements.get("signal1");

    match element {
        Some(ProcessElement::IntermediateThrowEvent(event)) => {
            assert_eq!(event.base.name, Some("Throw Signal".to_string()));

            match &event.event_definition {
                Some(EventDefinition::Signal { signal_ref }) => {
                    assert_eq!(signal_ref.as_deref(), Some("approvalSignal"));
                }
                _ => panic!("Expected Signal event definition"),
            }
        }
        _ => panic!("Expected IntermediateThrowEvent"),
    }
}

#[test]
fn test_parse_signal_catch_event() {
    let xml = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL">
            <process id="Process_1" isExecutable="true">
                <intermediateCatchEvent id="catch1" name="Catch Signal">
                    <signalEventDefinition signalRef="approvalSignal"/>
                </intermediateCatchEvent>
            </process>
        </definitions>
    "#;

    let result = ProcessDefinition::from_xml(xml);
    assert!(result.is_ok(), "Failed to parse XML: {:?}", result.err());

    let process = result.unwrap();
    let element = process.elements.get("catch1");

    match element {
        Some(ProcessElement::IntermediateCatchEvent(event)) => {
            assert_eq!(event.base.name, Some("Catch Signal".to_string()));

            match &event.event_definition {
                Some(EventDefinition::Signal { signal_ref }) => {
                    assert_eq!(signal_ref.as_deref(), Some("approvalSignal"));
                }
                _ => panic!("Expected Signal event definition"),
            }
        }
        _ => panic!("Expected IntermediateCatchEvent"),
    }
}

#[test]
fn test_parse_signal_event_with_bpmn2_namespace() {
    let xml = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL">
            <process id="Process_1" isExecutable="true">
                <bpmn2:intermediateThrowEvent id="signal1" name="Signal Event">
                    <bpmn2:signalEventDefinition signalRef="testSignal"/>
                </bpmn2:intermediateThrowEvent>
            </process>
        </definitions>
    "#;

    let result = ProcessDefinition::from_xml(xml);
    assert!(result.is_ok(), "Failed to parse XML with bpmn2 namespace: {:?}", result.err());

    let process = result.unwrap();
    let element = process.elements.get("signal1");

    match element {
        Some(ProcessElement::IntermediateThrowEvent(event)) => {
            assert_eq!(event.base.name, Some("Signal Event".to_string()));

            match &event.event_definition {
                Some(EventDefinition::Signal { signal_ref }) => {
                    assert_eq!(signal_ref.as_deref(), Some("testSignal"));
                }
                _ => panic!("Expected Signal event definition"),
            }
        }
        _ => panic!("Expected IntermediateThrowEvent"),
    }
}

#[test]
fn test_parse_signal_event_without_signal_ref() {
    let xml = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL">
            <process id="Process_1" isExecutable="true">
                <intermediateThrowEvent id="signal1">
                    <signalEventDefinition/>
                </intermediateThrowEvent>
            </process>
        </definitions>
    "#;

    let result = ProcessDefinition::from_xml(xml);
    assert!(result.is_ok(), "Failed to parse XML: {:?}", result.err());

    let process = result.unwrap();
    let element = process.elements.get("signal1");

    match element {
        Some(ProcessElement::IntermediateThrowEvent(event)) => {
            match &event.event_definition {
                Some(EventDefinition::Signal { signal_ref }) => {
                    assert!(signal_ref.is_none());
                }
                _ => panic!("Expected Signal event definition"),
            }
        }
        _ => panic!("Expected IntermediateThrowEvent"),
    }
}

#[tokio::test]
async fn test_signal_catch_event_activity() {
    let factory = DefaultActivityFactory::new();
    let event = ProcessElement::IntermediateCatchEvent(IntermediateCatchEvent {
        base: ElementBase {
            id: "catchSignal1".to_string(),
            name: Some("Catch Signal Event".to_string()),
            documentation: None,
        },
        event_definition: Some(EventDefinition::Signal {
            signal_ref: Some("approvalSignal".to_string()),
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
            assert!(reason.contains("approvalSignal"));
            // Check that signal info was stored in context
            let signal_key = "_signal_catchSignal1";
            assert!(context.variables.contains_key(signal_key));
        }
        _ => panic!("Expected Waiting result for signal catch event"),
    }
}

#[tokio::test]
async fn test_signal_throw_event_activity() {
    let factory = DefaultActivityFactory::new();
    let event = ProcessElement::IntermediateThrowEvent(IntermediateThrowEvent {
        base: ElementBase {
            id: "throwSignal1".to_string(),
            name: Some("Throw Signal Event".to_string()),
            documentation: None,
        },
        event_definition: Some(EventDefinition::Signal {
            signal_ref: Some("approvalSignal".to_string()),
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
        bpmn_engine::activity::ActivityResult::Continue { next_elements } => {
            // Check that signal was stored in context
            let signal_key = "_signal_throwSignal1";
            assert!(context.variables.contains_key(signal_key));
        }
        _ => panic!("Expected Continue result for signal throw event"),
    }
}

#[tokio::test]
async fn test_signal_throw_and_catch_flow() {
    // Test a complete flow with signal throw and catch
    let xml = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL">
            <process id="Process_1" isExecutable="true">
                <startEvent id="start1"/>
                <intermediateThrowEvent id="throw1">
                    <signalEventDefinition signalRef="approvalSignal"/>
                </intermediateThrowEvent>
                <intermediateCatchEvent id="catch1">
                    <signalEventDefinition signalRef="approvalSignal"/>
                </intermediateCatchEvent>
                <endEvent id="end1"/>
                <sequenceFlow id="flow1" sourceRef="start1" targetRef="throw1"/>
                <sequenceFlow id="flow2" sourceRef="throw1" targetRef="catch1"/>
                <sequenceFlow id="flow3" sourceRef="catch1" targetRef="end1"/>
            </process>
        </definitions>
    "#;

    let result = ProcessDefinition::from_xml(xml);
    assert!(result.is_ok(), "Failed to parse XML: {:?}", result.err());

    let process = result.unwrap();

    // Verify throw event
    let throw_element = process.elements.get("throw1");
    match throw_element {
        Some(ProcessElement::IntermediateThrowEvent(event)) => {
            match &event.event_definition {
                Some(EventDefinition::Signal { signal_ref }) => {
                    assert_eq!(signal_ref.as_deref(), Some("approvalSignal"));
                }
                _ => panic!("Expected Signal event definition for throw event"),
            }
        }
        _ => panic!("Expected IntermediateThrowEvent for throw1"),
    }

    // Verify catch event
    let catch_element = process.elements.get("catch1");
    match catch_element {
        Some(ProcessElement::IntermediateCatchEvent(event)) => {
            match &event.event_definition {
                Some(EventDefinition::Signal { signal_ref }) => {
                    assert_eq!(signal_ref.as_deref(), Some("approvalSignal"));
                }
                _ => panic!("Expected Signal event definition for catch event"),
            }
        }
        _ => panic!("Expected IntermediateCatchEvent for catch1"),
    }
}
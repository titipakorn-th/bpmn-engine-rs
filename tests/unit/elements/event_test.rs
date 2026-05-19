//! Unit tests for Event elements

use bpmn_engine::activity::{DefaultActivityFactory, ActivityFactory};
use bpmn_engine::engine::{ExecutionContext, ProcessInstanceState};
use bpmn_engine::model::{ProcessDefinition, ProcessElement, ElementBase, StartEvent, EndEvent, IntermediateCatchEvent, IntermediateThrowEvent};
use test_log::test;
use tokio_test;

#[tokio::test]
async fn test_start_event_activity_via_factory() {
    let factory = DefaultActivityFactory::new();
    let event = ProcessElement::StartEvent(StartEvent {
        base: ElementBase {
            id: "start1".to_string(),
            name: Some("Start".to_string()),
            documentation: None,
            extension_elements: None,
        },
        event_definition: None,
    });

    let activity = factory.create_activity(&event).unwrap();
    let mut definition = ProcessDefinition::from_json(r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [
            {
                "type": "startEvent",
                "id": "start1"
            },
            {
                "type": "serviceTask",
                "id": "task1"
            },
            {
                "type": "sequenceFlow",
                "id": "flow1",
                "sourceRef": "start1",
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
            assert_eq!(next_elements.len(), 1);
            assert_eq!(next_elements[0], "task1");
        }
        _ => panic!("Expected Continue result"),
    }
}

#[tokio::test]
async fn test_end_event_activity_via_factory() {
    let factory = DefaultActivityFactory::new();
    let event = ProcessElement::EndEvent(EndEvent {
        base: ElementBase {
            id: "end1".to_string(),
            name: Some("End".to_string()),
            documentation: None,
            extension_elements: None,
        },
        event_definition: None,
    });

    let activity = factory.create_activity(&event).unwrap();
    let mut definition = ProcessDefinition::from_json(r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [
            {
                "type": "endEvent",
                "id": "end1"
            }
        ],
        "variables": {}
    }
    "#).unwrap();
    let mut context = ExecutionContext::new(definition.clone(), "instance1".to_string());

    let result = activity.execute(&mut context).await.unwrap();
    match result {
        bpmn_engine::activity::ActivityResult::Completed { .. } => {}
        _ => panic!("Expected Completed result"),
    }
    assert_eq!(context.state, ProcessInstanceState::Completed);
}

#[tokio::test]
async fn test_intermediate_catch_event_activity_via_factory() {
    let factory = DefaultActivityFactory::new();
    let event = ProcessElement::IntermediateCatchEvent(IntermediateCatchEvent {
        base: ElementBase {
            id: "catchEvent1".to_string(),
            name: Some("Catch Event".to_string()),
            documentation: None,
            extension_elements: None,
        },
        event_definition: None,
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
        bpmn_engine::activity::ActivityResult::Waiting { .. } => {}
        _ => panic!("Expected Waiting result"),
    }
}

#[tokio::test]
async fn test_intermediate_throw_event_activity_via_factory() {
    let factory = DefaultActivityFactory::new();
    let event = ProcessElement::IntermediateThrowEvent(IntermediateThrowEvent {
        base: ElementBase {
            id: "throwEvent1".to_string(),
            name: Some("Throw Event".to_string()),
            documentation: None,
            extension_elements: None,
        },
        event_definition: None,
    });

    let activity = factory.create_activity(&event).unwrap();
    let mut definition = ProcessDefinition::from_json(r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [
            {
                "type": "intermediateThrowEvent",
                "id": "throwEvent1"
            },
            {
                "type": "serviceTask",
                "id": "task1"
            },
            {
                "type": "sequenceFlow",
                "id": "flow1",
                "sourceRef": "throwEvent1",
                "targetRef": "task1"
            }
        ],
        "variables": {}
    }
    "#).unwrap();
    let mut context = ExecutionContext::new(definition.clone(), "instance1".to_string());

    let result = activity.execute(&mut context).await.unwrap();
    match result {
        bpmn_engine::activity::ActivityResult::Continue { .. } => {}
        _ => panic!("Expected Continue result"),
    }
}

//! Unit tests for Gateway elements

use bpmn_engine::activity::{DefaultActivityFactory, ActivityFactory};
use bpmn_engine::engine::{ExecutionContext, GatewayDirection};
use bpmn_engine::model::{ProcessDefinition, ProcessElement, ElementBase, ExclusiveGateway, ParallelGateway, InclusiveGateway};
use test_log::test;
use tokio_test;

#[tokio::test]
async fn test_exclusive_gateway_activity_via_factory() {
    let factory = DefaultActivityFactory::new();
    let gateway = ProcessElement::ExclusiveGateway(ExclusiveGateway {
        base: ElementBase {
            id: "gateway1".to_string(),
            name: Some("Exclusive Gateway".to_string()),
            documentation: None,
            extension_elements: None,
        },
        default_flow: None,
    });

    let activity = factory.create_activity(&gateway).unwrap();
    let mut definition = ProcessDefinition::from_json(r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [
            {
                "type": "exclusiveGateway",
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
    let mut context = ExecutionContext::new(definition.clone(), "instance1".to_string());

    let result = activity.execute(&mut context).await.unwrap();
    match result {
        bpmn_engine::activity::ActivityResult::Continue { next_elements } => {
            assert!(!next_elements.is_empty());
        }
        _ => panic!("Expected Continue result"),
    }
}

#[tokio::test]
async fn test_parallel_gateway_activity_via_factory() {
    let factory = DefaultActivityFactory::new();
    let gateway = ProcessElement::ParallelGateway(ParallelGateway {
        base: ElementBase {
            id: "gateway1".to_string(),
            name: Some("Parallel Gateway".to_string()),
            documentation: None,
            extension_elements: None,
        },
        default_flow: None,
        gateway_direction: GatewayDirection::Unknown,
    });

    let activity = factory.create_activity(&gateway).unwrap();
    let mut definition = ProcessDefinition::from_json(r#"
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
                "type": "serviceTask",
                "id": "task2"
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
    let mut context = ExecutionContext::new(definition.clone(), "instance1".to_string());

    let result = activity.execute(&mut context).await.unwrap();
    match result {
        bpmn_engine::activity::ActivityResult::Continue { next_elements } => {
            assert_eq!(next_elements.len(), 2);
        }
        _ => panic!("Expected Continue result with multiple elements"),
    }
}

#[tokio::test]
async fn test_inclusive_gateway_activity_via_factory() {
    let factory = DefaultActivityFactory::new();
    let gateway = ProcessElement::InclusiveGateway(InclusiveGateway {
        base: ElementBase {
            id: "gateway1".to_string(),
            name: Some("Inclusive Gateway".to_string()),
            documentation: None,
            extension_elements: None,
        },
        default_flow: None,
    });

    let activity = factory.create_activity(&gateway).unwrap();
    let mut definition = ProcessDefinition::from_json(r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [
            {
                "type": "inclusiveGateway",
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
    let mut context = ExecutionContext::new(definition.clone(), "instance1".to_string());

    let result = activity.execute(&mut context).await.unwrap();
    match result {
        bpmn_engine::activity::ActivityResult::Continue { next_elements } => {
            assert!(!next_elements.is_empty());
        }
        _ => panic!("Expected Continue result"),
    }
}

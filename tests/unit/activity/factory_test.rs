//! Unit tests for ActivityFactory

use bpmn_engine::activity::{ActivityFactory, DefaultActivityFactory};
use bpmn_engine::engine::GatewayDirection;
use bpmn_engine::model::{ProcessElement, StartEvent, EndEvent, ServiceTask, UserTask, ScriptTask, ManualTask, ExclusiveGateway, ParallelGateway, InclusiveGateway, IntermediateCatchEvent, IntermediateThrowEvent};
use bpmn_engine::model::ElementBase;
use test_log::test;

#[test]
fn test_factory_create_start_event_activity() {
    let factory = DefaultActivityFactory::new();
    let element = ProcessElement::StartEvent(StartEvent {
        base: ElementBase {
            id: "start1".to_string(),
            name: Some("Start".to_string()),
            documentation: None,
        },
        event_definition: None,
    });

    let activity = factory.create_activity(&element).unwrap();
    assert_eq!(activity.id(), "start1");
    assert_eq!(activity.name(), Some("Start"));
}

#[test]
fn test_factory_create_end_event_activity() {
    let factory = DefaultActivityFactory::new();
    let element = ProcessElement::EndEvent(EndEvent {
        base: ElementBase {
            id: "end1".to_string(),
            name: Some("End".to_string()),
            documentation: None,
        },
        event_definition: None,
    });

    let activity = factory.create_activity(&element).unwrap();
    assert_eq!(activity.id(), "end1");
}

#[test]
fn test_factory_create_service_task_activity() {
    let factory = DefaultActivityFactory::new();
    let element = ProcessElement::ServiceTask(ServiceTask {
        base: ElementBase {
            id: "task1".to_string(),
            name: Some("Task".to_string()),
            documentation: None,
        },
        implementation: Some("webService".to_string()),
        operation_ref: None,
        io_mapping: Default::default(),
        loop_characteristics: None,
    });

    let activity = factory.create_activity(&element).unwrap();
    assert_eq!(activity.id(), "task1");
}

#[test]
fn test_factory_create_user_task_activity() {
    let factory = DefaultActivityFactory::new();
    let element = ProcessElement::UserTask(UserTask {
        base: ElementBase {
            id: "userTask1".to_string(),
            name: Some("User Task".to_string()),
            documentation: None,
        },
        assignment: None,
        form_key: None,
        loop_characteristics: None,
    });

    let activity = factory.create_activity(&element).unwrap();
    assert_eq!(activity.id(), "userTask1");
}

#[test]
fn test_factory_create_script_task_activity() {
    let factory = DefaultActivityFactory::new();
    let element = ProcessElement::ScriptTask(ScriptTask {
        base: ElementBase {
            id: "scriptTask1".to_string(),
            name: Some("Script Task".to_string()),
            documentation: None,
        },
        script_format: Some("javascript".to_string()),
        script: Some("console.log('test');".to_string()),
        loop_characteristics: None,
    });

    let activity = factory.create_activity(&element).unwrap();
    assert_eq!(activity.id(), "scriptTask1");
}

#[test]
fn test_factory_create_manual_task_activity() {
    let factory = DefaultActivityFactory::new();
    let element = ProcessElement::ManualTask(ManualTask {
        base: ElementBase {
            id: "manualTask1".to_string(),
            name: Some("Manual Task".to_string()),
            documentation: None,
        },
    });

    let activity = factory.create_activity(&element).unwrap();
    assert_eq!(activity.id(), "manualTask1");
}

#[test]
fn test_factory_create_exclusive_gateway_activity() {
    let factory = DefaultActivityFactory::new();
    let element = ProcessElement::ExclusiveGateway(ExclusiveGateway {
        base: ElementBase {
            id: "gateway1".to_string(),
            name: Some("Gateway".to_string()),
            documentation: None,
        },
        default_flow: None,
    });

    let activity = factory.create_activity(&element).unwrap();
    assert_eq!(activity.id(), "gateway1");
}

#[test]
fn test_factory_create_parallel_gateway_activity() {
    let factory = DefaultActivityFactory::new();
    let element = ProcessElement::ParallelGateway(ParallelGateway {
        base: ElementBase {
            id: "gateway1".to_string(),
            name: Some("Parallel Gateway".to_string()),
            documentation: None,
        },
        default_flow: None,
        gateway_direction: GatewayDirection::Unknown,
    });

    let activity = factory.create_activity(&element).unwrap();
    assert_eq!(activity.id(), "gateway1");
}

#[test]
fn test_factory_create_inclusive_gateway_activity() {
    let factory = DefaultActivityFactory::new();
    let element = ProcessElement::InclusiveGateway(InclusiveGateway {
        base: ElementBase {
            id: "gateway1".to_string(),
            name: Some("Inclusive Gateway".to_string()),
            documentation: None,
        },
        default_flow: None,
    });

    let activity = factory.create_activity(&element).unwrap();
    assert_eq!(activity.id(), "gateway1");
}

#[test]
fn test_factory_create_intermediate_catch_event_activity() {
    let factory = DefaultActivityFactory::new();
    let element = ProcessElement::IntermediateCatchEvent(IntermediateCatchEvent {
        base: ElementBase {
            id: "catchEvent1".to_string(),
            name: Some("Catch Event".to_string()),
            documentation: None,
        },
        event_definition: None,
    });

    let activity = factory.create_activity(&element).unwrap();
    assert_eq!(activity.id(), "catchEvent1");
}

#[test]
fn test_factory_create_intermediate_throw_event_activity() {
    let factory = DefaultActivityFactory::new();
    let element = ProcessElement::IntermediateThrowEvent(IntermediateThrowEvent {
        base: ElementBase {
            id: "throwEvent1".to_string(),
            name: Some("Throw Event".to_string()),
            documentation: None,
        },
        event_definition: None,
    });

    let activity = factory.create_activity(&element).unwrap();
    assert_eq!(activity.id(), "throwEvent1");
}


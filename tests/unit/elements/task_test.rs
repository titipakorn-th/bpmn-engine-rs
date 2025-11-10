//! Unit tests for Task elements

use bpmn_engine::activity::{DefaultActivityFactory, ActivityFactory};
use bpmn_engine::engine::ExecutionContext;
use bpmn_engine::model::{ProcessDefinition, ProcessElement, ServiceTask, UserTask, ScriptTask, ManualTask, ElementBase};
use test_log::test;
use tokio_test;

#[tokio::test]
async fn test_service_task_activity_via_factory() {
    let factory = DefaultActivityFactory::new();
    let task = ProcessElement::ServiceTask(ServiceTask {
        base: ElementBase {
            id: "task1".to_string(),
            name: Some("Service Task".to_string()),
            documentation: None,
        },
        implementation: Some("webService".to_string()),
        operation_ref: None,
        io_mapping: Default::default(),
    });

    let activity = factory.create_activity(&task).unwrap();
    let definition = ProcessDefinition::from_json(r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [],
        "variables": {}
    }
    "#).unwrap();
    let mut context = ExecutionContext::new(definition, "instance1".to_string());

    let result = activity.execute(&mut context).await.unwrap();
    match result {
        bpmn_engine::activity::ActivityResult::Completed { .. } => {}
        _ => panic!("Expected Completed result"),
    }
    assert_eq!(activity.id(), "task1");
}

#[tokio::test]
async fn test_user_task_activity_via_factory() {
    let factory = DefaultActivityFactory::new();
    let task = ProcessElement::UserTask(UserTask {
        base: ElementBase {
            id: "userTask1".to_string(),
            name: Some("User Task".to_string()),
            documentation: None,
        },
        assignment: None,
        form_key: None,
    });

    let activity = factory.create_activity(&task).unwrap();
    let definition = ProcessDefinition::from_json(r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [],
        "variables": {}
    }
    "#).unwrap();
    let mut context = ExecutionContext::new(definition, "instance1".to_string());

    let result = activity.execute(&mut context).await.unwrap();
    match result {
        bpmn_engine::activity::ActivityResult::Waiting { .. } => {}
        _ => panic!("Expected Waiting result for UserTask"),
    }
}

#[tokio::test]
async fn test_manual_task_activity_via_factory() {
    let factory = DefaultActivityFactory::new();
    let task = ProcessElement::ManualTask(ManualTask {
        base: ElementBase {
            id: "manualTask1".to_string(),
            name: Some("Manual Task".to_string()),
            documentation: None,
        },
    });

    let activity = factory.create_activity(&task).unwrap();
    let definition = ProcessDefinition::from_json(r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [],
        "variables": {}
    }
    "#).unwrap();
    let mut context = ExecutionContext::new(definition, "instance1".to_string());

    let result = activity.execute(&mut context).await.unwrap();
    match result {
        bpmn_engine::activity::ActivityResult::Completed { .. } => {}
        _ => panic!("Expected Completed result"),
    }
}

#[tokio::test]
async fn test_script_task_activity_via_factory() {
    let factory = DefaultActivityFactory::new();
    let task = ProcessElement::ScriptTask(ScriptTask {
        base: ElementBase {
            id: "scriptTask1".to_string(),
            name: Some("Script Task".to_string()),
            documentation: None,
        },
        script_format: Some("javascript".to_string()),
        script: Some("console.log('test');".to_string()),
    });

    let activity = factory.create_activity(&task).unwrap();
    let definition = ProcessDefinition::from_json(r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [],
        "variables": {}
    }
    "#).unwrap();
    let mut context = ExecutionContext::new(definition, "instance1".to_string());

    let result = activity.execute(&mut context).await.unwrap();
    match result {
        bpmn_engine::activity::ActivityResult::Completed { .. } => {}
        _ => panic!("Expected Completed result"),
    }
}

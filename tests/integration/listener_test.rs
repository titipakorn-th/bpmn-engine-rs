//! Integration tests for ProcessListener

use bpmn_engine::{Engine, ProcessDefinition};
use test_log::test;
use crate::helpers::mock_listener::MockProcessListener;
use std::sync::Arc;

#[tokio::test]
async fn test_listener_receives_process_start_and_complete() {
    let engine = Engine::new();
    let listener = Arc::new(MockProcessListener::new());
    engine.add_listener(listener.clone()).await;

    let xml = r#"<?xml version="1.0"?>
    <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL">
      <process id="test" isExecutable="true">
        <startEvent id="start"/>
        <endEvent id="end"/>
        <sequenceFlow id="f1" sourceRef="start" targetRef="end"/>
      </process>
    </definitions>"#;

    let definition = ProcessDefinition::from_xml(xml).unwrap();
    let _instance = engine.start_process(definition, None).await.unwrap();

    let events = listener.get_events();
    assert!(events.contains(&"process_start".to_string()));
    assert!(events.contains(&"process_complete".to_string()));
}

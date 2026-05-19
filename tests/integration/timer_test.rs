//! Integration tests for Timer Events

use bpmn_engine::{Engine, ProcessDefinition};
use test_log::test;
use crate::helpers::mock_listener::MockProcessListener;
use std::sync::Arc;

#[tokio::test]
async fn test_timer_event_returns_waiting() {
    let engine = Engine::new();
    let listener = Arc::new(MockProcessListener::new());
    engine.add_listener(listener.clone()).await;

    let xml = r#"<?xml version="1.0"?>
    <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL">
      <process id="test_timer" isExecutable="true">
        <startEvent id="start"/>
        <intermediateCatchEvent id="timer1">
          <timerEventDefinition>
            <timeDuration>PT1H</timeDuration>
          </timerEventDefinition>
        </intermediateCatchEvent>
        <endEvent id="end"/>
        <sequenceFlow id="f1" sourceRef="start" targetRef="timer1"/>
        <sequenceFlow id="f2" sourceRef="timer1" targetRef="end"/>
      </process>
    </definitions>"#;

    let definition = ProcessDefinition::from_xml(xml).unwrap();
    let instance = engine.start_process(definition, None).await.unwrap();

    // Timer event should be waiting
    let context = instance.context().await;
    assert!(matches!(
        context.state,
        bpmn_engine::engine::context::ProcessInstanceState::Active
    ));

    // Check listener received activity_start for timer
    let events = listener.get_events();
    assert!(events.iter().any(|e| e.starts_with("activity_start:timer1")));
}

#[tokio::test]
async fn test_start_to_end_without_timer() {
    let xml = r#"<?xml version="1.0"?>
    <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL">
      <process id="test_simple" isExecutable="true">
        <startEvent id="start"/>
        <endEvent id="end"/>
        <sequenceFlow id="f1" sourceRef="start" targetRef="end"/>
      </process>
    </definitions>"#;

    let definition = ProcessDefinition::from_xml(xml).unwrap();
    let engine = Engine::new();
    let instance = engine.start_process(definition, None).await.unwrap();

    // Simple process should complete
    let context = instance.context().await;
    assert!(matches!(
        context.state,
        bpmn_engine::engine::context::ProcessInstanceState::Completed
    ));
}

#[tokio::test]
async fn test_timer_event_listener_events() {
    let engine = Engine::new();
    let listener = Arc::new(MockProcessListener::new());
    engine.add_listener(listener.clone()).await;

    let xml = r#"<?xml version="1.0"?>
    <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL">
      <process id="test_timer_listener" isExecutable="true">
        <startEvent id="start"/>
        <intermediateCatchEvent id="timer1">
          <timerEventDefinition>
            <timeDuration>PT5M</timeDuration>
          </timerEventDefinition>
        </intermediateCatchEvent>
        <endEvent id="end"/>
        <sequenceFlow id="f1" sourceRef="start" targetRef="timer1"/>
        <sequenceFlow id="f2" sourceRef="timer1" targetRef="end"/>
      </process>
    </definitions>"#;

    let definition = ProcessDefinition::from_xml(xml).unwrap();
    let instance = engine.start_process(definition, None).await.unwrap();
    let instance_id = instance.id().to_string();

    // Verify activity_start was called for timer1
    let events = listener.get_events();
    assert!(events.iter().any(|e| e.starts_with("activity_start:timer1")));

    // Verify process is still active (waiting on timer)
    let retrieved = engine.get_instance(&instance_id).await;
    assert!(retrieved.is_some());
}
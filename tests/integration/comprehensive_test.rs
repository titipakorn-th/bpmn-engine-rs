//! Comprehensive integration tests for BPMN Engine features

use bpmn_engine::{
    engine::{Engine, ProcessInstanceState},
    model::ProcessDefinition,
};

#[tokio::test]
async fn test_timer_event_execution() {
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
    let engine = Engine::new();
    let instance = engine.start_process(definition, None).await.unwrap();

    // Timer event should leave process in Active state
    let context = instance.context().await;
    assert!(matches!(context.state, ProcessInstanceState::Active));
}

#[tokio::test]
async fn test_signal_event_execution() {
    let xml = r#"<?xml version="1.0"?>
    <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL">
      <process id="test_signal" isExecutable="true">
        <startEvent id="start"/>
        <intermediateThrowEvent id="signal1">
          <signalEventDefinition signalRef="approvalSignal"/>
        </intermediateThrowEvent>
        <endEvent id="end"/>
        <sequenceFlow id="f1" sourceRef="start" targetRef="signal1"/>
        <sequenceFlow id="f2" sourceRef="signal1" targetRef="end"/>
      </process>
    </definitions>"#;

    let definition = ProcessDefinition::from_xml(xml).unwrap();
    let engine = Engine::new();
    let instance = engine.start_process(definition, None).await.unwrap();

    // Signal throw event should complete
    let context = instance.context().await;
    assert!(matches!(context.state, ProcessInstanceState::Completed));
}

#[tokio::test]
async fn test_data_object_parsing() {
    let xml = r#"<?xml version="1.0"?>
    <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL">
      <process id="test_data" isExecutable="true">
        <dataObject id="financialCase" name="Financial Case"/>
        <startEvent id="start"/>
        <endEvent id="end"/>
        <sequenceFlow id="f1" sourceRef="start" targetRef="end"/>
      </process>
    </definitions>"#;

    let definition = ProcessDefinition::from_xml(xml).unwrap();
    let element = definition.get_element("financialCase");
    assert!(element.is_some());
}

#[tokio::test]
async fn test_extension_elements_parsing() {
    let xml = r#"<?xml version="1.0"?>
    <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL">
      <process id="test_ext" isExecutable="true">
        <startEvent id="start"/>
        <userTask id="task1">
          <extensionElements>
            <custom:formKey xmlns:custom="custom">approval-form</custom:formKey>
          </extensionElements>
        </userTask>
        <endEvent id="end"/>
        <sequenceFlow id="f1" sourceRef="start" targetRef="task1"/>
        <sequenceFlow id="f2" sourceRef="task1" targetRef="end"/>
      </process>
    </definitions>"#;

    let definition = ProcessDefinition::from_xml(xml).unwrap();
    let element = definition.get_element("task1");
    assert!(element.is_some());
}

#[tokio::test]
async fn test_call_activity_parsing() {
    let xml = r#"<?xml version="1.0"?>
    <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL">
      <process id="test_call" isExecutable="true">
        <startEvent id="start"/>
        <callActivity id="call1" calledElement="subprocess1"/>
        <endEvent id="end"/>
        <sequenceFlow id="f1" sourceRef="start" targetRef="call1"/>
        <sequenceFlow id="f2" sourceRef="call1" targetRef="end"/>
      </process>
    </definitions>"#;

    let definition = ProcessDefinition::from_xml(xml).unwrap();
    let element = definition.get_element("call1");
    assert!(element.is_some());
}

#[tokio::test]
async fn test_multiple_features_together() {
    // Test that multiple features work together in a single process
    let xml = r#"<?xml version="1.0"?>
    <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL">
      <process id="multi_feature_test" isExecutable="true">
        <dataObject id="requestData" name="Request Data"/>
        <startEvent id="start"/>
        <userTask id="approve" name="Approve Request">
          <extensionElements>
            <custom:priority xmlns:custom="custom">high</custom:priority>
          </extensionElements>
        </userTask>
        <exclusiveGateway id="gateway1" gatewayDirection="Diverging"/>
        <intermediateThrowEvent id="notify">
          <signalEventDefinition signalRef="notificationSignal"/>
        </intermediateThrowEvent>
        <endEvent id="end"/>
        <sequenceFlow id="f1" sourceRef="start" targetRef="approve"/>
        <sequenceFlow id="f2" sourceRef="approve" targetRef="gateway1"/>
        <sequenceFlow id="f3" sourceRef="gateway1" targetRef="notify">
          <conditionExpression>approved == true</conditionExpression>
        </sequenceFlow>
        <sequenceFlow id="f4" sourceRef="gateway1" targetRef="end">
          <conditionExpression>approved == false</conditionExpression>
        </sequenceFlow>
        <sequenceFlow id="f5" sourceRef="notify" targetRef="end"/>
      </process>
    </definitions>"#;

    let definition = ProcessDefinition::from_xml(xml).unwrap();

    // Verify all elements are parsed correctly
    assert!(definition.get_element("requestData").is_some());
    assert!(definition.get_element("approve").is_some());
    assert!(definition.get_element("gateway1").is_some());
    assert!(definition.get_element("notify").is_some());

    let engine = Engine::new();
    let instance = engine.start_process(definition, None).await.unwrap();

    // Process should be active after user task
    let context = instance.context().await;
    assert!(matches!(context.state, ProcessInstanceState::Active));
}

#[tokio::test]
async fn test_escalation_event_parsing() {
    let xml = r#"<?xml version="1.0"?>
    <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL">
      <process id="test_escalation" isExecutable="true">
        <startEvent id="start"/>
        <intermediateThrowEvent id="escalate1">
          <escalationEventDefinition escalationRef="managerEscalation"/>
        </intermediateThrowEvent>
        <endEvent id="end"/>
        <sequenceFlow id="f1" sourceRef="start" targetRef="escalate1"/>
        <sequenceFlow id="f2" sourceRef="escalate1" targetRef="end"/>
      </process>
    </definitions>"#;

    let definition = ProcessDefinition::from_xml(xml).unwrap();
    let element = definition.get_element("escalate1");
    assert!(element.is_some());
}

#[tokio::test]
async fn test_multi_instance_loop_characteristics() {
    let xml = r#"<?xml version="1.0"?>
    <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL">
      <process id="test_multi_instance" isExecutable="true">
        <startEvent id="start"/>
        <userTask id="parallelTask">
          <multiInstanceLoopCharacteristics isSequential="false">
            <loopCardinality>3</loopCardinality>
          </multiInstanceLoopCharacteristics>
        </userTask>
        <endEvent id="end"/>
        <sequenceFlow id="f1" sourceRef="start" targetRef="parallelTask"/>
        <sequenceFlow id="f2" sourceRef="parallelTask" targetRef="end"/>
      </process>
    </definitions>"#;

    let definition = ProcessDefinition::from_xml(xml).unwrap();
    let element = definition.get_element("parallelTask");
    assert!(element.is_some());
}
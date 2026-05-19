//! Multi-Instance Loop Characteristics Tests
//!
//! Tests for multi-instance parsing and behavior.

use bpmn_engine::model::{ProcessDefinition, ProcessElement};

/// Test parsing a user task with parallel multi-instance
#[test]
fn test_parse_parallel_multi_instance_task() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL"
             xmlns:bpmn="http://www.omg.org/spec/BPMN/20100524/MODEL"
             targetNamespace="http://bpmn.io/schema/bpmn">
  <process id="test" isExecutable="true">
    <userTask id="task1" name="Parallel Approval">
      <multiInstanceLoopCharacteristics isParallel="true">
        <loopCardinality>3</loopCardinality>
      </multiInstanceLoopCharacteristics>
    </userTask>
  </process>
</definitions>"#;

    let result = ProcessDefinition::from_xml(xml);
    assert!(result.is_ok(), "Failed to parse XML: {:?}", result.err());
    let process = result.unwrap();

    let element = process.get_element("task1");
    assert!(element.is_some(), "Task 'task1' not found");

    match element {
        Some(ProcessElement::UserTask(task)) => {
            assert!(task.loop_characteristics.is_some(), "Loop characteristics not set");
            let mi = task.loop_characteristics.as_ref().unwrap();
            assert!(mi.is_parallel, "Expected parallel multi-instance");
            assert_eq!(mi.loop_cardinality, Some(3), "Expected loop cardinality of 3");
        }
        _ => panic!("Expected UserTask element"),
    }
}

/// Test parsing a service task with sequential multi-instance
#[test]
fn test_parse_sequential_multi_instance_task() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL"
             xmlns:bpmn="http://www.omg.org/spec/BPMN/20100524/MODEL"
             targetNamespace="http://bpmn.io/schema/bpmn">
  <process id="test" isExecutable="true">
    <serviceTask id="task1" name="Sequential Tasks">
      <multiInstanceLoopCharacteristics isParallel="false">
        <loopCardinality>5</loopCardinality>
        <completionCondition>nrOfCompletedInstances >= 5</completionCondition>
      </multiInstanceLoopCharacteristics>
    </serviceTask>
  </process>
</definitions>"#;

    let result = ProcessDefinition::from_xml(xml);
    assert!(result.is_ok(), "Failed to parse XML: {:?}", result.err());
    let process = result.unwrap();

    let element = process.get_element("task1");
    assert!(element.is_some(), "Task 'task1' not found");

    match element {
        Some(ProcessElement::ServiceTask(task)) => {
            assert!(task.loop_characteristics.is_some(), "Loop characteristics not set");
            let mi = task.loop_characteristics.as_ref().unwrap();
            assert!(!mi.is_parallel, "Expected sequential multi-instance");
            assert_eq!(mi.loop_cardinality, Some(5), "Expected loop cardinality of 5");
            assert_eq!(
                mi.completion_condition.as_deref(),
                Some("nrOfCompletedInstances >= 5"),
                "Expected completion condition"
            );
        }
        _ => panic!("Expected ServiceTask element"),
    }
}

/// Test parsing a script task without multi-instance
#[test]
fn test_parse_task_without_multi_instance() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL"
             xmlns:bpmn="http://www.omg.org/spec/BPMN/20100524/MODEL"
             targetNamespace="http://bpmn.io/schema/bpmn">
  <process id="test" isExecutable="true">
    <scriptTask id="task1" name="Simple Script">
    </scriptTask>
  </process>
</definitions>"#;

    let result = ProcessDefinition::from_xml(xml);
    assert!(result.is_ok(), "Failed to parse XML: {:?}", result.err());
    let process = result.unwrap();

    let element = process.get_element("task1");
    assert!(element.is_some(), "Task 'task1' not found");

    match element {
        Some(ProcessElement::ScriptTask(task)) => {
            assert!(task.loop_characteristics.is_none(), "Loop characteristics should be None");
        }
        _ => panic!("Expected ScriptTask element"),
    }
}

/// Test multi-instance with completion condition
#[test]
fn test_multi_instance_with_completion_condition() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL"
             xmlns:bpmn="http://www.omg.org/spec/BPMN/20100524/MODEL"
             targetNamespace="http://bpmn.io/schema/bpmn">
  <process id="test" isExecutable="true">
    <userTask id="task1" name="Review Task">
      <multiInstanceLoopCharacteristics isParallel="true">
        <loopCardinality>10</loopCardinality>
        <completionCondition>nrOfCompletedInstances >= 7</completionCondition>
      </multiInstanceLoopCharacteristics>
    </userTask>
  </process>
</definitions>"#;

    let result = ProcessDefinition::from_xml(xml);
    assert!(result.is_ok(), "Failed to parse XML: {:?}", result.err());
    let process = result.unwrap();

    let element = process.get_element("task1").unwrap();
    match element {
        ProcessElement::UserTask(task) => {
            let mi = task.loop_characteristics.as_ref().unwrap();
            assert_eq!(mi.loop_cardinality, Some(10));
            assert_eq!(mi.completion_condition.as_deref(), Some("nrOfCompletedInstances >= 7"));
            assert!(mi.is_parallel);
        }
        _ => panic!("Expected UserTask element"),
    }
}

/// Test that multi-instance with isParallel defaults to sequential (false)
#[test]
fn test_multi_instance_defaults_to_sequential() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL"
             xmlns:bpmn="http://www.omg.org/spec/BPMN/20100524/MODEL"
             targetNamespace="http://bpmn.io/schema/bpmn">
  <process id="test" isExecutable="true">
    <userTask id="task1" name="Default Task">
      <multiInstanceLoopCharacteristics>
        <loopCardinality>3</loopCardinality>
      </multiInstanceLoopCharacteristics>
    </userTask>
  </process>
</definitions>"#;

    let result = ProcessDefinition::from_xml(xml);
    assert!(result.is_ok(), "Failed to parse XML: {:?}", result.err());
    let process = result.unwrap();

    let element = process.get_element("task1").unwrap();
    match element {
        ProcessElement::UserTask(task) => {
            let mi = task.loop_characteristics.as_ref().unwrap();
            assert!(!mi.is_parallel, "Default should be sequential (not parallel)");
            assert_eq!(mi.loop_cardinality, Some(3));
        }
        _ => panic!("Expected UserTask element"),
    }
}
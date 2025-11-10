//! XML parsing and serialization tests

use bpmn_engine::model::format::{BpmnFormat, FormatDetector, ParseError};
use bpmn_engine::model::{parse_bpmn_xml, serialize_bpmn_xml, ProcessDefinition};

#[test]
fn test_format_detection_xml() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<bpmn2:definitions xmlns:bpmn2="http://www.omg.org/spec/BPMN/20100524/MODEL">
  <bpmn2:process id="test" isExecutable="true">
    <bpmn2:startEvent id="start" />
  </bpmn2:process>
</bpmn2:definitions>"#;

    let format = FormatDetector::detect(xml).unwrap();
    assert_eq!(format, BpmnFormat::Xml);
}

#[test]
fn test_format_detection_json() {
    let json = r#"{"id":"test","processType":"process","isExecutable":true,"elements":[]}"#;
    
    let format = FormatDetector::detect(json).unwrap();
    assert_eq!(format, BpmnFormat::Json);
}

#[test]
fn test_format_detection_empty() {
    let result = FormatDetector::detect("");
    assert!(result.is_err());
}

#[test]
fn test_parse_simple_xml() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<bpmn2:definitions xmlns:bpmn2="http://www.omg.org/spec/BPMN/20100524/MODEL">
  <bpmn2:process id="test_process" name="Test Process" isExecutable="true">
    <bpmn2:startEvent id="start" name="Start" />
    <bpmn2:endEvent id="end" name="End" />
    <bpmn2:sequenceFlow id="flow1" sourceRef="start" targetRef="end" />
  </bpmn2:process>
</bpmn2:definitions>"#;

    let result = parse_bpmn_xml(xml);
    assert!(result.is_ok());
    
    let definition = result.unwrap();
    assert_eq!(definition.id, "test_process");
    assert_eq!(definition.name, Some("Test Process".to_string()));
    assert!(definition.is_executable);
    assert_eq!(definition.elements.len(), 2);
    assert_eq!(definition.flows.len(), 1);
}

#[test]
fn test_parse_xml_with_tasks() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<bpmn2:definitions xmlns:bpmn2="http://www.omg.org/spec/BPMN/20100524/MODEL">
  <bpmn2:process id="process1" isExecutable="true">
    <bpmn2:startEvent id="start" />
    <bpmn2:serviceTask id="task1" name="Service Task" />
    <bpmn2:userTask id="task2" name="User Task" />
    <bpmn2:endEvent id="end" />
    <bpmn2:sequenceFlow id="flow1" sourceRef="start" targetRef="task1" />
    <bpmn2:sequenceFlow id="flow2" sourceRef="task1" targetRef="task2" />
    <bpmn2:sequenceFlow id="flow3" sourceRef="task2" targetRef="end" />
  </bpmn2:process>
</bpmn2:definitions>"#;

    let result = parse_bpmn_xml(xml);
    assert!(result.is_ok());
    
    let definition = result.unwrap();
    assert_eq!(definition.elements.len(), 4); // start, task1, task2, end
    assert_eq!(definition.flows.len(), 3);
}

#[test]
fn test_parse_xml_with_gateways() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<bpmn2:definitions xmlns:bpmn2="http://www.omg.org/spec/BPMN/20100524/MODEL">
  <bpmn2:process id="process1" isExecutable="true">
    <bpmn2:startEvent id="start" />
    <bpmn2:exclusiveGateway id="gateway1" />
    <bpmn2:parallelGateway id="gateway2" />
    <bpmn2:endEvent id="end" />
  </bpmn2:process>
</bpmn2:definitions>"#;

    let result = parse_bpmn_xml(xml);
    assert!(result.is_ok());
    
    let definition = result.unwrap();
    assert_eq!(definition.elements.len(), 4);
}

#[test]
fn test_serialize_xml() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<bpmn2:definitions xmlns:bpmn2="http://www.omg.org/spec/BPMN/20100524/MODEL">
  <bpmn2:process id="test_process" isExecutable="true">
    <bpmn2:startEvent id="start" />
    <bpmn2:endEvent id="end" />
    <bpmn2:sequenceFlow id="flow1" sourceRef="start" targetRef="end" />
  </bpmn2:process>
</bpmn2:definitions>"#;

    let definition = parse_bpmn_xml(xml).unwrap();
    let serialized = serialize_bpmn_xml(&definition).unwrap();
    
    // Verify serialized XML contains key elements
    assert!(serialized.contains("bpmn2:definitions"));
    assert!(serialized.contains("bpmn2:process"));
    assert!(serialized.contains("test_process"));
    assert!(serialized.contains("bpmn2:startEvent"));
    assert!(serialized.contains("bpmn2:endEvent"));
    assert!(serialized.contains("bpmn2:sequenceFlow"));
}

#[test]
fn test_round_trip_xml() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<bpmn2:definitions xmlns:bpmn2="http://www.omg.org/spec/BPMN/20100524/MODEL">
  <bpmn2:process id="roundtrip_test" name="Round Trip Test" isExecutable="true">
    <bpmn2:startEvent id="start" name="Start" />
    <bpmn2:serviceTask id="task1" name="Task 1" />
    <bpmn2:endEvent id="end" name="End" />
    <bpmn2:sequenceFlow id="flow1" sourceRef="start" targetRef="task1" />
    <bpmn2:sequenceFlow id="flow2" sourceRef="task1" targetRef="end" />
  </bpmn2:process>
</bpmn2:definitions>"#;

    let definition1 = parse_bpmn_xml(xml).unwrap();
    let serialized = serialize_bpmn_xml(&definition1).unwrap();
    let definition2 = parse_bpmn_xml(&serialized).unwrap();
    
    assert_eq!(definition1.id, definition2.id);
    assert_eq!(definition1.name, definition2.name);
    assert_eq!(definition1.elements.len(), definition2.elements.len());
    assert_eq!(definition1.flows.len(), definition2.flows.len());
}

#[test]
fn test_parse_xml_invalid() {
    let invalid_xml = "not xml";
    
    let result = parse_bpmn_xml(invalid_xml);
    assert!(result.is_err());
}

#[test]
fn test_auto_detect_and_parse() {
    let xml = r#"<?xml version="1.0"?>
<bpmn2:definitions xmlns:bpmn2="http://www.omg.org/spec/BPMN/20100524/MODEL">
  <bpmn2:process id="auto_test" isExecutable="true">
    <bpmn2:startEvent id="start" />
  </bpmn2:process>
</bpmn2:definitions>"#;

    let result = ProcessDefinition::from_auto(xml);
    assert!(result.is_ok());
    
    let (definition, format) = result.unwrap();
    assert_eq!(format, BpmnFormat::Xml);
    assert_eq!(definition.id, "auto_test");
}


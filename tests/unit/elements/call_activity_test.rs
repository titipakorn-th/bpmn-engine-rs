//! Call Activity unit tests

use bpmn_engine::model::elements::ProcessElement;
use bpmn_engine::model::ProcessDefinition;

#[test]
fn test_parse_call_activity_xml() {
    let xml = r#"<?xml version="1.0"?>
<definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL"
             xmlns:bpmn="http://www.omg.org/spec/BPMN/20100524/MODEL"
             id="Definitions_1">
  <process id="test" isExecutable="true">
    <callActivity id="call1" calledElement="subprocess1"/>
  </process>
</definitions>"#;

    let result = ProcessDefinition::from_xml(xml);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());

    let process = result.unwrap();
    let element = process.get_element("call1");
    assert!(element.is_some(), "Call activity 'call1' not found");

    match element.unwrap() {
        ProcessElement::CallActivity(ca) => {
            assert_eq!(ca.base.id, "call1");
            assert_eq!(ca.called_element.as_deref(), Some("subprocess1"));
            assert!(ca.business_key.is_none());
        }
        _ => panic!("Expected CallActivity"),
    }
}

#[test]
fn test_parse_call_activity_with_business_key() {
    let xml = r#"<?xml version="1.0"?>
<definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL"
             xmlns:bpmn="http://www.omg.org/spec/BPMN/20100524/MODEL"
             id="Definitions_1">
  <process id="test" isExecutable="true">
    <callActivity id="call2" calledElement="subprocess2" businessKey="${execution.businessKey}"/>
  </process>
</definitions>"#;

    let result = ProcessDefinition::from_xml(xml);
    assert!(result.is_ok());

    let process = result.unwrap();
    let element = process.get_element("call2").unwrap();

    match element {
        ProcessElement::CallActivity(ca) => {
            assert_eq!(ca.base.id, "call2");
            assert_eq!(ca.called_element.as_deref(), Some("subprocess2"));
            assert_eq!(ca.business_key.as_deref(), Some("${execution.businessKey}"));
        }
        _ => panic!("Expected CallActivity"),
    }
}

#[test]
fn test_parse_call_activity_with_name() {
    let xml = r#"<?xml version="1.0"?>
<definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL"
             xmlns:bpmn="http://www.omg.org/spec/BPMN/20100524/MODEL"
             id="Definitions_1">
  <process id="test" isExecutable="true">
    <callActivity id="call3" name="Invoke Subprocess" calledElement="subprocess3"/>
  </process>
</definitions>"#;

    let result = ProcessDefinition::from_xml(xml);
    assert!(result.is_ok());

    let process = result.unwrap();
    let element = process.get_element("call3").unwrap();

    match element {
        ProcessElement::CallActivity(ca) => {
            assert_eq!(ca.base.id, "call3");
            assert_eq!(ca.base.name.as_deref(), Some("Invoke Subprocess"));
            assert_eq!(ca.called_element.as_deref(), Some("subprocess3"));
        }
        _ => panic!("Expected CallActivity"),
    }
}

#[test]
fn test_parse_call_activity_with_data_associations() {
    let xml = r#"<?xml version="1.0"?>
<definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL"
             xmlns:bpmn="http://www.omg.org/spec/BPMN/20100524/MODEL"
             id="Definitions_1">
  <process id="test" isExecutable="true">
    <callActivity id="call4" calledElement="subprocess4">
      <dataInputAssociation>
        <sourceRef>inputVar</sourceRef>
        <targetRef>inputParam</targetRef>
      </dataInputAssociation>
      <dataOutputAssociation>
        <sourceRef>outputParam</sourceRef>
        <targetRef>resultVar</targetRef>
      </dataOutputAssociation>
    </callActivity>
  </process>
</definitions>"#;

    let result = ProcessDefinition::from_xml(xml);
    assert!(result.is_ok());

    let process = result.unwrap();
    let element = process.get_element("call4").unwrap();

    match element {
        ProcessElement::CallActivity(ca) => {
            assert_eq!(ca.base.id, "call4");
            assert_eq!(ca.called_element.as_deref(), Some("subprocess4"));
            assert_eq!(ca.data_input_associations.len(), 1);
            assert_eq!(ca.data_output_associations.len(), 1);
            assert_eq!(ca.data_input_associations[0].source_ref.as_deref(), Some("inputVar"));
            assert_eq!(ca.data_input_associations[0].target_ref.as_deref(), Some("inputParam"));
            assert_eq!(ca.data_output_associations[0].source_ref.as_deref(), Some("outputParam"));
            assert_eq!(ca.data_output_associations[0].target_ref.as_deref(), Some("resultVar"));
        }
        _ => panic!("Expected CallActivity"),
    }
}

#[test]
fn test_call_activity_json_roundtrip() {
    let json = r#"{
  "id": "test",
  "name": "Test Process",
  "processType": "process",
  "isExecutable": true,
  "elements": [
    {
      "type": "callActivity",
      "id": "call1",
      "name": "Call Subprocess",
      "calledElement": "subprocess1",
      "businessKey": "${initiator}"
    }
  ],
  "variables": {}
}"#;

    let result = ProcessDefinition::from_json(json);
    assert!(result.is_ok(), "Failed to parse JSON: {:?}", result.err());

    let process = result.unwrap();
    let element = process.get_element("call1").unwrap();

    match element {
        ProcessElement::CallActivity(ca) => {
            assert_eq!(ca.base.id, "call1");
            assert_eq!(ca.base.name.as_deref(), Some("Call Subprocess"));
            assert_eq!(ca.called_element.as_deref(), Some("subprocess1"));
            assert_eq!(ca.business_key.as_deref(), Some("${initiator}"));
        }
        _ => panic!("Expected CallActivity"),
    }

    // Test serialization back to JSON
    let serialized = process.to_json();
    assert!(serialized.is_ok());
    let json_str = serialized.unwrap();
    println!("Serialized JSON: {}", json_str);
    assert!(json_str.contains("calledElement"));
    assert!(json_str.contains("businessKey"));
}

#[test]
fn test_call_activity_element_id() {
    let json = r#"{
  "id": "test",
  "processType": "process",
  "isExecutable": true,
  "elements": [
    {"type": "callActivity", "id": "myCallActivity", "calledElement": "target"}
  ],
  "variables": {}
}"#;

    let process = ProcessDefinition::from_json(json).unwrap();
    let element = process.get_element("myCallActivity").unwrap();
    assert_eq!(element.id(), "myCallActivity");
}

#[test]
fn test_call_activity_to_json() {
    let json = r#"{
  "id": "test",
  "processType": "process",
  "isExecutable": true,
  "elements": [
    {"type": "callActivity", "id": "call1", "calledElement": "subprocess1"}
  ],
  "variables": {}
}"#;

    let process = ProcessDefinition::from_json(json).unwrap();
    let element = process.get_element("call1").unwrap();

    let json_elem = element.to_json_element();
    match json_elem {
        bpmn_engine::model::json::BpmnJsonElement::CallActivity(ca) => {
            assert_eq!(ca.base.id, "call1");
            assert_eq!(ca.called_element.as_deref(), Some("subprocess1"));
        }
        _ => panic!("Expected CallActivity JSON element"),
    }
}
use bpmn_engine::model::elements::{ProcessElement, DataObject, DataInput, DataOutput, DataObjectReference};

#[test]
fn test_parse_data_object() {
    let xml = r#"<?xml version="1.0"?>
    <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL"
                 xmlns:bpmn="http://www.omg.org/spec/BPMN/20100524/MODEL">
      <process id="test" isExecutable="true">
        <dataObject id="do1" name="FinancialCase"/>
      </process>
    </definitions>"#;

    let result = bpmn_engine::model::ProcessDefinition::from_xml(xml);
    assert!(result.is_ok(), "Failed to parse XML: {:?}", result.err());
    let process = result.unwrap();

    let element = process.get_element("do1");
    assert!(element.is_some(), "Element do1 not found");

    match element.unwrap() {
        ProcessElement::DataObject(do_obj) => {
            assert_eq!(do_obj.base.id, "do1");
            assert_eq!(do_obj.base.name.as_deref(), Some("FinancialCase"));
        }
        _ => panic!("Expected DataObject, got {:?}", element),
    }
}

#[test]
fn test_parse_data_object_reference() {
    let xml = r#"<?xml version="1.0"?>
    <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL"
                 xmlns:bpmn="http://www.omg.org/spec/BPMN/20100524/MODEL">
      <process id="test" isExecutable="true">
        <dataObject id="do1" name="FinancialCase"/>
        <dataObjectReference id="dor1" name="Case Reference" dataObjectRef="do1"/>
      </process>
    </definitions>"#;

    let result = bpmn_engine::model::ProcessDefinition::from_xml(xml);
    assert!(result.is_ok(), "Failed to parse XML: {:?}", result.err());
    let process = result.unwrap();

    let element = process.get_element("dor1");
    assert!(element.is_some(), "Element dor1 not found");

    match element.unwrap() {
        ProcessElement::DataObjectReference(dor) => {
            assert_eq!(dor.base.id, "dor1");
            assert_eq!(dor.base.name.as_deref(), Some("Case Reference"));
            assert_eq!(dor.data_object_ref.as_deref(), Some("do1"));
        }
        _ => panic!("Expected DataObjectReference, got {:?}", element),
    }
}

#[test]
fn test_parse_data_input() {
    let xml = r#"<?xml version="1.0"?>
    <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL"
                 xmlns:bpmn="http://www.omg.org/spec/BPMN/20100524/MODEL">
      <process id="test" isExecutable="true">
        <dataInput id="di1" name="InputData"/>
      </process>
    </definitions>"#;

    let result = bpmn_engine::model::ProcessDefinition::from_xml(xml);
    assert!(result.is_ok(), "Failed to parse XML: {:?}", result.err());
    let process = result.unwrap();

    let element = process.get_element("di1");
    assert!(element.is_some(), "Element di1 not found");

    match element.unwrap() {
        ProcessElement::DataInput(di) => {
            assert_eq!(di.base.id, "di1");
            assert_eq!(di.base.name.as_deref(), Some("InputData"));
        }
        _ => panic!("Expected DataInput, got {:?}", element),
    }
}

#[test]
fn test_parse_data_output() {
    let xml = r#"<?xml version="1.0"?>
    <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL"
                 xmlns:bpmn="http://www.omg.org/spec/BPMN/20100524/MODEL">
      <process id="test" isExecutable="true">
        <dataOutput id="do2" name="OutputData"/>
      </process>
    </definitions>"#;

    let result = bpmn_engine::model::ProcessDefinition::from_xml(xml);
    assert!(result.is_ok(), "Failed to parse XML: {:?}", result.err());
    let process = result.unwrap();

    let element = process.get_element("do2");
    assert!(element.is_some(), "Element do2 not found");

    match element.unwrap() {
        ProcessElement::DataOutput(dout) => {
            assert_eq!(dout.base.id, "do2");
            assert_eq!(dout.base.name.as_deref(), Some("OutputData"));
        }
        _ => panic!("Expected DataOutput, got {:?}", element),
    }
}

#[test]
fn test_parse_data_object_with_data_state() {
    let xml = r#"<?xml version="1.0"?>
    <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL"
                 xmlns:bpmn="http://www.omg.org/spec/BPMN/20100524/MODEL">
      <process id="test" isExecutable="true">
        <dataObject id="do1" name="FinancialCase" dataState="draft"/>
      </process>
    </definitions>"#;

    let result = bpmn_engine::model::ProcessDefinition::from_xml(xml);
    assert!(result.is_ok(), "Failed to parse XML: {:?}", result.err());
    let process = result.unwrap();

    let element = process.get_element("do1");
    assert!(element.is_some(), "Element do1 not found");

    match element.unwrap() {
        ProcessElement::DataObject(do_obj) => {
            assert_eq!(do_obj.base.id, "do1");
            assert_eq!(do_obj.base.name.as_deref(), Some("FinancialCase"));
            assert_eq!(do_obj.data_state.as_deref(), Some("draft"));
        }
        _ => panic!("Expected DataObject, got {:?}", element),
    }
}

#[test]
fn test_parse_multiple_data_objects() {
    let xml = r#"<?xml version="1.0"?>
    <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL"
                 xmlns:bpmn="http://www.omg.org/spec/BPMN/20100524/MODEL">
      <process id="test" isExecutable="true">
        <dataObject id="do1" name="FinancialCase"/>
        <dataObject id="do2" name="LayoutAttachment"/>
        <dataInput id="di1" name="InputData"/>
        <dataOutput id="do3" name="OutputData"/>
      </process>
    </definitions>"#;

    let result = bpmn_engine::model::ProcessDefinition::from_xml(xml);
    assert!(result.is_ok(), "Failed to parse XML: {:?}", result.err());
    let process = result.unwrap();

    assert!(process.get_element("do1").is_some());
    assert!(process.get_element("do2").is_some());
    assert!(process.get_element("di1").is_some());
    assert!(process.get_element("do3").is_some());
}

#[test]
fn test_data_object_json_roundtrip() {
    let json = r#"{
      "id": "test",
      "name": "Test Process",
      "isExecutable": true,
      "processType": "process",
      "elements": [
        {
          "type": "dataObject",
          "id": "do1",
          "name": "FinancialCase",
          "dataState": "draft"
        }
      ],
      "variables": {}
    }"#;

    let result = bpmn_engine::model::ProcessDefinition::from_json(json);
    assert!(result.is_ok(), "Failed to parse JSON: {:?}", result.err());
    let process = result.unwrap();

    let element = process.get_element("do1");
    assert!(element.is_some(), "Element do1 not found");

    match element.unwrap() {
        ProcessElement::DataObject(do_obj) => {
            assert_eq!(do_obj.base.id, "do1");
            assert_eq!(do_obj.base.name.as_deref(), Some("FinancialCase"));
            assert_eq!(do_obj.data_state.as_deref(), Some("draft"));
        }
        _ => panic!("Expected DataObject, got something else"),
    }

    // Test serialization back to JSON
    let serialized = process.to_json();
    assert!(serialized.is_ok(), "Failed to serialize to JSON: {:?}", serialized.err());
}

#[test]
fn test_data_object_reference_json_roundtrip() {
    let json = r#"{
      "id": "test",
      "name": "Test Process",
      "isExecutable": true,
      "processType": "process",
      "elements": [
        {
          "type": "dataObjectReference",
          "id": "dor1",
          "name": "Case Reference",
          "dataObjectRef": "do1"
        }
      ],
      "variables": {}
    }"#;

    let result = bpmn_engine::model::ProcessDefinition::from_json(json);
    assert!(result.is_ok(), "Failed to parse JSON: {:?}", result.err());
    let process = result.unwrap();

    let element = process.get_element("dor1");
    assert!(element.is_some(), "Element dor1 not found");

    match element.unwrap() {
        ProcessElement::DataObjectReference(dor) => {
            assert_eq!(dor.base.id, "dor1");
            assert_eq!(dor.base.name.as_deref(), Some("Case Reference"));
            assert_eq!(dor.data_object_ref.as_deref(), Some("do1"));
        }
        _ => panic!("Expected DataObjectReference, got something else"),
    }
}

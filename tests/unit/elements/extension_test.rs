use bpmn_engine::model::elements::ProcessElement;
use bpmn_engine::model::elements::ExtensionElements;

#[test]
fn test_parse_extension_elements() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
    <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL"
                 xmlns:bpmn2="http://www.omg.org/spec/BPMN/20100524/MODEL"
                 xmlns:custom="http://example.com/custom">
      <process id="test" isExecutable="true">
        <userTask id="task1" name="User Task">
          <extensionElements>
            <custom:formKey>approval-form</custom:formKey>
            <custom:priority>high</custom:priority>
          </extensionElements>
        </userTask>
      </process>
    </definitions>"#;

    let result = bpmn_engine::model::ProcessDefinition::from_xml(xml);
    assert!(result.is_ok(), "Parsing failed: {:?}", result.err());

    let process = result.unwrap();
    assert_eq!(process.id, "test");

    let task1 = process.elements.get("task1");
    assert!(task1.is_some(), "Task1 not found");

    match task1 {
        Some(ProcessElement::UserTask(task)) => {
            assert!(task.base.extension_elements.is_some(), "Extension elements should be present");
            let ext = task.base.extension_elements.as_ref().unwrap();
            assert_eq!(ext.properties.get("formKey"), Some(&"approval-form".to_string()));
            assert_eq!(ext.properties.get("priority"), Some(&"high".to_string()));
        }
        _ => panic!("Expected UserTask"),
    }
}

#[test]
fn test_extension_elements_with_service_task() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
    <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL"
                 xmlns:bpmn2="http://www.omg.org/spec/BPMN/20100524/MODEL">
      <process id="test" isExecutable="true">
        <serviceTask id="service1" name="Service Task">
          <extensionElements>
            <custom:config>{"timeout": 30}</custom:config>
            <custom:retryCount>3</custom:retryCount>
          </extensionElements>
        </serviceTask>
      </process>
    </definitions>"#;

    let result = bpmn_engine::model::ProcessDefinition::from_xml(xml);
    assert!(result.is_ok());

    let process = result.unwrap();
    let service1 = process.elements.get("service1");
    assert!(service1.is_some());

    match service1 {
        Some(ProcessElement::ServiceTask(task)) => {
            assert!(task.base.extension_elements.is_some());
            let ext = task.base.extension_elements.as_ref().unwrap();
            assert_eq!(ext.properties.get("config"), Some(&"{\"timeout\": 30}".to_string()));
            assert_eq!(ext.properties.get("retryCount"), Some(&"3".to_string()));
        }
        _ => panic!("Expected ServiceTask"),
    }
}

#[test]
fn test_extension_elements_empty() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
    <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL"
                 xmlns:bpmn2="http://www.omg.org/spec/BPMN/20100524/MODEL">
      <process id="test" isExecutable="true">
        <userTask id="task1" name="User Task">
        </userTask>
      </process>
    </definitions>"#;

    let result = bpmn_engine::model::ProcessDefinition::from_xml(xml);
    assert!(result.is_ok());

    let process = result.unwrap();
    let task1 = process.elements.get("task1").unwrap();

    match task1 {
        ProcessElement::UserTask(task) => {
            assert!(task.base.extension_elements.is_none(), "Extension elements should not be present for tasks without extensions");
        }
        _ => panic!("Expected UserTask"),
    }
}

#[test]
fn test_extension_elements_builder() {
    let ext = ExtensionElements::new()
        .with_property("key1".to_string(), "value1".to_string())
        .with_property("key2".to_string(), "value2".to_string());

    assert_eq!(ext.properties.len(), 2);
    assert_eq!(ext.properties.get("key1"), Some(&"value1".to_string()));
    assert_eq!(ext.properties.get("key2"), Some(&"value2".to_string()));
}
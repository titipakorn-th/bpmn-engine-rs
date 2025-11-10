//! Unit tests for process model

use bpmn_engine::model::ProcessDefinition;
use test_log::test;

#[test]
fn test_process_definition_basic() {
    let json = r#"
    {
        "id": "process1",
        "name": "Test Process",
        "isExecutable": true,
        "elements": [],
        "variables": {}
    }
    "#;

    let definition = ProcessDefinition::from_json(json).unwrap();
    assert_eq!(definition.id, "process1");
    assert_eq!(definition.name, Some("Test Process".to_string()));
    assert!(definition.is_executable);
}

#[test]
fn test_process_definition_with_variables() {
    let json = r#"
    {
        "id": "process1",
        "isExecutable": true,
        "elements": [],
        "variables": {
            "var1": {
                "name": "var1",
                "variableType": "string",
                "defaultValue": "test"
            }
        }
    }
    "#;

    let definition = ProcessDefinition::from_json(json).unwrap();
    assert_eq!(definition.variables.len(), 1);
    assert!(definition.variables.contains_key("var1"));
    let var = definition.variables.get("var1").unwrap();
    assert_eq!(var.name, "var1");
    assert_eq!(var.variable_type, Some("string".to_string()));
}


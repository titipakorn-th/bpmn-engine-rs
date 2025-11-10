//! Test Builders
//!
//! Builder pattern implementations for creating test data.

use bpmn_engine::model::ProcessDefinition;
use std::collections::HashMap;

/// Builder for ProcessDefinition
pub struct ProcessDefinitionBuilder {
    id: String,
    name: Option<String>,
    elements: Vec<serde_json::Value>,
    variables: HashMap<String, serde_json::Value>,
}

impl ProcessDefinitionBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: None,
            elements: Vec::new(),
            variables: HashMap::new(),
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn add_start_event(mut self, id: impl Into<String>, name: Option<&str>) -> Self {
        let mut event = serde_json::json!({
            "type": "startEvent",
            "id": id.into()
        });
        if let Some(n) = name {
            event["name"] = serde_json::Value::String(n.to_string());
        }
        self.elements.push(event);
        self
    }

    pub fn add_end_event(mut self, id: impl Into<String>, name: Option<&str>) -> Self {
        let mut event = serde_json::json!({
            "type": "endEvent",
            "id": id.into()
        });
        if let Some(n) = name {
            event["name"] = serde_json::Value::String(n.to_string());
        }
        self.elements.push(event);
        self
    }

    pub fn add_service_task(mut self, id: impl Into<String>, name: Option<&str>) -> Self {
        let mut task = serde_json::json!({
            "type": "serviceTask",
            "id": id.into()
        });
        if let Some(n) = name {
            task["name"] = serde_json::Value::String(n.to_string());
        }
        self.elements.push(task);
        self
    }

    pub fn add_sequence_flow(
        mut self,
        id: impl Into<String>,
        source_ref: impl Into<String>,
        target_ref: impl Into<String>,
    ) -> Self {
        let flow = serde_json::json!({
            "type": "sequenceFlow",
            "id": id.into(),
            "sourceRef": source_ref.into(),
            "targetRef": target_ref.into()
        });
        self.elements.push(flow);
        self
    }

    pub fn add_variable(mut self, name: impl Into<String>, value: serde_json::Value) -> Self {
        self.variables.insert(name.into(), value);
        self
    }

    pub fn build(self) -> Result<ProcessDefinition, serde_json::Error> {
        let mut process_json = serde_json::json!({
            "id": self.id,
            "isExecutable": true,
            "elements": self.elements,
            "variables": {}
        });

        if let Some(name) = self.name {
            process_json["name"] = serde_json::Value::String(name);
        }

        let json_str = serde_json::to_string(&process_json)?;
        ProcessDefinition::from_json(&json_str)
    }
}

impl Default for ProcessDefinitionBuilder {
    fn default() -> Self {
        Self::new("test_process")
    }
}


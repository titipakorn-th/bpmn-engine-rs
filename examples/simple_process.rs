//! Simple Process Example
//!
//! Example demonstrating basic BPMN process execution.

use bpmn_engine::{Engine, ProcessDefinition};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create engine
    let engine = Engine::new();

    // Define a simple process: Start -> Task -> End
    let process_json = r#"
    {
        "id": "simple_process",
        "name": "Simple Process",
        "isExecutable": true,
        "elements": [
            {
                "type": "startEvent",
                "id": "start1",
                "name": "Start"
            },
            {
                "type": "serviceTask",
                "id": "task1",
                "name": "Process Task"
            },
            {
                "type": "endEvent",
                "id": "end1",
                "name": "End"
            },
            {
                "type": "sequenceFlow",
                "id": "flow1",
                "sourceRef": "start1",
                "targetRef": "task1"
            },
            {
                "type": "sequenceFlow",
                "id": "flow2",
                "sourceRef": "task1",
                "targetRef": "end1"
            }
        ],
        "variables": {}
    }
    "#;

    // Parse process definition
    let definition = ProcessDefinition::from_json(process_json)?;

    // Set initial variables
    let mut initial_variables = HashMap::new();
    initial_variables.insert("input".to_string(), serde_json::json!("test"));

    // Start process
    let instance = engine.start_process(definition, Some(initial_variables)).await?;

    println!("Process instance created: {}", instance.id());

    // Get instance state
    let context = instance.context().await;
    println!("Process state: {:?}", context.state);
    println!("Execution steps: {}", context.execution_history.len());

    Ok(())
}


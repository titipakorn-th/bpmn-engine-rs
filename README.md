# BPMN Engine Rust

BPMN 2.0 execution engine for Rust with support for condition evaluation, parallel gateways, and JSON/XML format parsing.

## Features

### Core BPMN 2.0 Elements
- **Events**: StartEvent, EndEvent, IntermediateCatchEvent, IntermediateThrowEvent
- **Tasks**: ServiceTask, UserTask, ScriptTask, ManualTask
- **Gateways**:
  - **Exclusive Gateway** — XOR routing with condition evaluation
  - **Parallel Gateway** — AND-split/join with token tracking
  - **Inclusive Gateway** — Multi-condition routing

### Condition Expression Evaluation
Supports expressions in the format `fieldPath operator value`:
```rust
// Examples
"status = approved"
"estimated_sales > 10000"
"tags contains urgent"
"site.region = South"
```

### Parallel Gateway Token Tracking
The engine tracks tokens for parallel gateway joins — waiting for all incoming branches to complete before proceeding through the join.

### Format Support
- **JSON** — Full parsing and serialization
- **XML** — Full parsing and serialization with namespace support
- **Auto-detection** — Automatically detects format and parses accordingly

### Architecture
- Activity/Capability-based design
- Extensible listener system for process monitoring
- Async execution support via Tokio

## Installation

```toml
[dependencies]
bpmn-engine = "0.1.0"
```

Or use a local path:
```toml
[dependencies]
bpmn-engine = { path = "./vendor/bpmn-engine" }
```

## Usage

### Parse and Execute a Process
```rust
use bpmn_engine::{Engine, ProcessDefinition};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml = r#"<?xml version="1.0"?>
    <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL">
      <process id="process1" isExecutable="true">
        <startEvent id="StartEvent_1"/>
        <userTask id="Task_1" name="Review"/>
        <endEvent id="EndEvent_1"/>
        <sequenceFlow id="F1" sourceRef="StartEvent_1" targetRef="Task_1"/>
        <sequenceFlow id="F2" sourceRef="Task_1" targetRef="EndEvent_1"/>
      </process>
    </definitions>"#;

    let definition = ProcessDefinition::from_xml(xml)?;
    let engine = Engine::new();

    // Start process with initial variables
    let instance = engine.start_process(definition, None).await?;
    println!("Started: {}", instance.id());

    Ok(())
}
```

### Condition Evaluation
```rust
// Exclusive gateway with conditions
// XML: <conditionExpression>status = approved</conditionExpression>

let mut context = ExecutionContext::new(definition, "instance_1".to_string());
context.set_variable("status".to_string(), serde_json::json!("approved"));

// Gateway evaluates conditions against context.variables
```

### Parallel Gateway Join
```rust
// When a parallel gateway acts as a join (N incoming flows):
// - First branch arrives → gateway returns Waiting
// - Second branch arrives → tokens tracked
// - All branches arrive → gateway proceeds to outgoing
```

## Testing

```bash
cargo test
```

## Status

| Feature | Status |
|---------|--------|
| XML/JSON Parsing | ✅ Complete |
| Condition Evaluation | ✅ Complete |
| Parallel Gateway AND-join | ✅ Complete |
| Exclusive Gateway | ✅ Complete |
| Inclusive Gateway | ✅ Complete |
| User Task (Waiting) | ✅ Complete |
| ProcessListener | 🚧 In Progress |

## License

Apache-2.0

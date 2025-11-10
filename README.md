# BPMN Engine Rust

BPMN 2.0 execution engine for Rust, based on [bpmn-engine](https://www.npmjs.com/package/bpmn-engine) npm package.

## Features

- BPMN 2.0 JSON and XML format support (standard I/O)
- Automatic format detection (JSON/XML)
- Bidirectional conversion (JSON ↔ XML ↔ Internal Model)
- Activity/Capability-based design following DoDAF v2 DM2 principles
- High-performance, type-safe execution engine
- Extensible architecture for custom tasks and listeners
- Future-ready for GraphQL API and persistence layer integration
- 100% test coverage (TDD)

## Design

This project follows Semantic Driven Development principles with:
- DoDAF v2 DM2-based OWL design files
- Activity/Capability modeling
- JSON-LD semantic annotations

See `PROJECT.jsonld`, `capabilities.jsonld`, and `activities.jsonld` for design documentation.

## Testing

The project maintains 100% test coverage with comprehensive test suites:

- **Unit Tests**: 74 tests covering all modules (including XML parsing/serialization)
- **Integration Tests**: 4 tests for end-to-end process execution
- **Test Infrastructure**: Mock implementations, fixtures, and builders

Run tests:
```bash
cargo test
```

Generate coverage report:
```bash
make test-coverage
```

## Status

✅ Core implementation completed
✅ XML format support completed
✅ Format detection and auto-parsing implemented
✅ JSON/XML serialization implemented
✅ Test infrastructure setup completed
✅ 79 tests passing (74 unit + 4 integration + 1 doc)

## Usage

### Parse BPMN JSON
```rust
use bpmn_engine::model::ProcessDefinition;

let json = r#"{"id":"process1","processType":"process","isExecutable":true,"elements":[]}"#;
let definition = ProcessDefinition::from_json(json)?;
```

### Parse BPMN XML
```rust
use bpmn_engine::model::ProcessDefinition;

let xml = r#"<?xml version="1.0"?>
<bpmn2:definitions xmlns:bpmn2="http://www.omg.org/spec/BPMN/20100524/MODEL">
  <bpmn2:process id="process1" isExecutable="true">
    <bpmn2:startEvent id="start" />
  </bpmn2:process>
</bpmn2:definitions>"#;
let definition = ProcessDefinition::from_xml(xml)?;
```

### Auto-detect Format
```rust
use bpmn_engine::model::ProcessDefinition;

let input = "..." // JSON or XML
let (definition, format) = ProcessDefinition::from_auto(input)?;
```

### Serialize to JSON/XML
```rust
let json = definition.to_json()?;
let xml = definition.to_xml()?;
```

## License

Apache-2.0


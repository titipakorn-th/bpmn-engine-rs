# BPMN Engine Rust

BPMN 2.0 execution engine for Rust, based on [bpmn-engine](https://www.npmjs.com/package/bpmn-engine) npm package.

## Features

- BPMN 2.0 JSON format support (standard I/O)
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

- **Unit Tests**: 64 tests covering all modules
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
✅ Test infrastructure setup completed
✅ 68 tests passing

## License

MIT


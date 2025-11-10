//! # BPMN Engine
//!
//! BPMN 2.0 execution engine for Rust.
//!
//! This crate provides a high-performance, type-safe BPMN 2.0 execution engine
//! that supports both BPMN 2.0 JSON and XML formats as standard I/O, with
//! automatic format detection and bidirectional conversion.
//!
//! ## Design Principles
//!
//! - **Activity/Capability-based design**: Following DoDAF v2 DM2 principles
//! - **Type safety**: Leveraging Rust's type system
//! - **Extensibility**: Support for custom tasks and listeners
//! - **Future-ready**: Designed for GraphQL API and persistence layer integration
//!
//! ## Examples
//!
//! ### Parse JSON
//! ```no_run
//! use bpmn_engine::model::ProcessDefinition;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let definition = ProcessDefinition::from_json(r#"
//! {
//!   "id": "process1",
//!   "name": "Example Process",
//!   "elements": []
//! }"#)?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Parse XML
//! ```no_run
//! use bpmn_engine::model::ProcessDefinition;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let definition = ProcessDefinition::from_xml(r#"
//! <?xml version="1.0"?>
//! <bpmn2:definitions xmlns:bpmn2="http://www.omg.org/spec/BPMN/20100524/MODEL">
//!   <bpmn2:process id="process1" isExecutable="true">
//!     <bpmn2:startEvent id="start" />
//!   </bpmn2:process>
//! </bpmn2:definitions>"#)?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Auto-detect Format
//! ```no_run
//! use bpmn_engine::model::{ProcessDefinition, format::BpmnFormat};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let input = r#"{"id":"process1","elements":[]}"#;
//! let (definition, format) = ProcessDefinition::from_auto(input)?;
//! assert_eq!(format, BpmnFormat::Json);
//! # Ok(())
//! # }
//! ```

pub mod activity;
pub mod capability;
pub mod elements;
pub mod engine;
pub mod model;
pub mod repository;

pub use engine::{Engine, EngineBuilder, EngineError};
pub use engine::instance::ProcessInstance;
pub use engine::context::ExecutionContext;
pub use model::ProcessDefinition;
pub use activity::{Activity, ActivityError, ActivityResult, ProcessListener, ActivityFactory, DefaultActivityFactory};


//! # BPMN Engine
//!
//! BPMN 2.0 execution engine for Rust.
//!
//! This crate provides a high-performance, type-safe BPMN 2.0 execution engine
//! that supports BPMN 2.0 JSON format as standard I/O.
//!
//! ## Design Principles
//!
//! - **Activity/Capability-based design**: Following DoDAF v2 DM2 principles
//! - **Type safety**: Leveraging Rust's type system
//! - **Extensibility**: Support for custom tasks and listeners
//! - **Future-ready**: Designed for GraphQL API and persistence layer integration
//!
//! ## Example
//!
//! ```no_run
//! use bpmn_engine::{Engine, ProcessDefinition};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let engine = Engine::new();
//! let definition = ProcessDefinition::from_json(r#"
//! {
//!   "id": "process1",
//!   "name": "Example Process",
//!   "elements": []
//! }"#)?;
//!
//! let instance = engine.start_process(definition).await?;
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
pub use model::ProcessDefinition;


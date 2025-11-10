//! BPMN Model
//!
//! Core data structures for representing BPMN 2.0 process definitions and instances.

pub mod elements;
pub mod json;
pub mod process;

pub use elements::ProcessDefinition;
pub use json::*;
pub use process::*;


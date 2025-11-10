//! BPMN Model
//!
//! Core data structures for representing BPMN 2.0 process definitions and instances.

pub mod elements;
pub mod json;
pub mod process;

pub use elements::{ProcessDefinition, ProcessElement, ElementBase, StartEvent, EndEvent, IntermediateCatchEvent, IntermediateThrowEvent, ServiceTask, UserTask, ScriptTask, ManualTask, ExclusiveGateway, ParallelGateway, InclusiveGateway, SequenceFlow, EventDefinition, ConditionExpression, Variable};
pub use json::*;
pub use process::*;


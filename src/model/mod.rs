//! BPMN Model
//!
//! Core data structures for representing BPMN 2.0 process definitions and instances.

pub mod elements;
pub mod format;
pub mod json;
pub mod process;
pub mod xml;

pub use elements::{ProcessDefinition, ProcessElement, ElementBase, StartEvent, EndEvent, IntermediateCatchEvent, IntermediateThrowEvent, ServiceTask, UserTask, ScriptTask, ManualTask, ExclusiveGateway, ParallelGateway, InclusiveGateway, SequenceFlow, EventDefinition, ConditionExpression, Variable};
pub use format::{BpmnFormat, FormatDetector, ParseError, SerializeError, BpmnParser, JsonParser, XmlParser, AutoParser};
pub use json::*;
pub use process::*;
pub use xml::{parse_bpmn_xml, serialize_bpmn_xml};


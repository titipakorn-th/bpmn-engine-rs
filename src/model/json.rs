//! BPMN 2.0 JSON Format
//!
//! JSON representation of BPMN 2.0 process definitions.
//!
//! This module defines the JSON schema for BPMN 2.0 processes,
//! designed to be compatible with standard BPMN 2.0 concepts
//! while using JSON as the serialization format.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// BPMN 2.0 JSON Process Definition
///
/// Represents a complete BPMN process definition in JSON format.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BpmnJsonProcess {
    /// Process ID
    pub id: String,
    /// Process name
    pub name: Option<String>,
    /// Process type (default: "process")
    #[serde(default = "default_process_type")]
    pub process_type: String,
    /// Is executable
    #[serde(default = "default_true")]
    pub is_executable: bool,
    /// Process elements (tasks, gateways, events, flows)
    pub elements: Vec<BpmnJsonElement>,
    /// Process variables
    #[serde(default)]
    pub variables: HashMap<String, BpmnJsonVariable>,
}

fn default_process_type() -> String {
    "process".to_string()
}

fn default_true() -> bool {
    true
}

/// BPMN JSON Element
///
/// Represents any BPMN element (task, gateway, event, flow, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum BpmnJsonElement {
    /// Start Event
    StartEvent(BpmnJsonStartEvent),
    /// End Event
    EndEvent(BpmnJsonEndEvent),
    /// Intermediate Catch Event
    IntermediateCatchEvent(BpmnJsonIntermediateCatchEvent),
    /// Intermediate Throw Event
    IntermediateThrowEvent(BpmnJsonIntermediateThrowEvent),
    /// Service Task
    ServiceTask(BpmnJsonServiceTask),
    /// User Task
    UserTask(BpmnJsonUserTask),
    /// Script Task
    ScriptTask(BpmnJsonScriptTask),
    /// Manual Task
    ManualTask(BpmnJsonManualTask),
    /// Exclusive Gateway
    ExclusiveGateway(BpmnJsonExclusiveGateway),
    /// Parallel Gateway
    ParallelGateway(BpmnJsonParallelGateway),
    /// Inclusive Gateway
    InclusiveGateway(BpmnJsonInclusiveGateway),
    /// Data Object
    DataObject(BpmnJsonDataObject),
    /// Data Input
    DataInput(BpmnJsonDataInput),
    /// Data Output
    DataOutput(BpmnJsonDataOutput),
    /// Data Object Reference
    DataObjectReference(BpmnJsonDataObjectReference),
    /// Call Activity
    CallActivity(BpmnJsonCallActivity),
    /// Sequence Flow
    SequenceFlow(BpmnJsonSequenceFlow),
}

/// Base properties for all BPMN elements
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BpmnJsonElementBase {
    /// Element ID
    pub id: String,
    /// Element name
    pub name: Option<String>,
    /// Documentation
    pub documentation: Option<String>,
}

/// Start Event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BpmnJsonStartEvent {
    #[serde(flatten)]
    pub base: BpmnJsonElementBase,
    /// Event definition (message, timer, signal, etc.)
    pub event_definition: Option<BpmnJsonEventDefinition>,
}

/// End Event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BpmnJsonEndEvent {
    #[serde(flatten)]
    pub base: BpmnJsonElementBase,
    /// Event definition
    pub event_definition: Option<BpmnJsonEventDefinition>,
}

/// Intermediate Catch Event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BpmnJsonIntermediateCatchEvent {
    #[serde(flatten)]
    pub base: BpmnJsonElementBase,
    /// Event definition
    pub event_definition: Option<BpmnJsonEventDefinition>,
}

/// Intermediate Throw Event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BpmnJsonIntermediateThrowEvent {
    #[serde(flatten)]
    pub base: BpmnJsonElementBase,
    /// Event definition
    pub event_definition: Option<BpmnJsonEventDefinition>,
}

/// Service Task
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BpmnJsonServiceTask {
    #[serde(flatten)]
    pub base: BpmnJsonElementBase,
    /// Implementation (e.g., "webService", "expression")
    pub implementation: Option<String>,
    /// Operation reference
    pub operation_ref: Option<String>,
    /// Input/output mappings
    #[serde(default)]
    pub io_mapping: BpmnJsonIoMapping,
    /// Multi-instance loop characteristics
    #[serde(default)]
    pub loop_characteristics: Option<BpmnJsonMultiInstanceLoopCharacteristics>,
}

/// User Task
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BpmnJsonUserTask {
    #[serde(flatten)]
    pub base: BpmnJsonElementBase,
    /// Assignment
    pub assignment: Option<BpmnJsonAssignment>,
    /// Form key
    pub form_key: Option<String>,
    /// Multi-instance loop characteristics
    #[serde(default)]
    pub loop_characteristics: Option<BpmnJsonMultiInstanceLoopCharacteristics>,
}

/// Script Task
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BpmnJsonScriptTask {
    #[serde(flatten)]
    pub base: BpmnJsonElementBase,
    /// Script format (e.g., "javascript", "groovy")
    pub script_format: Option<String>,
    /// Script content
    pub script: Option<String>,
    /// Multi-instance loop characteristics
    #[serde(default)]
    pub loop_characteristics: Option<BpmnJsonMultiInstanceLoopCharacteristics>,
}

/// Manual Task
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BpmnJsonManualTask {
    #[serde(flatten)]
    pub base: BpmnJsonElementBase,
}

/// Exclusive Gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BpmnJsonExclusiveGateway {
    #[serde(flatten)]
    pub base: BpmnJsonElementBase,
    /// Default flow ID
    pub default_flow: Option<String>,
}

/// Parallel Gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BpmnJsonParallelGateway {
    #[serde(flatten)]
    pub base: BpmnJsonElementBase,
    /// Default flow ID
    pub default_flow: Option<String>,
    /// Gateway direction
    pub gateway_direction: Option<crate::engine::context::GatewayDirection>,
}

/// Inclusive Gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BpmnJsonInclusiveGateway {
    #[serde(flatten)]
    pub base: BpmnJsonElementBase,
    /// Default flow ID
    pub default_flow: Option<String>,
}

/// Data Object
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BpmnJsonDataObject {
    #[serde(flatten)]
    pub base: BpmnJsonElementBase,
    /// Data state
    pub data_state: Option<String>,
}

/// Data Input
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BpmnJsonDataInput {
    #[serde(flatten)]
    pub base: BpmnJsonElementBase,
    /// Input set reference
    pub input_set: Option<String>,
}

/// Data Output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BpmnJsonDataOutput {
    #[serde(flatten)]
    pub base: BpmnJsonElementBase,
    /// Output set reference
    pub output_set: Option<String>,
}

/// Data Object Reference
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BpmnJsonDataObjectReference {
    #[serde(flatten)]
    pub base: BpmnJsonElementBase,
    /// Data object reference
    pub data_object_ref: Option<String>,
}

/// Call Activity
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BpmnJsonCallActivity {
    #[serde(flatten)]
    pub base: BpmnJsonElementBase,
    /// Called element ID (reference to external process)
    pub called_element: Option<String>,
    /// Business key expression
    pub business_key: Option<String>,
    /// Input data associations
    #[serde(default)]
    pub data_input_associations: Vec<BpmnJsonDataAssociation>,
    /// Output data associations
    #[serde(default)]
    pub data_output_associations: Vec<BpmnJsonDataAssociation>,
}

/// Data Association
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BpmnJsonDataAssociation {
    /// Source reference
    pub source_ref: Option<String>,
    /// Target reference
    pub target_ref: Option<String>,
    /// Transformation expression
    pub transformation: Option<String>,
}

/// Sequence Flow
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BpmnJsonSequenceFlow {
    #[serde(flatten)]
    pub base: BpmnJsonElementBase,
    /// Source element ID
    pub source_ref: String,
    /// Target element ID
    pub target_ref: String,
    /// Condition expression
    pub condition_expression: Option<BpmnJsonConditionExpression>,
}

/// Event Definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum BpmnJsonEventDefinition {
    /// Message Event Definition
    Message {
        /// Message reference
        message_ref: Option<String>,
    },
    /// Timer Event Definition
    Timer {
        /// Time definition (e.g., "PT1H", "R/PT1H")
        time_definition: Option<String>,
    },
    /// Signal Event Definition
    Signal {
        /// Signal reference
        signal_ref: Option<String>,
    },
    /// Error Event Definition
    Error {
        /// Error reference
        error_ref: Option<String>,
    },
    /// Escalation Event Definition
    Escalation {
        /// Escalation reference
        escalation_ref: Option<String>,
    },
    /// Cancel Event Definition
    Cancel,
    /// Compensation Event Definition
    Compensation {
        /// Activity reference
        activity_ref: Option<String>,
    },
    /// Conditional Event Definition
    Conditional {
        /// Condition expression
        condition: Option<BpmnJsonConditionExpression>,
    },
    /// Link Event Definition
    Link {
        /// Link name
        name: Option<String>,
    },
    /// Terminate Event Definition
    Terminate,
    /// None (no specific event definition)
    None,
}

/// Condition Expression
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BpmnJsonConditionExpression {
    /// Expression language (e.g., "javascript", "groovy")
    pub language: Option<String>,
    /// Expression body
    pub body: String,
}

/// Input/Output Mapping
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BpmnJsonIoMapping {
    /// Input parameters
    #[serde(default)]
    pub input_parameters: Vec<BpmnJsonIoParameter>,
    /// Output parameters
    #[serde(default)]
    pub output_parameters: Vec<BpmnJsonIoParameter>,
}

/// Input/Output Parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BpmnJsonIoParameter {
    /// Parameter name
    pub name: String,
    /// Parameter source/target
    pub source: Option<String>,
    pub target: Option<String>,
    /// Parameter value/expression
    pub value: Option<String>,
}

/// Assignment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BpmnJsonAssignment {
    /// Assignment type (e.g., "assignee", "candidateUsers", "candidateGroups")
    pub assignment_type: String,
    /// Assignment value
    pub value: String,
}

/// Variable Definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BpmnJsonVariable {
    /// Variable name
    pub name: String,
    /// Variable type
    pub variable_type: Option<String>,
    /// Default value
    pub default_value: Option<serde_json::Value>,
}

/// Multi-Instance Loop Characteristics
///
/// JSON representation of BPMN multi-instance loop characteristics.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BpmnJsonMultiInstanceLoopCharacteristics {
    /// Sequential (false) or parallel (true) execution
    pub is_parallel: Option<bool>,
    /// Number of instances to create
    pub loop_cardinality: Option<i32>,
    /// Completion condition expression
    pub completion_condition: Option<String>,
    /// Behavior when one instance completes
    pub behavior: Option<String>,
}


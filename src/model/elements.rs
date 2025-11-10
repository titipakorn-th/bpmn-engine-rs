//! BPMN Model Elements
//!
//! Internal representation of BPMN elements after parsing from JSON.

use crate::model::json::*;
use std::collections::HashMap;

/// Process Definition
///
/// Internal representation of a BPMN process definition.
#[derive(Debug, Clone)]
pub struct ProcessDefinition {
    /// Process ID
    pub id: String,
    /// Process name
    pub name: Option<String>,
    /// Process type
    pub process_type: String,
    /// Is executable
    pub is_executable: bool,
    /// Process elements indexed by ID
    pub elements: HashMap<String, ProcessElement>,
    /// Sequence flows indexed by ID
    pub flows: HashMap<String, SequenceFlow>,
    /// Variables
    pub variables: HashMap<String, Variable>,
}

impl ProcessDefinition {
    /// Create a new process definition from JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        let json_process: BpmnJsonProcess = serde_json::from_str(json)?;
        Self::from_bpmn_json(json_process)
    }

    /// Create from BPMn JSON structure
    pub fn from_bpmn_json(json_process: BpmnJsonProcess) -> Result<Self, serde_json::Error> {
        let mut elements = HashMap::new();
        let mut flows = HashMap::new();

        for element in json_process.elements {
            match element {
                BpmnJsonElement::SequenceFlow(flow) => {
                    let seq_flow = SequenceFlow::from_json(flow);
                    flows.insert(seq_flow.id.clone(), seq_flow);
                }
                _ => {
                    let process_elem = ProcessElement::from_json_element(element)?;
                    elements.insert(process_elem.id().to_string(), process_elem);
                }
            }
        }

        Ok(Self {
            id: json_process.id,
            name: json_process.name,
            process_type: json_process.process_type,
            is_executable: json_process.is_executable,
            elements,
            flows,
            variables: json_process
                .variables
                .into_iter()
                .map(|(k, v)| (k, Variable::from_json(v)))
                .collect(),
        })
    }

    /// Get element by ID
    pub fn get_element(&self, id: &str) -> Option<&ProcessElement> {
        self.elements.get(id)
    }

    /// Get sequence flow by ID
    pub fn get_flow(&self, id: &str) -> Option<&SequenceFlow> {
        self.flows.get(id)
    }

    /// Get outgoing flows from an element
    pub fn get_outgoing_flows(&self, element_id: &str) -> Vec<&SequenceFlow> {
        self.flows
            .values()
            .filter(|flow| flow.source_ref == element_id)
            .collect()
    }

    /// Get incoming flows to an element
    pub fn get_incoming_flows(&self, element_id: &str) -> Vec<&SequenceFlow> {
        self.flows
            .values()
            .filter(|flow| flow.target_ref == element_id)
            .collect()
    }
}

/// Process Element
///
/// Represents any executable BPMN element (task, gateway, event).
#[derive(Debug, Clone)]
pub enum ProcessElement {
    /// Start Event
    StartEvent(StartEvent),
    /// End Event
    EndEvent(EndEvent),
    /// Intermediate Catch Event
    IntermediateCatchEvent(IntermediateCatchEvent),
    /// Intermediate Throw Event
    IntermediateThrowEvent(IntermediateThrowEvent),
    /// Service Task
    ServiceTask(ServiceTask),
    /// User Task
    UserTask(UserTask),
    /// Script Task
    ScriptTask(ScriptTask),
    /// Manual Task
    ManualTask(ManualTask),
    /// Exclusive Gateway
    ExclusiveGateway(ExclusiveGateway),
    /// Parallel Gateway
    ParallelGateway(ParallelGateway),
    /// Inclusive Gateway
    InclusiveGateway(InclusiveGateway),
}

impl ProcessElement {
    pub fn from_json_element(element: BpmnJsonElement) -> Result<Self, serde_json::Error> {
        match element {
            BpmnJsonElement::StartEvent(e) => Ok(ProcessElement::StartEvent(StartEvent::from_json(e))),
            BpmnJsonElement::EndEvent(e) => Ok(ProcessElement::EndEvent(EndEvent::from_json(e))),
            BpmnJsonElement::IntermediateCatchEvent(e) => {
                Ok(ProcessElement::IntermediateCatchEvent(IntermediateCatchEvent::from_json(e)))
            }
            BpmnJsonElement::IntermediateThrowEvent(e) => {
                Ok(ProcessElement::IntermediateThrowEvent(IntermediateThrowEvent::from_json(e)))
            }
            BpmnJsonElement::ServiceTask(e) => {
                Ok(ProcessElement::ServiceTask(ServiceTask::from_json(e)))
            }
            BpmnJsonElement::UserTask(e) => Ok(ProcessElement::UserTask(UserTask::from_json(e))),
            BpmnJsonElement::ScriptTask(e) => Ok(ProcessElement::ScriptTask(ScriptTask::from_json(e))),
            BpmnJsonElement::ManualTask(e) => Ok(ProcessElement::ManualTask(ManualTask::from_json(e))),
            BpmnJsonElement::ExclusiveGateway(e) => {
                Ok(ProcessElement::ExclusiveGateway(ExclusiveGateway::from_json(e)))
            }
            BpmnJsonElement::ParallelGateway(e) => {
                Ok(ProcessElement::ParallelGateway(ParallelGateway::from_json(e)))
            }
            BpmnJsonElement::InclusiveGateway(e) => {
                Ok(ProcessElement::InclusiveGateway(InclusiveGateway::from_json(e)))
            }
            BpmnJsonElement::SequenceFlow(_) => {
                use serde::de::Error;
                Err(serde_json::Error::custom("SequenceFlow should be handled separately"))
            }
        }
    }

    pub fn id(&self) -> &str {
        match self {
            ProcessElement::StartEvent(e) => &e.base.id,
            ProcessElement::EndEvent(e) => &e.base.id,
            ProcessElement::IntermediateCatchEvent(e) => &e.base.id,
            ProcessElement::IntermediateThrowEvent(e) => &e.base.id,
            ProcessElement::ServiceTask(e) => &e.base.id,
            ProcessElement::UserTask(e) => &e.base.id,
            ProcessElement::ScriptTask(e) => &e.base.id,
            ProcessElement::ManualTask(e) => &e.base.id,
            ProcessElement::ExclusiveGateway(e) => &e.base.id,
            ProcessElement::ParallelGateway(e) => &e.base.id,
            ProcessElement::InclusiveGateway(e) => &e.base.id,
        }
    }
}

/// Base element properties
#[derive(Debug, Clone)]
pub struct ElementBase {
    pub id: String,
    pub name: Option<String>,
    pub documentation: Option<String>,
}

impl ElementBase {
    pub fn from_json(base: BpmnJsonElementBase) -> Self {
        Self {
            id: base.id,
            name: base.name,
            documentation: base.documentation,
        }
    }
}

/// Start Event
#[derive(Debug, Clone)]
pub struct StartEvent {
    pub base: ElementBase,
    pub event_definition: Option<EventDefinition>,
}

impl StartEvent {
    pub fn from_json(json: BpmnJsonStartEvent) -> Self {
        Self {
            base: ElementBase::from_json(json.base),
            event_definition: json.event_definition.map(EventDefinition::from_json),
        }
    }
}

/// End Event
#[derive(Debug, Clone)]
pub struct EndEvent {
    pub base: ElementBase,
    pub event_definition: Option<EventDefinition>,
}

impl EndEvent {
    pub fn from_json(json: BpmnJsonEndEvent) -> Self {
        Self {
            base: ElementBase::from_json(json.base),
            event_definition: json.event_definition.map(EventDefinition::from_json),
        }
    }
}

/// Intermediate Catch Event
#[derive(Debug, Clone)]
pub struct IntermediateCatchEvent {
    pub base: ElementBase,
    pub event_definition: Option<EventDefinition>,
}

impl IntermediateCatchEvent {
    pub fn from_json(json: BpmnJsonIntermediateCatchEvent) -> Self {
        Self {
            base: ElementBase::from_json(json.base),
            event_definition: json.event_definition.map(EventDefinition::from_json),
        }
    }
}

/// Intermediate Throw Event
#[derive(Debug, Clone)]
pub struct IntermediateThrowEvent {
    pub base: ElementBase,
    pub event_definition: Option<EventDefinition>,
}

impl IntermediateThrowEvent {
    pub fn from_json(json: BpmnJsonIntermediateThrowEvent) -> Self {
        Self {
            base: ElementBase::from_json(json.base),
            event_definition: json.event_definition.map(EventDefinition::from_json),
        }
    }
}

/// Service Task
#[derive(Debug, Clone)]
pub struct ServiceTask {
    pub base: ElementBase,
    pub implementation: Option<String>,
    pub operation_ref: Option<String>,
    pub io_mapping: IoMapping,
}

impl ServiceTask {
    pub fn from_json(json: BpmnJsonServiceTask) -> Self {
        Self {
            base: ElementBase::from_json(json.base),
            implementation: json.implementation,
            operation_ref: json.operation_ref,
            io_mapping: IoMapping::from_json(json.io_mapping),
        }
    }
}

/// User Task
#[derive(Debug, Clone)]
pub struct UserTask {
    pub base: ElementBase,
    pub assignment: Option<Assignment>,
    pub form_key: Option<String>,
}

impl UserTask {
    pub fn from_json(json: BpmnJsonUserTask) -> Self {
        Self {
            base: ElementBase::from_json(json.base),
            assignment: json.assignment.map(Assignment::from_json),
            form_key: json.form_key,
        }
    }
}

/// Script Task
#[derive(Debug, Clone)]
pub struct ScriptTask {
    pub base: ElementBase,
    pub script_format: Option<String>,
    pub script: Option<String>,
}

impl ScriptTask {
    pub fn from_json(json: BpmnJsonScriptTask) -> Self {
        Self {
            base: ElementBase::from_json(json.base),
            script_format: json.script_format,
            script: json.script,
        }
    }
}

/// Manual Task
#[derive(Debug, Clone)]
pub struct ManualTask {
    pub base: ElementBase,
}

impl ManualTask {
    pub fn from_json(json: BpmnJsonManualTask) -> Self {
        Self {
            base: ElementBase::from_json(json.base),
        }
    }
}

/// Exclusive Gateway
#[derive(Debug, Clone)]
pub struct ExclusiveGateway {
    pub base: ElementBase,
    pub default_flow: Option<String>,
}

impl ExclusiveGateway {
    pub fn from_json(json: BpmnJsonExclusiveGateway) -> Self {
        Self {
            base: ElementBase::from_json(json.base),
            default_flow: json.default_flow,
        }
    }
}

/// Parallel Gateway
#[derive(Debug, Clone)]
pub struct ParallelGateway {
    pub base: ElementBase,
}

impl ParallelGateway {
    pub fn from_json(json: BpmnJsonParallelGateway) -> Self {
        Self {
            base: ElementBase::from_json(json.base),
        }
    }
}

/// Inclusive Gateway
#[derive(Debug, Clone)]
pub struct InclusiveGateway {
    pub base: ElementBase,
    pub default_flow: Option<String>,
}

impl InclusiveGateway {
    pub fn from_json(json: BpmnJsonInclusiveGateway) -> Self {
        Self {
            base: ElementBase::from_json(json.base),
            default_flow: json.default_flow,
        }
    }
}

/// Sequence Flow
#[derive(Debug, Clone)]
pub struct SequenceFlow {
    pub id: String,
    pub name: Option<String>,
    pub source_ref: String,
    pub target_ref: String,
    pub condition_expression: Option<ConditionExpression>,
}

impl SequenceFlow {
    pub fn from_json(json: BpmnJsonSequenceFlow) -> Self {
        Self {
            id: json.base.id,
            name: json.base.name,
            source_ref: json.source_ref,
            target_ref: json.target_ref,
            condition_expression: json.condition_expression.map(ConditionExpression::from_json),
        }
    }
}

/// Event Definition
#[derive(Debug, Clone)]
pub enum EventDefinition {
    Message { message_ref: Option<String> },
    Timer { time_definition: Option<String> },
    Signal { signal_ref: Option<String> },
    Error { error_ref: Option<String> },
    Escalation { escalation_ref: Option<String> },
    Cancel,
    Compensation { activity_ref: Option<String> },
    Conditional { condition: Option<ConditionExpression> },
    Link { name: Option<String> },
    Terminate,
    None,
}

impl EventDefinition {
    pub fn from_json(json: BpmnJsonEventDefinition) -> Self {
        match json {
            BpmnJsonEventDefinition::Message { message_ref } => EventDefinition::Message { message_ref },
            BpmnJsonEventDefinition::Timer { time_definition } => EventDefinition::Timer { time_definition },
            BpmnJsonEventDefinition::Signal { signal_ref } => EventDefinition::Signal { signal_ref },
            BpmnJsonEventDefinition::Error { error_ref } => EventDefinition::Error { error_ref },
            BpmnJsonEventDefinition::Escalation { escalation_ref } => {
                EventDefinition::Escalation { escalation_ref }
            }
            BpmnJsonEventDefinition::Cancel => EventDefinition::Cancel,
            BpmnJsonEventDefinition::Compensation { activity_ref } => {
                EventDefinition::Compensation { activity_ref }
            }
            BpmnJsonEventDefinition::Conditional { condition } => {
                EventDefinition::Conditional {
                    condition: condition.map(ConditionExpression::from_json),
                }
            }
            BpmnJsonEventDefinition::Link { name } => EventDefinition::Link { name },
            BpmnJsonEventDefinition::Terminate => EventDefinition::Terminate,
            BpmnJsonEventDefinition::None => EventDefinition::None,
        }
    }
}

/// Condition Expression
#[derive(Debug, Clone)]
pub struct ConditionExpression {
    pub language: Option<String>,
    pub body: String,
}

impl ConditionExpression {
    pub fn from_json(json: BpmnJsonConditionExpression) -> Self {
        Self {
            language: json.language,
            body: json.body,
        }
    }
}

/// Input/Output Mapping
#[derive(Debug, Clone, Default)]
pub struct IoMapping {
    pub input_parameters: Vec<IoParameter>,
    pub output_parameters: Vec<IoParameter>,
}

impl IoMapping {
    pub fn from_json(json: BpmnJsonIoMapping) -> Self {
        Self {
            input_parameters: json.input_parameters.into_iter().map(IoParameter::from_json).collect(),
            output_parameters: json.output_parameters.into_iter().map(IoParameter::from_json).collect(),
        }
    }
}

/// Input/Output Parameter
#[derive(Debug, Clone)]
pub struct IoParameter {
    pub name: String,
    pub source: Option<String>,
    pub target: Option<String>,
    pub value: Option<String>,
}

impl IoParameter {
    pub fn from_json(json: BpmnJsonIoParameter) -> Self {
        Self {
            name: json.name,
            source: json.source,
            target: json.target,
            value: json.value,
        }
    }
}

/// Assignment
#[derive(Debug, Clone)]
pub struct Assignment {
    pub assignment_type: String,
    pub value: String,
}

impl Assignment {
    pub fn from_json(json: BpmnJsonAssignment) -> Self {
        Self {
            assignment_type: json.assignment_type,
            value: json.value,
        }
    }
}

/// Variable
#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub variable_type: Option<String>,
    pub default_value: Option<serde_json::Value>,
}

impl Variable {
    pub fn from_json(json: BpmnJsonVariable) -> Self {
        Self {
            name: json.name,
            variable_type: json.variable_type,
            default_value: json.default_value,
        }
    }
}


//! BPMN 2.0 XML Format
//!
//! XML representation of BPMN 2.0 process definitions.
//!
//! This module handles parsing and serialization of BPMN 2.0 XML format,
//! which is the standard format defined by OMG.

use crate::model::elements::*;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::collections::HashMap;
use std::io::Cursor;

/// BPMN 2.0 XML Namespaces
pub mod namespaces {
    pub const BPMN2: &str = "http://www.omg.org/spec/BPMN/20100524/MODEL";
    pub const BPMNDI: &str = "http://www.omg.org/spec/BPMN/20100524/DI";
    pub const DC: &str = "http://www.omg.org/spec/DD/20100524/DC";
    pub const DI: &str = "http://www.omg.org/spec/DD/20100524/DI";
}

/// Timer definition data collected during parsing
#[derive(Debug, Clone, Default)]
pub struct TimerData {
    pub time_date: Option<String>,
    pub time_duration: Option<String>,
    pub time_cycle: Option<String>,
}

/// Signal data collected during parsing
#[derive(Debug, Clone, Default)]
pub struct SignalData {
    pub signal_ref: Option<String>,
    pub has_signal_event: bool,
}

/// Escalation data collected during parsing
#[derive(Debug, Clone, Default)]
pub struct EscalationData {
    pub escalation_ref: Option<String>,
    pub has_escalation_event: bool,
}

/// Data association collected during parsing
#[derive(Debug, Clone, Default)]
pub struct DataAssociationData {
    pub source_ref: Option<String>,
    pub target_ref: Option<String>,
    pub transformation: Option<String>,
}

/// Call Activity data collected during parsing
#[derive(Debug, Clone, Default)]
pub struct CallActivityData {
    pub called_element: Option<String>,
    pub business_key: Option<String>,
    pub data_input_associations: Vec<DataAssociationData>,
    pub data_output_associations: Vec<DataAssociationData>,
    pub current_data_association: Option<DataAssociationData>,
    pub parsing_data_input: bool,
    pub parsing_data_output: bool,
}

/// Multi-Instance Loop Characteristics data collected during parsing
#[derive(Debug, Clone, Default)]
pub struct MultiInstanceData {
    pub is_parallel: Option<bool>,
    pub loop_cardinality: Option<i32>,
    pub completion_condition: Option<String>,
    pub behavior: Option<String>,
}

/// Extension Elements data collected during parsing
#[derive(Debug, Clone, Default)]
pub struct ExtensionElementsData {
    pub properties: HashMap<String, String>,
    pub current_key: Option<String>,
}

/// Helper function to extract attributes from XML element
fn extract_attributes(e: &BytesStart) -> HashMap<String, String> {
    let mut attrs = HashMap::new();
    for attr in e.attributes() {
        if let Ok(attr) = attr {
            let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
            let value = String::from_utf8_lossy(&attr.value).to_string();
            attrs.insert(key, value);
        }
    }
    attrs
}

/// Helper function to check if element name matches (handles namespaces)
fn matches_element_name(name: &[u8], patterns: &[&[u8]]) -> bool {
    patterns.iter().any(|&pattern| name == pattern)
}

/// Parse BPMN XML to ProcessDefinition
///
/// # Arguments
/// * `xml` - XML string containing BPMN 2.0 definition
///
/// # Returns
/// * `Ok(ProcessDefinition)` - Parsed process definition
/// * `Err(ParseError)` - Parse error
pub fn parse_bpmn_xml(xml: &str) -> Result<ProcessDefinition, crate::model::format::ParseError> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut process_id: Option<String> = None;
    let mut process_name: Option<String> = None;
    let mut is_executable: bool = true;
    let mut elements: HashMap<String, ProcessElement> = HashMap::new();
    let mut flows: HashMap<String, SequenceFlow> = HashMap::new();
    let variables: HashMap<String, Variable> = HashMap::new();

    // State for nested element parsing
    let mut current_event_id: Option<String> = None;
    let mut current_event_name: Option<String> = None;
    let mut timer_data: TimerData = TimerData::default();
    let mut signal_data: SignalData = SignalData::default();
    let mut escalation_data: EscalationData = EscalationData::default();
    let mut call_activity_data: CallActivityData = CallActivityData::default();
    let mut current_call_activity_id: Option<String> = None;
    let mut current_call_activity_name: Option<String> = None;
    let mut multi_instance_data: MultiInstanceData = MultiInstanceData::default();
    let mut extension_elements_data: ExtensionElementsData = ExtensionElementsData::default();
    let mut current_task_id: Option<String> = None;
    let mut current_task_type: Option<String> = None;

    // Stack to track nested elements
    let mut element_stack: Vec<String> = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = e.name();
                let attrs = extract_attributes(&e);

                // Process element
                if matches_element_name(name.as_ref(), &[b"bpmn2:process", b"bpmn:process", b"process"]) {
                    if let Some(id) = attrs.get("id") {
                        process_id = Some(id.clone());
                    }
                    if let Some(name_attr) = attrs.get("name") {
                        process_name = Some(name_attr.clone());
                    }
                    if let Some(exec) = attrs.get("isExecutable") {
                        is_executable = exec == "true";
                    }
                }
                // Start Event
                else if matches_element_name(name.as_ref(), &[b"bpmn2:startEvent", b"bpmn:startEvent", b"startEvent"]) {
                    current_event_id = attrs.get("id").cloned();
                    current_event_name = attrs.get("name").cloned();
                    timer_data = TimerData::default();
                    element_stack.push("startEvent".to_string());
                }
                // End Event
                else if matches_element_name(name.as_ref(), &[b"bpmn2:endEvent", b"bpmn:endEvent", b"endEvent"]) {
                    current_event_id = attrs.get("id").cloned();
                    current_event_name = attrs.get("name").cloned();
                    timer_data = TimerData::default();
                    element_stack.push("endEvent".to_string());
                }
                // Intermediate Catch Event
                else if matches_element_name(name.as_ref(), &[b"bpmn2:intermediateCatchEvent", b"bpmn:intermediateCatchEvent", b"intermediateCatchEvent"]) {
                    current_event_id = attrs.get("id").cloned();
                    current_event_name = attrs.get("name").cloned();
                    timer_data = TimerData::default();
                    signal_data = SignalData::default();
                    escalation_data = EscalationData::default();
                    element_stack.push("intermediateCatchEvent".to_string());
                }
                // Intermediate Throw Event
                else if matches_element_name(name.as_ref(), &[b"bpmn2:intermediateThrowEvent", b"bpmn:intermediateThrowEvent", b"intermediateThrowEvent"]) {
                    current_event_id = attrs.get("id").cloned();
                    current_event_name = attrs.get("name").cloned();
                    signal_data = SignalData::default();
                    escalation_data = EscalationData::default();
                    element_stack.push("intermediateThrowEvent".to_string());
                }
                // Timer Event Definition
                else if matches_element_name(name.as_ref(), &[b"bpmn2:timerEventDefinition", b"bpmn:timerEventDefinition", b"timerEventDefinition"]) {
                    timer_data = TimerData::default();
                    element_stack.push("timerEventDefinition".to_string());
                }
                // Time Date
                else if matches_element_name(name.as_ref(), &[b"bpmn2:timeDate", b"bpmn:timeDate", b"timeDate"]) {
                    element_stack.push("timeDate".to_string());
                }
                // Time Duration
                else if matches_element_name(name.as_ref(), &[b"bpmn2:timeDuration", b"bpmn:timeDuration", b"timeDuration"]) {
                    element_stack.push("timeDuration".to_string());
                }
                // Time Cycle
                else if matches_element_name(name.as_ref(), &[b"bpmn2:timeCycle", b"bpmn:timeCycle", b"timeCycle"]) {
                    element_stack.push("timeCycle".to_string());
                }
                // Signal Event Definition
                else if matches_element_name(name.as_ref(), &[b"bpmn2:signalEventDefinition", b"bpmn:signalEventDefinition", b"signalEventDefinition"]) {
                    signal_data.has_signal_event = true;
                    if let Some(signal_ref) = attrs.get("signalRef") {
                        signal_data.signal_ref = Some(signal_ref.clone());
                    }
                    element_stack.push("signalEventDefinition".to_string());
                }
                // Escalation Event Definition
                else if matches_element_name(name.as_ref(), &[b"bpmn2:escalationEventDefinition", b"bpmn:escalationEventDefinition", b"escalationEventDefinition"]) {
                    escalation_data.has_escalation_event = true;
                    if let Some(escalation_ref) = attrs.get("escalationRef") {
                        escalation_data.escalation_ref = Some(escalation_ref.clone());
                    }
                    element_stack.push("escalationEventDefinition".to_string());
                }
                // Service Task
                else if matches_element_name(name.as_ref(), &[b"bpmn2:serviceTask", b"bpmn:serviceTask", b"serviceTask"]) {
                    current_task_id = attrs.get("id").cloned();
                    current_task_type = Some("serviceTask".to_string());
                    multi_instance_data = MultiInstanceData::default();
                    if let Some(id) = attrs.get("id") {
                        elements.insert(
                            id.clone(),
                            ProcessElement::ServiceTask(ServiceTask {
                                base: ElementBase {
                                    id: id.clone(),
                                    name: attrs.get("name").cloned(),
                                    documentation: None,
                                    extension_elements: None,
                                },
                                implementation: attrs.get("implementation").cloned(),
                                operation_ref: attrs.get("operationRef").cloned(),
                                io_mapping: Default::default(),
                                loop_characteristics: None,
                            }),
                        );
                    }
                    element_stack.push("serviceTask".to_string());
                }
                // User Task
                else if matches_element_name(name.as_ref(), &[b"bpmn2:userTask", b"bpmn:userTask", b"userTask"]) {
                    current_task_id = attrs.get("id").cloned();
                    current_task_type = Some("userTask".to_string());
                    multi_instance_data = MultiInstanceData::default();
                    if let Some(id) = attrs.get("id") {
                        elements.insert(
                            id.clone(),
                            ProcessElement::UserTask(UserTask {
                                base: ElementBase {
                                    id: id.clone(),
                                    name: attrs.get("name").cloned(),
                                    documentation: None,
                                    extension_elements: None,
                                },
                                assignment: None,
                                form_key: attrs.get("formKey").cloned(),
                                loop_characteristics: None,
                            }),
                        );
                    }
                    element_stack.push("userTask".to_string());
                }
                // Script Task
                else if matches_element_name(name.as_ref(), &[b"bpmn2:scriptTask", b"bpmn:scriptTask", b"scriptTask"]) {
                    current_task_id = attrs.get("id").cloned();
                    current_task_type = Some("scriptTask".to_string());
                    multi_instance_data = MultiInstanceData::default();
                    if let Some(id) = attrs.get("id") {
                        elements.insert(
                            id.clone(),
                            ProcessElement::ScriptTask(ScriptTask {
                                base: ElementBase {
                                    id: id.clone(),
                                    name: attrs.get("name").cloned(),
                                    documentation: None,
                                    extension_elements: None,
                                },
                                script_format: attrs.get("scriptFormat").cloned(),
                                script: None,
                                loop_characteristics: None,
                            }),
                        );
                    }
                    element_stack.push("scriptTask".to_string());
                }
                // Manual Task
                else if matches_element_name(name.as_ref(), &[b"bpmn2:manualTask", b"bpmn:manualTask", b"manualTask"]) {
                    if let Some(id) = attrs.get("id") {
                        elements.insert(
                            id.clone(),
                            ProcessElement::ManualTask(ManualTask {
                                base: ElementBase {
                                    id: id.clone(),
                                    name: attrs.get("name").cloned(),
                                    documentation: None,
                                    extension_elements: None,
                                },
                            }),
                        );
                    }
                }
                // Exclusive Gateway
                else if matches_element_name(name.as_ref(), &[b"bpmn2:exclusiveGateway", b"bpmn:exclusiveGateway", b"exclusiveGateway"]) {
                    if let Some(id) = attrs.get("id") {
                        elements.insert(
                            id.clone(),
                            ProcessElement::ExclusiveGateway(ExclusiveGateway {
                                base: ElementBase {
                                    id: id.clone(),
                                    name: attrs.get("name").cloned(),
                                    documentation: None,
                                    extension_elements: None,
                                },
                                default_flow: attrs.get("default").cloned(),
                            }),
                        );
                    }
                }
                // Parallel Gateway
                else if matches_element_name(name.as_ref(), &[b"bpmn2:parallelGateway", b"bpmn:parallelGateway", b"parallelGateway"]) {
                    if let Some(id) = attrs.get("id") {
                        elements.insert(
                            id.clone(),
                            ProcessElement::ParallelGateway(ParallelGateway {
                                base: ElementBase {
                                    id: id.clone(),
                                    name: attrs.get("name").cloned(),
                                    documentation: None,
                                    extension_elements: None,
                                },
                                default_flow: attrs.get("default").cloned(),
                                gateway_direction: crate::engine::context::GatewayDirection::Unknown,
                            }),
                        );
                    }
                }
                // Inclusive Gateway
                else if matches_element_name(name.as_ref(), &[b"bpmn2:inclusiveGateway", b"bpmn:inclusiveGateway", b"inclusiveGateway"]) {
                    if let Some(id) = attrs.get("id") {
                        elements.insert(
                            id.clone(),
                            ProcessElement::InclusiveGateway(InclusiveGateway {
                                base: ElementBase {
                                    id: id.clone(),
                                    name: attrs.get("name").cloned(),
                                    documentation: None,
                                    extension_elements: None,
                                },
                                default_flow: attrs.get("default").cloned(),
                            }),
                        );
                    }
                }
                // Data Object
                else if matches_element_name(name.as_ref(), &[b"bpmn2:dataObject", b"bpmn:dataObject", b"dataObject"]) {
                    if let Some(id) = attrs.get("id") {
                        elements.insert(
                            id.clone(),
                            ProcessElement::DataObject(DataObject {
                                base: ElementBase {
                                    id: id.clone(),
                                    name: attrs.get("name").cloned(),
                                    documentation: None,
                                    extension_elements: None,
                                },
                                data_state: attrs.get("dataState").cloned(),
                            }),
                        );
                    }
                }
                // Data Object Reference
                else if matches_element_name(name.as_ref(), &[b"bpmn2:dataObjectReference", b"bpmn:dataObjectReference", b"dataObjectReference"]) {
                    if let Some(id) = attrs.get("id") {
                        elements.insert(
                            id.clone(),
                            ProcessElement::DataObjectReference(DataObjectReference {
                                base: ElementBase {
                                    id: id.clone(),
                                    name: attrs.get("name").cloned(),
                                    documentation: None,
                                    extension_elements: None,
                                },
                                data_object_ref: attrs.get("dataObjectRef").cloned(),
                            }),
                        );
                    }
                }
                // Data Input
                else if matches_element_name(name.as_ref(), &[b"bpmn2:dataInput", b"bpmn:dataInput", b"dataInput"]) {
                    if let Some(id) = attrs.get("id") {
                        elements.insert(
                            id.clone(),
                            ProcessElement::DataInput(DataInput {
                                base: ElementBase {
                                    id: id.clone(),
                                    name: attrs.get("name").cloned(),
                                    documentation: None,
                                    extension_elements: None,
                                },
                                input_set: attrs.get("inputSet").cloned(),
                            }),
                        );
                    }
                }
                // Data Output
                else if matches_element_name(name.as_ref(), &[b"bpmn2:dataOutput", b"bpmn:dataOutput", b"dataOutput"]) {
                    if let Some(id) = attrs.get("id") {
                        elements.insert(
                            id.clone(),
                            ProcessElement::DataOutput(DataOutput {
                                base: ElementBase {
                                    id: id.clone(),
                                    name: attrs.get("name").cloned(),
                                    documentation: None,
                                    extension_elements: None,
                                },
                                output_set: attrs.get("outputSet").cloned(),
                            }),
                        );
                    }
                }
                // Call Activity
                else if matches_element_name(name.as_ref(), &[b"bpmn2:callActivity", b"bpmn:callActivity", b"callActivity"]) {
                    current_call_activity_id = attrs.get("id").cloned();
                    current_call_activity_name = attrs.get("name").cloned();
                    call_activity_data = CallActivityData {
                        called_element: attrs.get("calledElement").cloned(),
                        business_key: attrs.get("businessKey").cloned(),
                        ..Default::default()
                    };
                    element_stack.push("callActivity".to_string());
                }
                // Data Input Association
                else if matches_element_name(name.as_ref(), &[b"bpmn2:dataInputAssociation", b"bpmn:dataInputAssociation", b"dataInputAssociation"]) {
                    if !element_stack.contains(&"callActivity".to_string()) {
                        element_stack.push("dataInputAssociation".to_string());
                    }
                    call_activity_data.parsing_data_input = true;
                    call_activity_data.current_data_association = Some(DataAssociationData::default());
                }
                // Data Output Association
                else if matches_element_name(name.as_ref(), &[b"bpmn2:dataOutputAssociation", b"bpmn:dataOutputAssociation", b"dataOutputAssociation"]) {
                    if !element_stack.contains(&"callActivity".to_string()) {
                        element_stack.push("dataOutputAssociation".to_string());
                    }
                    call_activity_data.parsing_data_output = true;
                    call_activity_data.current_data_association = Some(DataAssociationData::default());
                }
                // Source Ref (for data association)
                else if matches_element_name(name.as_ref(), &[b"bpmn2:sourceRef", b"bpmn:sourceRef", b"sourceRef"]) {
                    element_stack.push("sourceRef".to_string());
                }
                // Target Ref (for data association)
                else if matches_element_name(name.as_ref(), &[b"bpmn2:targetRef", b"bpmn:targetRef", b"targetRef"]) {
                    element_stack.push("targetRef".to_string());
                }
                // Sequence Flow
                else if matches_element_name(name.as_ref(), &[b"bpmn2:sequenceFlow", b"bpmn:sequenceFlow", b"sequenceFlow"]) {
                    if let (Some(id), Some(source), Some(target)) = (
                        attrs.get("id"),
                        attrs.get("sourceRef"),
                        attrs.get("targetRef"),
                    ) {
                        flows.insert(
                            id.clone(),
                            SequenceFlow {
                                id: id.clone(),
                                name: attrs.get("name").cloned(),
                                source_ref: source.clone(),
                                target_ref: target.clone(),
                                condition_expression: None,
                            },
                        );
                    }
                }
                // Multi-Instance Loop Characteristics
                else if matches_element_name(name.as_ref(), &[b"bpmn2:multiInstanceLoopCharacteristics", b"bpmn:multiInstanceLoopCharacteristics", b"multiInstanceLoopCharacteristics"]) {
                    multi_instance_data = MultiInstanceData {
                        is_parallel: attrs.get("isParallel").map(|v| v == "true"),
                        loop_cardinality: attrs.get("loopCardinality").and_then(|v| v.parse().ok()),
                        completion_condition: attrs.get("completionCondition").cloned(),
                        behavior: attrs.get("behavior").cloned(),
                    };
                    element_stack.push("multiInstanceLoopCharacteristics".to_string());
                }
                // Loop Cardinality
                else if matches_element_name(name.as_ref(), &[b"bpmn2:loopCardinality", b"bpmn:loopCardinality", b"loopCardinality"]) {
                    element_stack.push("loopCardinality".to_string());
                }
                // Completion Condition
                else if matches_element_name(name.as_ref(), &[b"bpmn2:completionCondition", b"bpmn:completionCondition", b"completionCondition"]) {
                    element_stack.push("completionCondition".to_string());
                }
                // Extension Elements
                else if matches_element_name(name.as_ref(), &[b"bpmn2:extensionElements", b"bpmn:extensionElements", b"extensionElements"]) {
                    extension_elements_data = ExtensionElementsData::default();
                    element_stack.push("extensionElements".to_string());
                }
                // Any child element under extensionElements (custom extension property)
                else if element_stack.contains(&"extensionElements".to_string()) {
                    // This is a custom extension element (e.g., custom:formKey)
                    // Extract the local name (without namespace prefix)
                    let name_str = String::from_utf8_lossy(name.as_ref()).to_string();
                    // Remove namespace prefix if present (e.g., "custom:formKey" -> "formKey")
                    let local_name = if name_str.contains(':') {
                        name_str.split(':').last().unwrap_or(&name_str).to_string()
                    } else {
                        name_str
                    };
                    extension_elements_data.current_key = Some(local_name);
                }
            }
            Ok(Event::Empty(e)) => {
                // Handle self-closing elements (empty elements like <element />)
                let name = e.name();
                let attrs = extract_attributes(&e);

                // Signal Event Definition (empty)
                if matches_element_name(name.as_ref(), &[b"bpmn2:signalEventDefinition", b"bpmn:signalEventDefinition", b"signalEventDefinition"]) {
                    signal_data.has_signal_event = true;
                    if let Some(signal_ref) = attrs.get("signalRef") {
                        signal_data.signal_ref = Some(signal_ref.clone());
                    }
                }
                // Service Task
                if matches_element_name(name.as_ref(), &[b"bpmn2:serviceTask", b"bpmn:serviceTask", b"serviceTask"]) {
                    if let Some(id) = attrs.get("id") {
                        elements.insert(
                            id.clone(),
                            ProcessElement::ServiceTask(ServiceTask {
                                base: ElementBase {
                                    id: id.clone(),
                                    name: attrs.get("name").cloned(),
                                    documentation: None,
                                    extension_elements: None,
                                },
                                implementation: attrs.get("implementation").cloned(),
                                operation_ref: attrs.get("operationRef").cloned(),
                                io_mapping: Default::default(),
                                loop_characteristics: None,
                            }),
                        );
                    }
                }
                // User Task
                else if matches_element_name(name.as_ref(), &[b"bpmn2:userTask", b"bpmn:userTask", b"userTask"]) {
                    current_task_id = attrs.get("id").cloned();
                    current_task_type = Some("userTask".to_string());
                    multi_instance_data = MultiInstanceData::default();
                    if let Some(id) = attrs.get("id") {
                        elements.insert(
                            id.clone(),
                            ProcessElement::UserTask(UserTask {
                                base: ElementBase {
                                    id: id.clone(),
                                    name: attrs.get("name").cloned(),
                                    documentation: None,
                                    extension_elements: None,
                                },
                                assignment: None,
                                form_key: attrs.get("formKey").cloned(),
                                loop_characteristics: None,
                            }),
                        );
                    }
                    element_stack.push("userTask".to_string());
                }
                // Script Task
                else if matches_element_name(name.as_ref(), &[b"bpmn2:scriptTask", b"bpmn:scriptTask", b"scriptTask"]) {
                    current_task_id = attrs.get("id").cloned();
                    current_task_type = Some("scriptTask".to_string());
                    multi_instance_data = MultiInstanceData::default();
                    if let Some(id) = attrs.get("id") {
                        elements.insert(
                            id.clone(),
                            ProcessElement::ScriptTask(ScriptTask {
                                base: ElementBase {
                                    id: id.clone(),
                                    name: attrs.get("name").cloned(),
                                    documentation: None,
                                    extension_elements: None,
                                },
                                script_format: attrs.get("scriptFormat").cloned(),
                                script: None,
                                loop_characteristics: None,
                            }),
                        );
                    }
                    element_stack.push("scriptTask".to_string());
                }
                // Manual Task
                else if matches_element_name(name.as_ref(), &[b"bpmn2:manualTask", b"bpmn:manualTask", b"manualTask"]) {
                    if let Some(id) = attrs.get("id") {
                        elements.insert(
                            id.clone(),
                            ProcessElement::ManualTask(ManualTask {
                                base: ElementBase {
                                    id: id.clone(),
                                    name: attrs.get("name").cloned(),
                                    documentation: None,
                                    extension_elements: None,
                                },
                            }),
                        );
                    }
                }
                // Exclusive Gateway
                else if matches_element_name(name.as_ref(), &[b"bpmn2:exclusiveGateway", b"bpmn:exclusiveGateway", b"exclusiveGateway"]) {
                    if let Some(id) = attrs.get("id") {
                        elements.insert(
                            id.clone(),
                            ProcessElement::ExclusiveGateway(ExclusiveGateway {
                                base: ElementBase {
                                    id: id.clone(),
                                    name: attrs.get("name").cloned(),
                                    documentation: None,
                                    extension_elements: None,
                                },
                                default_flow: attrs.get("default").cloned(),
                            }),
                        );
                    }
                }
                // Parallel Gateway
                else if matches_element_name(name.as_ref(), &[b"bpmn2:parallelGateway", b"bpmn:parallelGateway", b"parallelGateway"]) {
                    if let Some(id) = attrs.get("id") {
                        elements.insert(
                            id.clone(),
                            ProcessElement::ParallelGateway(ParallelGateway {
                                base: ElementBase {
                                    id: id.clone(),
                                    name: attrs.get("name").cloned(),
                                    documentation: None,
                                    extension_elements: None,
                                },
                                default_flow: attrs.get("default").cloned(),
                                gateway_direction: crate::engine::context::GatewayDirection::Unknown,
                            }),
                        );
                    }
                }
                // Inclusive Gateway (empty - no children)
                else if matches_element_name(name.as_ref(), &[b"bpmn2:inclusiveGateway", b"bpmn:inclusiveGateway", b"inclusiveGateway"]) {
                    if let Some(id) = attrs.get("id") {
                        elements.insert(
                            id.clone(),
                            ProcessElement::InclusiveGateway(InclusiveGateway {
                                base: ElementBase {
                                    id: id.clone(),
                                    name: attrs.get("name").cloned(),
                                    documentation: None,
                                    extension_elements: None,
                                },
                                default_flow: attrs.get("default").cloned(),
                            }),
                        );
                    }
                }
                // Data Object (empty - no children)
                else if matches_element_name(name.as_ref(), &[b"bpmn2:dataObject", b"bpmn:dataObject", b"dataObject"]) {
                    if let Some(id) = attrs.get("id") {
                        elements.insert(
                            id.clone(),
                            ProcessElement::DataObject(DataObject {
                                base: ElementBase {
                                    id: id.clone(),
                                    name: attrs.get("name").cloned(),
                                    documentation: None,
                                    extension_elements: None,
                                },
                                data_state: attrs.get("dataState").cloned(),
                            }),
                        );
                    }
                }
                // Data Object Reference (empty - no children)
                else if matches_element_name(name.as_ref(), &[b"bpmn2:dataObjectReference", b"bpmn:dataObjectReference", b"dataObjectReference"]) {
                    if let Some(id) = attrs.get("id") {
                        elements.insert(
                            id.clone(),
                            ProcessElement::DataObjectReference(DataObjectReference {
                                base: ElementBase {
                                    id: id.clone(),
                                    name: attrs.get("name").cloned(),
                                    documentation: None,
                                    extension_elements: None,
                                },
                                data_object_ref: attrs.get("dataObjectRef").cloned(),
                            }),
                        );
                    }
                }
                // Data Input (empty - no children)
                else if matches_element_name(name.as_ref(), &[b"bpmn2:dataInput", b"bpmn:dataInput", b"dataInput"]) {
                    if let Some(id) = attrs.get("id") {
                        elements.insert(
                            id.clone(),
                            ProcessElement::DataInput(DataInput {
                                base: ElementBase {
                                    id: id.clone(),
                                    name: attrs.get("name").cloned(),
                                    documentation: None,
                                    extension_elements: None,
                                },
                                input_set: attrs.get("inputSet").cloned(),
                            }),
                        );
                    }
                }
                // Data Output (empty - no children)
                else if matches_element_name(name.as_ref(), &[b"bpmn2:dataOutput", b"bpmn:dataOutput", b"dataOutput"]) {
                    if let Some(id) = attrs.get("id") {
                        elements.insert(
                            id.clone(),
                            ProcessElement::DataOutput(DataOutput {
                                base: ElementBase {
                                    id: id.clone(),
                                    name: attrs.get("name").cloned(),
                                    documentation: None,
                                    extension_elements: None,
                                },
                                output_set: attrs.get("outputSet").cloned(),
                            }),
                        );
                    }
                }
                // Call Activity (empty - no children)
                else if matches_element_name(name.as_ref(), &[b"bpmn2:callActivity", b"bpmn:callActivity", b"callActivity"]) {
                    if let Some(id) = attrs.get("id") {
                        elements.insert(
                            id.clone(),
                            ProcessElement::CallActivity(CallActivity {
                                base: ElementBase {
                                    id: id.clone(),
                                    name: attrs.get("name").cloned(),
                                    documentation: None,
                                    extension_elements: None,
                                },
                                called_element: attrs.get("calledElement").cloned(),
                                business_key: attrs.get("businessKey").cloned(),
                                data_input_associations: Vec::new(),
                                data_output_associations: Vec::new(),
                            }),
                        );
                    }
                }
                // Start Event (empty - no children)
                else if matches_element_name(name.as_ref(), &[b"bpmn2:startEvent", b"bpmn:startEvent", b"startEvent"]) {
                    if let Some(event_id) = attrs.get("id").cloned() {
                        elements.insert(
                            event_id.clone(),
                            ProcessElement::StartEvent(StartEvent {
                                base: ElementBase {
                                    id: event_id,
                                    name: attrs.get("name").cloned(),
                                    documentation: None,
                                    extension_elements: None,
                                },
                                event_definition: None,
                            }),
                        );
                    }
                }
                // End Event (empty - no children)
                else if matches_element_name(name.as_ref(), &[b"bpmn2:endEvent", b"bpmn:endEvent", b"endEvent"]) {
                    if let Some(event_id) = attrs.get("id").cloned() {
                        elements.insert(
                            event_id.clone(),
                            ProcessElement::EndEvent(EndEvent {
                                base: ElementBase {
                                    id: event_id,
                                    name: attrs.get("name").cloned(),
                                    documentation: None,
                                    extension_elements: None,
                                },
                                event_definition: None,
                            }),
                        );
                    }
                }
            }
            Ok(Event::Text(e)) => {
                let text = String::from_utf8_lossy(&e).trim().to_string();
                if !text.is_empty() {
                    // Check if we're inside a timer child element
                    if element_stack.last().map(|s| s.as_str()) == Some("timeDate") {
                        timer_data.time_date = Some(text);
                    } else if element_stack.last().map(|s| s.as_str()) == Some("timeDuration") {
                        timer_data.time_duration = Some(text);
                    } else if element_stack.last().map(|s| s.as_str()) == Some("timeCycle") {
                        timer_data.time_cycle = Some(text);
                    }
                    // Handle sourceRef in data association
                    else if element_stack.last().map(|s| s.as_str()) == Some("sourceRef") {
                        if let Some(ref mut current) = call_activity_data.current_data_association {
                            current.source_ref = Some(text);
                        }
                    }
                    // Handle targetRef in data association
                    else if element_stack.last().map(|s| s.as_str()) == Some("targetRef") {
                        if let Some(ref mut current) = call_activity_data.current_data_association {
                            current.target_ref = Some(text);
                        }
                    }
                    // Handle loopCardinality text
                    else if element_stack.last().map(|s| s.as_str()) == Some("loopCardinality") {
                        if let Ok(cardinality) = text.parse::<i32>() {
                            multi_instance_data.loop_cardinality = Some(cardinality);
                        }
                    }
                    // Handle completionCondition text
                    else if element_stack.last().map(|s| s.as_str()) == Some("completionCondition") {
                        multi_instance_data.completion_condition = Some(text);
                    }
                    // Handle extension element text (custom property value)
                    else if element_stack.contains(&"extensionElements".to_string()) {
                        if let Some(ref key) = extension_elements_data.current_key {
                            extension_elements_data.properties.insert(key.clone(), text);
                        }
                    }
                }
            }
            Ok(Event::End(e)) => {
                let name = e.name();

                // Handle closing of timer child elements
                if matches_element_name(name.as_ref(), &[b"bpmn2:timeDate", b"bpmn:timeDate", b"timeDate"]) ||
                   matches_element_name(name.as_ref(), &[b"bpmn2:timeDuration", b"bpmn:timeDuration", b"timeDuration"]) ||
                   matches_element_name(name.as_ref(), &[b"bpmn2:timeCycle", b"bpmn:timeCycle", b"timeCycle"]) {
                    element_stack.pop();
                }
                // Handle closing of timerEventDefinition
                else if matches_element_name(name.as_ref(), &[b"bpmn2:timerEventDefinition", b"bpmn:timerEventDefinition", b"timerEventDefinition"]) {
                    element_stack.pop();
                }
                // Handle closing of signalEventDefinition
                else if matches_element_name(name.as_ref(), &[b"bpmn2:signalEventDefinition", b"bpmn:signalEventDefinition", b"signalEventDefinition"]) {
                    element_stack.pop();
                }
                // Handle closing of Start Event
                else if matches_element_name(name.as_ref(), &[b"bpmn2:startEvent", b"bpmn:startEvent", b"startEvent"]) {
                    if let Some(event_id) = current_event_id.take() {
                        let event_def = if timer_data.time_date.is_some() || timer_data.time_duration.is_some() || timer_data.time_cycle.is_some() {
                            timer_data = TimerData::default();
                            Some(EventDefinition::Timer {
                                time_definition: timer_data.time_duration.clone().or(timer_data.time_date.clone()).or(timer_data.time_cycle.clone()),
                                timer_def: Some(TimerDefinition::from_timer_data(&timer_data)),
                            })
                        } else if signal_data.has_signal_event {
                            Some(EventDefinition::Signal { signal_ref: signal_data.signal_ref.clone() })
                        } else {
                            None
                        };

                        elements.insert(
                            event_id.clone(),
                            ProcessElement::StartEvent(StartEvent {
                                base: ElementBase {
                                    id: event_id,
                                    name: current_event_name.take(),
                                    documentation: None,
                                    extension_elements: None,
                                },
                                event_definition: event_def,
                            }),
                        );
                    }
                    timer_data = TimerData::default();
                    signal_data = SignalData::default();
                    element_stack.clear();
                }
                // Handle closing of End Event
                else if matches_element_name(name.as_ref(), &[b"bpmn2:endEvent", b"bpmn:endEvent", b"endEvent"]) {
                    if let Some(event_id) = current_event_id.take() {
                        let event_def = if timer_data.time_date.is_some() || timer_data.time_duration.is_some() || timer_data.time_cycle.is_some() {
                            Some(EventDefinition::Timer {
                                time_definition: timer_data.time_duration.clone().or(timer_data.time_date.clone()).or(timer_data.time_cycle.clone()),
                                timer_def: Some(TimerDefinition::from_timer_data(&timer_data)),
                            })
                        } else if signal_data.has_signal_event {
                            Some(EventDefinition::Signal { signal_ref: signal_data.signal_ref.clone() })
                        } else {
                            None
                        };

                        elements.insert(
                            event_id.clone(),
                            ProcessElement::EndEvent(EndEvent {
                                base: ElementBase {
                                    id: event_id,
                                    name: current_event_name.take(),
                                    documentation: None,
                                    extension_elements: None,
                                },
                                event_definition: event_def,
                            }),
                        );
                    }
                    timer_data = TimerData::default();
                    signal_data = SignalData::default();
                    element_stack.clear();
                }
                // Handle closing of Intermediate Catch Event
                else if matches_element_name(name.as_ref(), &[b"bpmn2:intermediateCatchEvent", b"bpmn:intermediateCatchEvent", b"intermediateCatchEvent"]) {
                    if let Some(event_id) = current_event_id.take() {
                        let event_def = if timer_data.time_date.is_some() || timer_data.time_duration.is_some() || timer_data.time_cycle.is_some() {
                            Some(EventDefinition::Timer {
                                time_definition: timer_data.time_duration.clone().or(timer_data.time_date.clone()).or(timer_data.time_cycle.clone()),
                                timer_def: Some(TimerDefinition::from_timer_data(&timer_data)),
                            })
                        } else if signal_data.has_signal_event {
                            Some(EventDefinition::Signal { signal_ref: signal_data.signal_ref.clone() })
                        } else if escalation_data.has_escalation_event {
                            Some(EventDefinition::Escalation { escalation_ref: escalation_data.escalation_ref.clone() })
                        } else {
                            None
                        };

                        elements.insert(
                            event_id.clone(),
                            ProcessElement::IntermediateCatchEvent(IntermediateCatchEvent {
                                base: ElementBase {
                                    id: event_id,
                                    name: current_event_name.take(),
                                    documentation: None,
                                    extension_elements: None,
                                },
                                event_definition: event_def,
                            }),
                        );
                    }
                    timer_data = TimerData::default();
                    signal_data = SignalData::default();
                    escalation_data = EscalationData::default();
                    element_stack.clear();
                }
                // Handle closing of Intermediate Throw Event
                else if matches_element_name(name.as_ref(), &[b"bpmn2:intermediateThrowEvent", b"bpmn:intermediateThrowEvent", b"intermediateThrowEvent"]) {
                    if let Some(event_id) = current_event_id.take() {
                        // Check if signalEventDefinition or escalationEventDefinition was encountered
                        let event_def = if signal_data.has_signal_event {
                            Some(EventDefinition::Signal { signal_ref: signal_data.signal_ref.clone() })
                        } else if escalation_data.has_escalation_event {
                            Some(EventDefinition::Escalation { escalation_ref: escalation_data.escalation_ref.clone() })
                        } else {
                            None
                        };

                        elements.insert(
                            event_id.clone(),
                            ProcessElement::IntermediateThrowEvent(IntermediateThrowEvent {
                                base: ElementBase {
                                    id: event_id,
                                    name: current_event_name.take(),
                                    documentation: None,
                                    extension_elements: None,
                                },
                                event_definition: event_def,
                            }),
                        );
                    }
                    timer_data = TimerData::default();
                    signal_data = SignalData::default();
                    element_stack.clear();
                }
                // Handle closing of sourceRef
                else if matches_element_name(name.as_ref(), &[b"bpmn2:sourceRef", b"bpmn:sourceRef", b"sourceRef"]) {
                    element_stack.pop();
                }
                // Handle closing of targetRef
                else if matches_element_name(name.as_ref(), &[b"bpmn2:targetRef", b"bpmn:targetRef", b"targetRef"]) {
                    element_stack.pop();
                }
                // Handle closing of dataInputAssociation
                else if matches_element_name(name.as_ref(), &[b"bpmn2:dataInputAssociation", b"bpmn:dataInputAssociation", b"dataInputAssociation"]) {
                    if let Some(current) = call_activity_data.current_data_association.take() {
                        call_activity_data.data_input_associations.push(current);
                    }
                    call_activity_data.parsing_data_input = false;
                    element_stack.retain(|e| e != "dataInputAssociation");
                }
                // Handle closing of dataOutputAssociation
                else if matches_element_name(name.as_ref(), &[b"bpmn2:dataOutputAssociation", b"bpmn:dataOutputAssociation", b"dataOutputAssociation"]) {
                    if let Some(current) = call_activity_data.current_data_association.take() {
                        call_activity_data.data_output_associations.push(current);
                    }
                    call_activity_data.parsing_data_output = false;
                    element_stack.retain(|e| e != "dataOutputAssociation");
                }
                // Handle closing of loopCardinality
                else if matches_element_name(name.as_ref(), &[b"bpmn2:loopCardinality", b"bpmn:loopCardinality", b"loopCardinality"]) {
                    element_stack.pop();
                }
                // Handle closing of completionCondition
                else if matches_element_name(name.as_ref(), &[b"bpmn2:completionCondition", b"bpmn:completionCondition", b"completionCondition"]) {
                    element_stack.pop();
                }
                // Handle closing of multiInstanceLoopCharacteristics
                else if matches_element_name(name.as_ref(), &[b"bpmn2:multiInstanceLoopCharacteristics", b"bpmn:multiInstanceLoopCharacteristics", b"multiInstanceLoopCharacteristics"]) {
                    element_stack.pop();
                }
                // Handle closing of extensionElements
                else if matches_element_name(name.as_ref(), &[b"bpmn2:extensionElements", b"bpmn:extensionElements", b"extensionElements"]) {
                    element_stack.pop();
                    extension_elements_data.current_key = None;
                }
                // Handle closing of serviceTask
                else if matches_element_name(name.as_ref(), &[b"bpmn2:serviceTask", b"bpmn:serviceTask", b"serviceTask"]) {
                    if let Some(task_id) = current_task_id.take() {
                        let mi = multi_instance_data.clone();
                        // Only set loop characteristics if we actually parsed multi-instance data
                        if mi.loop_cardinality.is_some() || mi.is_parallel.is_some() ||
                           mi.completion_condition.is_some() || mi.behavior.is_some() {
                            if let Some(ProcessElement::ServiceTask(ref mut task)) = elements.get_mut(&task_id) {
                                task.loop_characteristics = Some(MultiInstanceLoopCharacteristics {
                                    is_parallel: mi.is_parallel.unwrap_or(false),
                                    loop_cardinality: mi.loop_cardinality,
                                    completion_condition: mi.completion_condition,
                                    behavior: mi.behavior,
                                });
                            }
                        }
                        // Attach extension elements if any were parsed
                        if !extension_elements_data.properties.is_empty() || extension_elements_data.current_key.is_some() {
                            if let Some(ProcessElement::ServiceTask(ref mut task)) = elements.get_mut(&task_id) {
                                task.base.extension_elements = Some(ExtensionElements {
                                    properties: extension_elements_data.properties.clone(),
                                    documentation: None,
                                });
                            }
                        }
                    }
                    multi_instance_data = MultiInstanceData::default();
                    extension_elements_data = ExtensionElementsData::default();
                    current_task_type = None;
                    element_stack.retain(|e| e != "serviceTask" && e != "multiInstanceLoopCharacteristics");
                }
                // Handle closing of userTask
                else if matches_element_name(name.as_ref(), &[b"bpmn2:userTask", b"bpmn:userTask", b"userTask"]) {
                    if let Some(task_id) = current_task_id.take() {
                        let mi = multi_instance_data.clone();
                        // Only set loop characteristics if we actually parsed multi-instance data
                        if mi.loop_cardinality.is_some() || mi.is_parallel.is_some() ||
                           mi.completion_condition.is_some() || mi.behavior.is_some() {
                            if let Some(ProcessElement::UserTask(ref mut task)) = elements.get_mut(&task_id) {
                                task.loop_characteristics = Some(MultiInstanceLoopCharacteristics {
                                    is_parallel: mi.is_parallel.unwrap_or(false),
                                    loop_cardinality: mi.loop_cardinality,
                                    completion_condition: mi.completion_condition,
                                    behavior: mi.behavior,
                                });
                            }
                        }
                        // Attach extension elements if any were parsed
                        if !extension_elements_data.properties.is_empty() || extension_elements_data.current_key.is_some() {
                            if let Some(ProcessElement::UserTask(ref mut task)) = elements.get_mut(&task_id) {
                                task.base.extension_elements = Some(ExtensionElements {
                                    properties: extension_elements_data.properties.clone(),
                                    documentation: None,
                                });
                            }
                        }
                    }
                    multi_instance_data = MultiInstanceData::default();
                    extension_elements_data = ExtensionElementsData::default();
                    current_task_type = None;
                    element_stack.retain(|e| e != "userTask" && e != "multiInstanceLoopCharacteristics");
                }
                // Handle closing of scriptTask
                else if matches_element_name(name.as_ref(), &[b"bpmn2:scriptTask", b"bpmn:scriptTask", b"scriptTask"]) {
                    if let Some(task_id) = current_task_id.take() {
                        let mi = multi_instance_data.clone();
                        // Only set loop characteristics if we actually parsed multi-instance data
                        if mi.loop_cardinality.is_some() || mi.is_parallel.is_some() ||
                           mi.completion_condition.is_some() || mi.behavior.is_some() {
                            if let Some(ProcessElement::ScriptTask(ref mut task)) = elements.get_mut(&task_id) {
                                task.loop_characteristics = Some(MultiInstanceLoopCharacteristics {
                                    is_parallel: mi.is_parallel.unwrap_or(false),
                                    loop_cardinality: mi.loop_cardinality,
                                    completion_condition: mi.completion_condition,
                                    behavior: mi.behavior,
                                });
                            }
                        }
                        // Attach extension elements if any were parsed
                        if !extension_elements_data.properties.is_empty() || extension_elements_data.current_key.is_some() {
                            if let Some(ProcessElement::ScriptTask(ref mut task)) = elements.get_mut(&task_id) {
                                task.base.extension_elements = Some(ExtensionElements {
                                    properties: extension_elements_data.properties.clone(),
                                    documentation: None,
                                });
                            }
                        }
                    }
                    multi_instance_data = MultiInstanceData::default();
                    extension_elements_data = ExtensionElementsData::default();
                    current_task_type = None;
                    element_stack.retain(|e| e != "scriptTask" && e != "multiInstanceLoopCharacteristics");
                }
                // Handle closing of Call Activity
                else if matches_element_name(name.as_ref(), &[b"bpmn2:callActivity", b"bpmn:callActivity", b"callActivity"]) {
                    if let Some(call_id) = current_call_activity_id.take() {
                        elements.insert(
                            call_id.clone(),
                            ProcessElement::CallActivity(CallActivity {
                                base: ElementBase {
                                    id: call_id,
                                    name: current_call_activity_name.take(),
                                    documentation: None,
                                    extension_elements: None,
                                },
                                called_element: call_activity_data.called_element.clone(),
                                business_key: call_activity_data.business_key.clone(),
                                data_input_associations: call_activity_data.data_input_associations.iter().map(|d| {
                                    DataAssociation {
                                        source_ref: d.source_ref.clone(),
                                        target_ref: d.target_ref.clone(),
                                        transformation: d.transformation.clone(),
                                    }
                                }).collect(),
                                data_output_associations: call_activity_data.data_output_associations.iter().map(|d| {
                                    DataAssociation {
                                        source_ref: d.source_ref.clone(),
                                        target_ref: d.target_ref.clone(),
                                        transformation: d.transformation.clone(),
                                    }
                                }).collect(),
                            }),
                        );
                    }
                    call_activity_data = CallActivityData::default();
                    element_stack.clear();
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                return Err(crate::model::format::ParseError::Xml(format!(
                    "XML parse error: {}",
                    e
                )));
            }
            _ => {}
        }
        buf.clear();
    }

    let process_id = process_id.ok_or_else(|| {
        crate::model::format::ParseError::Xml("Process ID not found".to_string())
    })?;

    Ok(ProcessDefinition {
        id: process_id,
        name: process_name,
        process_type: "process".to_string(),
        is_executable,
        elements,
        flows,
        variables,
    })
}

/// Serialize ProcessDefinition to BPMN XML
///
/// # Arguments
/// * `definition` - Process definition to serialize
///
/// # Returns
/// * `Ok(String)` - XML string
/// * `Err(SerializeError)` - Serialization error
pub fn serialize_bpmn_xml(definition: &ProcessDefinition) -> Result<String, crate::model::format::SerializeError> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    
    // Write XML declaration
    let decl = BytesDecl::new("1.0", Some("UTF-8"), None);
    writer.write_event(Event::Decl(decl))
        .map_err(|e| crate::model::format::SerializeError::Xml(format!("{}", e)))?;

    // Write definitions element
    let mut definitions = BytesStart::new("bpmn2:definitions");
    definitions.push_attribute(("xmlns:bpmn2", namespaces::BPMN2));
    definitions.push_attribute(("xmlns:bpmndi", namespaces::BPMNDI));
    definitions.push_attribute(("xmlns:dc", namespaces::DC));
    definitions.push_attribute(("xmlns:di", namespaces::DI));
    definitions.push_attribute(("id", "Definitions"));
    definitions.push_attribute(("targetNamespace", "http://bpmn.io/schema/bpmn"));
    
    writer.write_event(Event::Start(definitions))
        .map_err(|e| crate::model::format::SerializeError::Xml(format!("{}", e)))?;

    // Write process element
    let mut process = BytesStart::new("bpmn2:process");
    process.push_attribute(("id", definition.id.as_str()));
    if let Some(name) = &definition.name {
        process.push_attribute(("name", name.as_str()));
    }
    process.push_attribute(("isExecutable", if definition.is_executable { "true" } else { "false" }));
    
    writer.write_event(Event::Start(process))
        .map_err(|e| crate::model::format::SerializeError::Xml(format!("{}", e)))?;

    // Write elements
    for element in definition.elements.values() {
        match element {
            ProcessElement::StartEvent(e) => {
                let mut start = BytesStart::new("bpmn2:startEvent");
                start.push_attribute(("id", e.base.id.as_str()));
                if let Some(name) = &e.base.name {
                    start.push_attribute(("name", name.as_str()));
                }
                writer.write_event(Event::Empty(start))
                    .map_err(|e| crate::model::format::SerializeError::Xml(format!("{}", e)))?;
            }
            ProcessElement::EndEvent(e) => {
                let mut end = BytesStart::new("bpmn2:endEvent");
                end.push_attribute(("id", e.base.id.as_str()));
                if let Some(name) = &e.base.name {
                    end.push_attribute(("name", name.as_str()));
                }
                writer.write_event(Event::Empty(end))
                    .map_err(|e| crate::model::format::SerializeError::Xml(format!("{}", e)))?;
            }
            ProcessElement::ServiceTask(e) => {
                let mut task = BytesStart::new("bpmn2:serviceTask");
                task.push_attribute(("id", e.base.id.as_str()));
                if let Some(name) = &e.base.name {
                    task.push_attribute(("name", name.as_str()));
                }
                writer.write_event(Event::Empty(task))
                    .map_err(|e| crate::model::format::SerializeError::Xml(format!("{}", e)))?;
            }
            ProcessElement::UserTask(e) => {
                let mut task = BytesStart::new("bpmn2:userTask");
                task.push_attribute(("id", e.base.id.as_str()));
                if let Some(name) = &e.base.name {
                    task.push_attribute(("name", name.as_str()));
                }
                writer.write_event(Event::Empty(task))
                    .map_err(|e| crate::model::format::SerializeError::Xml(format!("{}", e)))?;
            }
            ProcessElement::ExclusiveGateway(e) => {
                let mut gateway = BytesStart::new("bpmn2:exclusiveGateway");
                gateway.push_attribute(("id", e.base.id.as_str()));
                if let Some(name) = &e.base.name {
                    gateway.push_attribute(("name", name.as_str()));
                }
                writer.write_event(Event::Empty(gateway))
                    .map_err(|e| crate::model::format::SerializeError::Xml(format!("{}", e)))?;
            }
            ProcessElement::ParallelGateway(e) => {
                let mut gateway = BytesStart::new("bpmn2:parallelGateway");
                gateway.push_attribute(("id", e.base.id.as_str()));
                if let Some(name) = &e.base.name {
                    gateway.push_attribute(("name", name.as_str()));
                }
                writer.write_event(Event::Empty(gateway))
                    .map_err(|e| crate::model::format::SerializeError::Xml(format!("{}", e)))?;
            }
            _ => {
                // Handle other element types as needed
            }
        }
    }

    // Write sequence flows
    for flow in definition.flows.values() {
        let mut seq_flow = BytesStart::new("bpmn2:sequenceFlow");
        seq_flow.push_attribute(("id", flow.id.as_str()));
        seq_flow.push_attribute(("sourceRef", flow.source_ref.as_str()));
        seq_flow.push_attribute(("targetRef", flow.target_ref.as_str()));
        writer.write_event(Event::Empty(seq_flow))
            .map_err(|e| crate::model::format::SerializeError::Xml(format!("{}", e)))?;
    }

    // Close process
    writer.write_event(Event::End(BytesEnd::new("bpmn2:process")))
        .map_err(|e| crate::model::format::SerializeError::Xml(format!("{}", e)))?;

    // Close definitions
    writer.write_event(Event::End(BytesEnd::new("bpmn2:definitions")))
        .map_err(|e| crate::model::format::SerializeError::Xml(format!("{}", e)))?;

    let result = writer.into_inner().into_inner();
    String::from_utf8(result).map_err(|e| {
        crate::model::format::SerializeError::Xml(format!("UTF-8 error: {}", e))
    })
}

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

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
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
                    if let Some(id) = attrs.get("id") {
                        elements.insert(
                            id.clone(),
                            ProcessElement::StartEvent(StartEvent {
                                base: ElementBase {
                                    id: id.clone(),
                                    name: attrs.get("name").cloned(),
                                    documentation: None,
                                },
                                event_definition: None,
                            }),
                        );
                    }
                }
                // End Event
                else if matches_element_name(name.as_ref(), &[b"bpmn2:endEvent", b"bpmn:endEvent", b"endEvent"]) {
                    if let Some(id) = attrs.get("id") {
                        elements.insert(
                            id.clone(),
                            ProcessElement::EndEvent(EndEvent {
                                base: ElementBase {
                                    id: id.clone(),
                                    name: attrs.get("name").cloned(),
                                    documentation: None,
                                },
                                event_definition: None,
                            }),
                        );
                    }
                }
                // Service Task
                else if matches_element_name(name.as_ref(), &[b"bpmn2:serviceTask", b"bpmn:serviceTask", b"serviceTask"]) {
                    if let Some(id) = attrs.get("id") {
                        elements.insert(
                            id.clone(),
                            ProcessElement::ServiceTask(ServiceTask {
                                base: ElementBase {
                                    id: id.clone(),
                                    name: attrs.get("name").cloned(),
                                    documentation: None,
                                },
                                implementation: attrs.get("implementation").cloned(),
                                operation_ref: attrs.get("operationRef").cloned(),
                                io_mapping: Default::default(),
                            }),
                        );
                    }
                }
                // User Task
                else if matches_element_name(name.as_ref(), &[b"bpmn2:userTask", b"bpmn:userTask", b"userTask"]) {
                    if let Some(id) = attrs.get("id") {
                        elements.insert(
                            id.clone(),
                            ProcessElement::UserTask(UserTask {
                                base: ElementBase {
                                    id: id.clone(),
                                    name: attrs.get("name").cloned(),
                                    documentation: None,
                                },
                                assignment: None,
                                form_key: attrs.get("formKey").cloned(),
                            }),
                        );
                    }
                }
                // Script Task
                else if matches_element_name(name.as_ref(), &[b"bpmn2:scriptTask", b"bpmn:scriptTask", b"scriptTask"]) {
                    if let Some(id) = attrs.get("id") {
                        elements.insert(
                            id.clone(),
                            ProcessElement::ScriptTask(ScriptTask {
                                base: ElementBase {
                                    id: id.clone(),
                                    name: attrs.get("name").cloned(),
                                    documentation: None,
                                },
                                script_format: attrs.get("scriptFormat").cloned(),
                                script: None,
                            }),
                        );
                    }
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
                                },
                                default_flow: attrs.get("default").cloned(),
                            }),
                        );
                    }
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

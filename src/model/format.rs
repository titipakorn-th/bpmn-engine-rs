//! BPMN Format Detection and Unified Interface
//!
//! Format detection and unified parsing interface for BPMN 2.0 definitions.

use crate::model::ProcessDefinition;
use thiserror::Error;

/// BPMN Format
///
/// Supported BPMN serialization formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BpmnFormat {
    /// JSON format
    Json,
    /// XML format (BPMN 2.0 standard)
    Xml,
}

/// Format Detection Error
#[derive(Debug, Error)]
pub enum DetectionError {
    #[error("Unable to detect format: input is empty or ambiguous")]
    UnableToDetect,
    #[error("Invalid format: {0}")]
    InvalidFormat(String),
}

/// Format Detector
///
/// Detects the format of BPMN input data (JSON or XML).
pub struct FormatDetector;

impl FormatDetector {
    /// Detect the format of input data
    ///
    /// # Arguments
    /// * `input` - Input string (JSON or XML)
    ///
    /// # Returns
    /// * `Ok(BpmnFormat)` - Detected format
    /// * `Err(DetectionError)` - Detection error
    pub fn detect(input: &str) -> Result<BpmnFormat, DetectionError> {
        let trimmed = input.trim();
        
        if trimmed.is_empty() {
            return Err(DetectionError::UnableToDetect);
        }

        // Check for XML indicators
        if trimmed.starts_with("<?xml") || trimmed.starts_with("<bpmn2:") || trimmed.starts_with("<bpmn:") {
            return Ok(BpmnFormat::Xml);
        }

        // Check for JSON indicators
        if trimmed.starts_with('{') || trimmed.starts_with('[') {
            return Ok(BpmnFormat::Json);
        }

        // Try to detect by looking for XML tags
        if trimmed.contains('<') && trimmed.contains('>') {
            // Check for common BPMN XML elements
            if trimmed.contains("bpmn2:") || trimmed.contains("bpmn:") || trimmed.contains("definitions") {
                return Ok(BpmnFormat::Xml);
            }
        }

        // Default to JSON if it looks like JSON structure
        if trimmed.starts_with('{') {
            return Ok(BpmnFormat::Json);
        }

        Err(DetectionError::UnableToDetect)
    }

    /// Detect format with confidence score
    ///
    /// Returns the detected format along with a confidence score (0.0 to 1.0).
    pub fn detect_with_confidence(input: &str) -> Result<(BpmnFormat, f64), DetectionError> {
        let format = Self::detect(input)?;
        
        let confidence = match format {
            BpmnFormat::Xml => {
                if input.trim().starts_with("<?xml") {
                    1.0
                } else if input.contains("bpmn2:") || input.contains("bpmn:") {
                    0.9
                } else {
                    0.7
                }
            }
            BpmnFormat::Json => {
                if input.trim().starts_with('{') && input.trim().ends_with('}') {
                    0.95
                } else {
                    0.8
                }
            }
        };

        Ok((format, confidence))
    }
}

/// Parse Error
///
/// Unified error type for parsing operations.
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("XML parse error: {0}")]
    Xml(String),
    #[error("Format detection error: {0}")]
    Detection(#[from] DetectionError),
    #[error("Unsupported format")]
    UnsupportedFormat,
}

/// Serialize Error
///
/// Unified error type for serialization operations.
#[derive(Debug, Error)]
pub enum SerializeError {
    #[error("JSON serialize error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("XML serialize error: {0}")]
    Xml(String),
    #[error("Unsupported format")]
    UnsupportedFormat,
}

/// BPMN Parser Trait
///
/// Unified interface for parsing BPMN definitions from different formats.
pub trait BpmnParser: Send + Sync {
    /// Parse input string into ProcessDefinition
    fn parse(&self, input: &str) -> Result<ProcessDefinition, ParseError>;
    
    /// Get the format this parser handles
    fn format(&self) -> BpmnFormat;
}

/// JSON Parser
///
/// Parser for BPMN JSON format.
pub struct JsonParser;

impl BpmnParser for JsonParser {
    fn parse(&self, input: &str) -> Result<ProcessDefinition, ParseError> {
        ProcessDefinition::from_json(input).map_err(ParseError::Json)
    }

    fn format(&self) -> BpmnFormat {
        BpmnFormat::Json
    }
}

/// XML Parser
///
/// Parser for BPMN XML format.
pub struct XmlParser;

impl BpmnParser for XmlParser {
    fn parse(&self, input: &str) -> Result<ProcessDefinition, ParseError> {
        ProcessDefinition::from_xml(input)
    }

    fn format(&self) -> BpmnFormat {
        BpmnFormat::Xml
    }
}

/// Auto Parser
///
/// Parser that automatically detects format and parses accordingly.
pub struct AutoParser;

impl AutoParser {
    /// Parse with automatic format detection
    pub fn parse(input: &str) -> Result<(ProcessDefinition, BpmnFormat), ParseError> {
        let format = FormatDetector::detect(input)?;
        
        let definition = match format {
            BpmnFormat::Json => ProcessDefinition::from_json(input).map_err(ParseError::Json)?,
            BpmnFormat::Xml => ProcessDefinition::from_xml(input)?,
        };

        Ok((definition, format))
    }
}


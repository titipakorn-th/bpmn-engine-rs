//! Test Fixtures
//!
//! BPMN JSON definition fixtures for testing.

/// Simple process: Start -> Task -> End
pub const SIMPLE_PROCESS_JSON: &str = r#"
{
    "id": "simple_process",
    "name": "Simple Process",
    "isExecutable": true,
    "elements": [
        {
            "type": "startEvent",
            "id": "start1",
            "name": "Start"
        },
        {
            "type": "serviceTask",
            "id": "task1",
            "name": "Process Task"
        },
        {
            "type": "endEvent",
            "id": "end1",
            "name": "End"
        },
        {
            "type": "sequenceFlow",
            "id": "flow1",
            "sourceRef": "start1",
            "targetRef": "task1"
        },
        {
            "type": "sequenceFlow",
            "id": "flow2",
            "sourceRef": "task1",
            "targetRef": "end1"
        }
    ],
    "variables": {}
}
"#;

/// Process with gateway: Start -> Gateway -> Task1/Task2 -> End
pub const GATEWAY_PROCESS_JSON: &str = r#"
{
    "id": "gateway_process",
    "name": "Gateway Process",
    "isExecutable": true,
    "elements": [
        {
            "type": "startEvent",
            "id": "start1",
            "name": "Start"
        },
        {
            "type": "exclusiveGateway",
            "id": "gateway1",
            "name": "Decision Gateway"
        },
        {
            "type": "serviceTask",
            "id": "task1",
            "name": "Task 1"
        },
        {
            "type": "serviceTask",
            "id": "task2",
            "name": "Task 2"
        },
        {
            "type": "endEvent",
            "id": "end1",
            "name": "End"
        },
        {
            "type": "sequenceFlow",
            "id": "flow1",
            "sourceRef": "start1",
            "targetRef": "gateway1"
        },
        {
            "type": "sequenceFlow",
            "id": "flow2",
            "sourceRef": "gateway1",
            "targetRef": "task1",
            "conditionExpression": {
                "language": "javascript",
                "body": "true"
            }
        },
        {
            "type": "sequenceFlow",
            "id": "flow3",
            "sourceRef": "gateway1",
            "targetRef": "task2"
        },
        {
            "type": "sequenceFlow",
            "id": "flow4",
            "sourceRef": "task1",
            "targetRef": "end1"
        },
        {
            "type": "sequenceFlow",
            "id": "flow5",
            "sourceRef": "task2",
            "targetRef": "end1"
        }
    ],
    "variables": {}
}
"#;

/// Parallel process: Start -> Parallel Gateway -> Task1 & Task2 -> Join -> End
pub const PARALLEL_PROCESS_JSON: &str = r#"
{
    "id": "parallel_process",
    "name": "Parallel Process",
    "isExecutable": true,
    "elements": [
        {
            "type": "startEvent",
            "id": "start1",
            "name": "Start"
        },
        {
            "type": "parallelGateway",
            "id": "split1",
            "name": "Split"
        },
        {
            "type": "serviceTask",
            "id": "task1",
            "name": "Task 1"
        },
        {
            "type": "serviceTask",
            "id": "task2",
            "name": "Task 2"
        },
        {
            "type": "parallelGateway",
            "id": "join1",
            "name": "Join"
        },
        {
            "type": "endEvent",
            "id": "end1",
            "name": "End"
        },
        {
            "type": "sequenceFlow",
            "id": "flow1",
            "sourceRef": "start1",
            "targetRef": "split1"
        },
        {
            "type": "sequenceFlow",
            "id": "flow2",
            "sourceRef": "split1",
            "targetRef": "task1"
        },
        {
            "type": "sequenceFlow",
            "id": "flow3",
            "sourceRef": "split1",
            "targetRef": "task2"
        },
        {
            "type": "sequenceFlow",
            "id": "flow4",
            "sourceRef": "task1",
            "targetRef": "join1"
        },
        {
            "type": "sequenceFlow",
            "id": "flow5",
            "sourceRef": "task2",
            "targetRef": "join1"
        },
        {
            "type": "sequenceFlow",
            "id": "flow6",
            "sourceRef": "join1",
            "targetRef": "end1"
        }
    ],
    "variables": {}
}
"#;

/// Complex process with multiple elements
pub const COMPLEX_PROCESS_JSON: &str = r#"
{
    "id": "complex_process",
    "name": "Complex Process",
    "isExecutable": true,
    "elements": [
        {
            "type": "startEvent",
            "id": "start1",
            "name": "Start"
        },
        {
            "type": "userTask",
            "id": "userTask1",
            "name": "User Task"
        },
        {
            "type": "scriptTask",
            "id": "scriptTask1",
            "name": "Script Task",
            "scriptFormat": "javascript",
            "script": "console.log('test');"
        },
        {
            "type": "manualTask",
            "id": "manualTask1",
            "name": "Manual Task"
        },
        {
            "type": "inclusiveGateway",
            "id": "inclusiveGateway1",
            "name": "Inclusive Gateway"
        },
        {
            "type": "intermediateCatchEvent",
            "id": "catchEvent1",
            "name": "Catch Event"
        },
        {
            "type": "intermediateThrowEvent",
            "id": "throwEvent1",
            "name": "Throw Event"
        },
        {
            "type": "endEvent",
            "id": "end1",
            "name": "End"
        },
        {
            "type": "sequenceFlow",
            "id": "flow1",
            "sourceRef": "start1",
            "targetRef": "userTask1"
        },
        {
            "type": "sequenceFlow",
            "id": "flow2",
            "sourceRef": "userTask1",
            "targetRef": "scriptTask1"
        },
        {
            "type": "sequenceFlow",
            "id": "flow3",
            "sourceRef": "scriptTask1",
            "targetRef": "manualTask1"
        },
        {
            "type": "sequenceFlow",
            "id": "flow4",
            "sourceRef": "manualTask1",
            "targetRef": "inclusiveGateway1"
        },
        {
            "type": "sequenceFlow",
            "id": "flow5",
            "sourceRef": "inclusiveGateway1",
            "targetRef": "catchEvent1"
        },
        {
            "type": "sequenceFlow",
            "id": "flow6",
            "sourceRef": "catchEvent1",
            "targetRef": "throwEvent1"
        },
        {
            "type": "sequenceFlow",
            "id": "flow7",
            "sourceRef": "throwEvent1",
            "targetRef": "end1"
        }
    ],
    "variables": {
        "var1": {
            "name": "var1",
            "variableType": "string",
            "defaultValue": "test"
        }
    }
}
"#;

/// Invalid process (missing start event)
pub const INVALID_PROCESS_NO_START_JSON: &str = r#"
{
    "id": "invalid_process",
    "name": "Invalid Process",
    "isExecutable": true,
    "elements": [
        {
            "type": "serviceTask",
            "id": "task1",
            "name": "Task"
        },
        {
            "type": "endEvent",
            "id": "end1",
            "name": "End"
        }
    ],
    "variables": {}
}
"#;

/// Invalid JSON (malformed)
pub const INVALID_JSON: &str = r#"
{
    "id": "invalid",
    "name": "Invalid",
    "elements": [
        {
            "type": "startEvent",
            "id": "start1"
            // Missing comma
            "name": "Start"
        }
    ]
}
"#;


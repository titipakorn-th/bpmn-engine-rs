//! Timer event parsing unit tests

use bpmn_engine::model::elements::*;
use bpmn_engine::model::elements::EventDefinition;
use bpmn_engine::model::elements::TimerDefinition;

#[test]
fn test_timer_definition_time_date() {
    let timer = TimerDefinition::TimeDate("2025-01-15T10:00:00".to_string());
    match timer {
        TimerDefinition::TimeDate(date) => {
            assert_eq!(date, "2025-01-15T10:00:00");
        }
        _ => panic!("Expected TimeDate variant"),
    }
}

#[test]
fn test_timer_definition_time_duration() {
    let timer = TimerDefinition::TimeDuration("PT3D".to_string());
    match timer {
        TimerDefinition::TimeDuration(duration) => {
            assert_eq!(duration, "PT3D");
        }
        _ => panic!("Expected TimeDuration variant"),
    }
}

#[test]
fn test_timer_definition_time_cycle() {
    let timer = TimerDefinition::TimeCycle("R3/PT1H".to_string());
    match timer {
        TimerDefinition::TimeCycle(cycle) => {
            assert_eq!(cycle, "R3/PT1H");
        }
        _ => panic!("Expected TimeCycle variant"),
    }
}

#[test]
fn test_timer_definition_clone() {
    let timer = TimerDefinition::TimeDuration("PT1H".to_string());
    let cloned = timer.clone();
    assert_eq!(timer, cloned);
}

#[test]
fn test_timer_definition_debug() {
    let timer = TimerDefinition::TimeDate("2025-01-01".to_string());
    let debug_str = format!("{:?}", timer);
    assert!(debug_str.contains("TimeDate"));
}

#[test]
fn test_parse_timer_event_with_duration() {
    let xml = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL">
            <process id="Process_1" isExecutable="true">
                <intermediateCatchEvent id="timer1" name="3 Day Timer">
                    <timerEventDefinition>
                        <timeDuration>PT3D</timeDuration>
                    </timerEventDefinition>
                </intermediateCatchEvent>
            </process>
        </definitions>
    "#;

    let result = ProcessDefinition::from_xml(xml);
    assert!(result.is_ok());

    let process = result.unwrap();
    let element = process.elements.get("timer1");

    match element {
        Some(ProcessElement::IntermediateCatchEvent(event)) => {
            assert_eq!(event.base.name, Some("3 Day Timer".to_string()));

            match &event.event_definition {
                Some(EventDefinition::Timer { time_definition, timer_def }) => {
                    assert_eq!(time_definition.as_deref(), Some("PT3D"));
                    match timer_def {
                        Some(TimerDefinition::TimeDuration(dur)) => {
                            assert_eq!(dur, "PT3D");
                        }
                        _ => panic!("Expected TimeDuration variant"),
                    }
                }
                _ => panic!("Expected Timer event definition"),
            }
        }
        _ => panic!("Expected IntermediateCatchEvent"),
    }
}

#[test]
fn test_parse_start_event_with_timer() {
    let xml = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL">
            <process id="Process_1" isExecutable="true">
                <startEvent id="startTimer" name="Timer Start">
                    <timerEventDefinition>
                        <timeDate>2025-01-15T10:00:00</timeDate>
                    </timerEventDefinition>
                </startEvent>
            </process>
        </definitions>
    "#;

    let result = ProcessDefinition::from_xml(xml);
    assert!(result.is_ok());

    let process = result.unwrap();
    let element = process.elements.get("startTimer");

    match element {
        Some(ProcessElement::StartEvent(event)) => {
            assert_eq!(event.base.name, Some("Timer Start".to_string()));

            match &event.event_definition {
                Some(EventDefinition::Timer { time_definition, timer_def }) => {
                    assert_eq!(time_definition.as_deref(), Some("2025-01-15T10:00:00"));
                    match timer_def {
                        Some(TimerDefinition::TimeDate(date)) => {
                            assert_eq!(date, "2025-01-15T10:00:00");
                        }
                        _ => panic!("Expected TimeDate variant"),
                    }
                }
                _ => panic!("Expected Timer event definition"),
            }
        }
        _ => panic!("Expected StartEvent"),
    }
}

#[test]
fn test_parse_event_with_time_cycle() {
    let xml = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL">
            <process id="Process_1" isExecutable="true">
                <intermediateCatchEvent id="cycleTimer" name="Hourly Timer">
                    <timerEventDefinition>
                        <timeCycle>R3/PT1H</timeCycle>
                    </timerEventDefinition>
                </intermediateCatchEvent>
            </process>
        </definitions>
    "#;

    let result = ProcessDefinition::from_xml(xml);
    assert!(result.is_ok());

    let process = result.unwrap();
    let element = process.elements.get("cycleTimer");

    match element {
        Some(ProcessElement::IntermediateCatchEvent(event)) => {
            match &event.event_definition {
                Some(EventDefinition::Timer { time_definition, timer_def }) => {
                    assert_eq!(time_definition.as_deref(), Some("R3/PT1H"));
                    match timer_def {
                        Some(TimerDefinition::TimeCycle(cycle)) => {
                            assert_eq!(cycle, "R3/PT1H");
                        }
                        _ => panic!("Expected TimeCycle variant"),
                    }
                }
                _ => panic!("Expected Timer event definition"),
            }
        }
        _ => panic!("Expected IntermediateCatchEvent"),
    }
}

#[test]
fn test_parse_end_event_without_timer() {
    let xml = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL">
            <process id="Process_1" isExecutable="true">
                <endEvent id="end1" name="End Event"/>
            </process>
        </definitions>
    "#;

    let result = ProcessDefinition::from_xml(xml);
    assert!(result.is_ok());

    let process = result.unwrap();
    let element = process.elements.get("end1");

    match element {
        Some(ProcessElement::EndEvent(event)) => {
            assert_eq!(event.base.name, Some("End Event".to_string()));
            assert!(event.event_definition.is_none());
        }
        _ => panic!("Expected EndEvent"),
    }
}

#[test]
fn test_timer_definition_from_timer_data() {
    use bpmn_engine::model::xml::TimerData;

    let data = TimerData {
        time_date: Some("2025-06-15".to_string()),
        time_duration: None,
        time_cycle: None,
    };
    let timer = TimerDefinition::from_timer_data(&data);
    match timer {
        TimerDefinition::TimeDate(date) => assert_eq!(date, "2025-06-15"),
        _ => panic!("Expected TimeDate"),
    }

    let data = TimerData {
        time_date: None,
        time_duration: Some("PT2H".to_string()),
        time_cycle: None,
    };
    let timer = TimerDefinition::from_timer_data(&data);
    match timer {
        TimerDefinition::TimeDuration(dur) => assert_eq!(dur, "PT2H"),
        _ => panic!("Expected TimeDuration"),
    }

    let data = TimerData {
        time_date: None,
        time_duration: None,
        time_cycle: Some("R5/PT30M".to_string()),
    };
    let timer = TimerDefinition::from_timer_data(&data);
    match timer {
        TimerDefinition::TimeCycle(cycle) => assert_eq!(cycle, "R5/PT30M"),
        _ => panic!("Expected TimeCycle"),
    }
}

#[test]
fn test_timer_event_with_bpmn2_namespace() {
    let xml = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL">
            <process id="Process_1" isExecutable="true">
                <bpmn2:intermediateCatchEvent id="timer1" name="Timer Event">
                    <bpmn2:timerEventDefinition>
                        <bpmn2:timeDuration>PT1H30M</bpmn2:timeDuration>
                    </bpmn2:timerEventDefinition>
                </bpmn2:intermediateCatchEvent>
            </process>
        </definitions>
    "#;

    let result = ProcessDefinition::from_xml(xml);
    assert!(result.is_ok());

    let process = result.unwrap();
    let element = process.elements.get("timer1");

    match element {
        Some(ProcessElement::IntermediateCatchEvent(event)) => {
            assert_eq!(event.base.name, Some("Timer Event".to_string()));

            match &event.event_definition {
                Some(EventDefinition::Timer { time_definition, timer_def }) => {
                    assert_eq!(time_definition.as_deref(), Some("PT1H30M"));
                    match timer_def {
                        Some(TimerDefinition::TimeDuration(dur)) => {
                            assert_eq!(dur, "PT1H30M");
                        }
                        _ => panic!("Expected TimeDuration variant"),
                    }
                }
                _ => panic!("Expected Timer event definition"),
            }
        }
        _ => panic!("Expected IntermediateCatchEvent"),
    }
}
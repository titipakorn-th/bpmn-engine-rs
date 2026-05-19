//! Condition Expression Evaluator
//!
//! Evaluates BPMN condition expressions against process context.

use serde_json::Value;

/// Evaluates a ConditionExpression against the process context.
/// Returns true if the condition passes, false otherwise.
pub fn evaluate_condition(
    condition: &crate::model::ConditionExpression,
    context: &Value,
) -> bool {
    if condition.body.trim().is_empty() {
        return false;
    }
    evaluate_expression(&condition.body, context)
}

/// Convenience function to evaluate a raw expression string.
pub fn evaluate_expression(expression: &str, context: &Value) -> bool {
    let expression = expression.trim();
    if expression.is_empty() {
        return false;
    }

    // Parse: "fieldPath op value" — split on whitespace operators
    let parts: Vec<&str> = expression.split_whitespace().collect();
    if parts.len() < 3 {
        return false;
    }

    let field_path = parts[0];
    let operator = parts[1];
    let joined = parts[2..].join(" ");
    let raw_value = joined.trim_matches('"').trim_matches('\'');

    let field_value = resolve_field_path(field_path, context);
    let Some(fv) = field_value else { return false; };

    match operator {
        "=" | "==" => compare_eq(&fv, raw_value),
        "!=" => !compare_eq(&fv, raw_value),
        ">" => compare_num(&fv, raw_value, |a, b| a > b),
        "<" => compare_num(&fv, raw_value, |a, b| a < b),
        ">=" => compare_num(&fv, raw_value, |a, b| a >= b),
        "<=" => compare_num(&fv, raw_value, |a, b| a <= b),
        "contains" => compare_contains(&fv, raw_value),
        _ => false,
    }
}

/// Resolves a field path in the JSON context (supports dot notation).
fn resolve_field_path(path: &str, context: &Value) -> Option<Value> {
    let parts: Vec<&str> = path.split('.').collect();
    let mut current: Option<&Value> = Some(context);

    for (i, part) in parts.iter().enumerate() {
        if i == parts.len() - 1 {
            if let Some(c) = current {
                if let Value::Object(map) = c {
                    current = map.get(*part);
                } else {
                    return None;
                }
            } else {
                return None;
            }
        } else {
            if let Some(c) = current {
                if let Value::Object(map) = c {
                    current = map.get(*part);
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
    }
    current.cloned()
}

fn compare_eq(field: &Value, raw_value: &str) -> bool {
    match field {
        Value::String(s) => s.eq_ignore_ascii_case(raw_value),
        Value::Number(n) => raw_value.parse::<f64>().map(|v| n.as_f64().map(|nf| (nf - v).abs() < f64::EPSILON).unwrap_or(false)).unwrap_or(false),
        Value::Bool(b) => raw_value.parse::<bool>().map(|v| *b == v).unwrap_or(false),
        Value::Null => raw_value.eq_ignore_ascii_case("null") || raw_value.is_empty(),
        _ => field.to_string().eq_ignore_ascii_case(raw_value),
    }
}

fn compare_num<F>(field: &Value, raw_value: &str, op: F) -> bool
where
    F: FnOnce(f64, f64) -> bool,
{
    match (field.as_f64(), raw_value.parse::<f64>()) {
        (Some(field_num), Ok(target_num)) => op(field_num, target_num),
        _ => false,
    }
}

fn compare_contains(field: &Value, raw_value: &str) -> bool {
    match field {
        Value::String(s) => s.to_lowercase().contains(&raw_value.to_lowercase()),
        Value::Array(arr) => arr.iter().any(|v| {
            if let Value::String(s) = v {
                s.eq_ignore_ascii_case(raw_value)
            } else {
                false
            }
        }),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq_string() {
        let ctx = serde_json::json!({"status": "Abnormal"});
        assert!(evaluate_expression("status = Abnormal", &ctx));
        assert!(evaluate_expression("status == Abnormal", &ctx));
    }

    #[test]
    fn test_eq_numeric() {
        let ctx = serde_json::json!({"sales": 15000.0});
        assert!(evaluate_expression("sales > 10000", &ctx));
        assert!(evaluate_expression("sales < 20000", &ctx));
        assert!(!evaluate_expression("sales > 20000", &ctx));
    }

    #[test]
    fn test_contains() {
        let ctx = serde_json::json!({"tags": ["urgent", "review"]});
        assert!(evaluate_expression("tags contains urgent", &ctx));
        assert!(evaluate_expression("tags contains review", &ctx));
        assert!(!evaluate_expression("tags contains unknown", &ctx));
    }

    #[test]
    fn test_nested_field() {
        let ctx = serde_json::json!({"site": {"region": "South"}});
        assert!(evaluate_expression("site.region = South", &ctx));
        assert!(evaluate_expression("site.region = south", &ctx)); // case insensitive
    }

    #[test]
    fn test_missing_field() {
        let ctx = serde_json::json!({"status": "ok"});
        assert!(!evaluate_expression("nonexistent = value", &ctx));
    }

    #[test]
    fn test_empty_body() {
        let ctx = serde_json::json!({"status": "ok"});
        assert!(!evaluate_expression("", &ctx));
        assert!(!evaluate_expression("   ", &ctx));
    }
}

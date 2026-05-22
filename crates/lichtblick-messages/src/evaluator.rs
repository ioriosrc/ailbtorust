// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use crate::parser::*;
use serde_json::Value;
use std::collections::HashMap;

/// Evaluate a parsed message path against a JSON message.
///
/// Returns all matching values from the message.
pub fn evaluate_message_path(
    path: &MessagePath,
    message: &Value,
    variables: &HashMap<String, Value>,
) -> Vec<Value> {
    let mut results = vec![message.clone()];

    for part in &path.parts {
        let mut next_results = Vec::new();
        for value in &results {
            match part {
                MessagePathPart::Field(name) => {
                    if let Some(v) = value.get(name) {
                        next_results.push(v.clone());
                    }
                }
                MessagePathPart::Index(idx) => {
                    if let Value::Array(arr) = value {
                        let actual_idx = if *idx < 0 {
                            (arr.len() as i64 + idx) as usize
                        } else {
                            *idx as usize
                        };
                        if let Some(v) = arr.get(actual_idx) {
                            next_results.push(v.clone());
                        }
                    }
                }
                MessagePathPart::Slice { start, end } => {
                    if let Value::Array(arr) = value {
                        let len = arr.len() as i64;
                        let s = start.unwrap_or(0);
                        let e = end.unwrap_or(len);
                        let s = if s < 0 { (len + s) as usize } else { s as usize };
                        let e = if e < 0 { (len + e) as usize } else { e as usize };
                        let e = e.min(arr.len());
                        if s < e {
                            for item in &arr[s..e] {
                                next_results.push(item.clone());
                            }
                        }
                    }
                }
                MessagePathPart::Filter(condition) => {
                    if let Value::Array(arr) = value {
                        for item in arr {
                            if matches_filter(item, condition) {
                                next_results.push(item.clone());
                            }
                        }
                    } else if matches_filter(value, condition) {
                        next_results.push(value.clone());
                    }
                }
                MessagePathPart::Variable(var_name) => {
                    if let Some(var_value) = variables.get(var_name.as_str()) {
                        if let Value::Array(arr) = value {
                            if let Some(idx) = var_value.as_i64() {
                                let actual_idx = if idx < 0 {
                                    (arr.len() as i64 + idx) as usize
                                } else {
                                    idx as usize
                                };
                                if let Some(v) = arr.get(actual_idx) {
                                    next_results.push(v.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
        results = next_results;
    }

    results
}

fn matches_filter(value: &Value, condition: &FilterCondition) -> bool {
    let field_value = match value.get(&condition.field) {
        Some(v) => v,
        None => return false,
    };

    match (&condition.value, &condition.operator) {
        (FilterValue::Number(n), op) => {
            if let Some(fv) = field_value.as_f64() {
                compare_f64(fv, *n, *op)
            } else {
                false
            }
        }
        (FilterValue::String(s), op) => {
            if let Some(fv) = field_value.as_str() {
                compare_str(fv, s, *op)
            } else {
                false
            }
        }
        (FilterValue::Bool(b), FilterOperator::Equal) => {
            field_value.as_bool() == Some(*b)
        }
        (FilterValue::Bool(b), FilterOperator::NotEqual) => {
            field_value.as_bool() != Some(*b)
        }
        _ => false,
    }
}

fn compare_f64(a: f64, b: f64, op: FilterOperator) -> bool {
    match op {
        FilterOperator::Equal => (a - b).abs() < f64::EPSILON,
        FilterOperator::NotEqual => (a - b).abs() >= f64::EPSILON,
        FilterOperator::LessThan => a < b,
        FilterOperator::GreaterThan => a > b,
        FilterOperator::LessOrEqual => a <= b,
        FilterOperator::GreaterOrEqual => a >= b,
    }
}

fn compare_str(a: &str, b: &str, op: FilterOperator) -> bool {
    match op {
        FilterOperator::Equal => a == b,
        FilterOperator::NotEqual => a != b,
        FilterOperator::LessThan => a < b,
        FilterOperator::GreaterThan => a > b,
        FilterOperator::LessOrEqual => a <= b,
        FilterOperator::GreaterOrEqual => a >= b,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_simple_field() {
        let path = crate::parse_message_path("/topic.x").unwrap();
        let msg = json!({"x": 42});
        let results = evaluate_message_path(&path, &msg, &HashMap::new());
        assert_eq!(results, vec![json!(42)]);
    }

    #[test]
    fn test_nested_fields() {
        let path = crate::parse_message_path("/topic.header.stamp.sec").unwrap();
        let msg = json!({"header": {"stamp": {"sec": 100}}});
        let results = evaluate_message_path(&path, &msg, &HashMap::new());
        assert_eq!(results, vec![json!(100)]);
    }

    #[test]
    fn test_array_index() {
        let path = crate::parse_message_path("/topic.data[1]").unwrap();
        let msg = json!({"data": [10, 20, 30]});
        let results = evaluate_message_path(&path, &msg, &HashMap::new());
        assert_eq!(results, vec![json!(20)]);
    }

    #[test]
    fn test_filter() {
        let path = crate::parse_message_path("/topic.items{id==2}").unwrap();
        let msg = json!({"items": [{"id": 1, "v": "a"}, {"id": 2, "v": "b"}]});
        let results = evaluate_message_path(&path, &msg, &HashMap::new());
        assert_eq!(results, vec![json!({"id": 2, "v": "b"})]);
    }
}

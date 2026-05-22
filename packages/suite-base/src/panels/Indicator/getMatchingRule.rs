```rust
use std::assert_eq;

fn get_matching_rule(rawValue: serde_json::Value, rules: &[IndicatorRule]) -> Option<IndicatorRule> {
    let value = match raw_value {
        serde_json::Value::Bool(val) => val,
        serde_json::Value::Number(val) => val.as_f64().unwrap() as usize,
        serde_json::Value::String(_) | serde_json::Value::Null => return None, // Null values are not supported
        serde_json::Value::Array(_) => return None, // Arrays are not supported
        serde_json::Value::Object(_) => return None, // Objects are not supported
    };

    for rule in rules {
        let rhs: usize;
        match &rule.rawValue {
            serde_json::Value::Bool(val) => rhs = *val as usize,
            serde_json::Value::Number(val) => rhs = val.as_f64().unwrap() as usize,
            serde_json::Value::String(val) | serde_json::Value::Null => continue, // Null values are not supported
            serde_json::Value::Array(_) => continue, // Arrays are not supported
            serde_json::Value::Object(_) => return None, // Objects are not supported
        }

        match &rule.operator {
            "=" if value == rhs => return Some(rule),
            "<" if value < rhs => return Some(rule),
            "<=" if value <= rhs => return Some(rule),
            ">" if value > rhs => return Some(rule),
            ">=" if value >= rhs => return Some(rule),
            _ => continue,
        }
    }

    None
}
```
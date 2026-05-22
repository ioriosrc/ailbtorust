```rust
use serde_json::Value;

fn build_sample_message(datatypes: &Value, datatype: &str) -> Value {
    if let Some(builtin_value) = datatypes.get_builtin_sample_values().get(datatype) {
        return Value::from(builtin_value);
    }

    if let Some(fields) = datatypes.get_definition_fields(datatype).unwrap_or(&vec![]).iter() {
        let mut obj: serde_json::Value = serde_json::json!({});

        for field in fields {
            if field.is_constant.unwrap_or(false) || field.is_array.unwrap_or(false) {
                continue;
            }

            let sample = build_sample_message(datatypes, field.type.as_str());
            if field.is_array.unwrap_or(false) {
                if let Some(array_length) = field.array_length {
                    obj[field.name] = serde_json::json!(vec![sample; array_length]);
                } else {
                    obj[field.name] = serde_json::json!([sample]);
                }
            } else {
                obj[field.name] = sample;
            }
        }

        return obj;
    }

    serde_json::Value::Null
}
```
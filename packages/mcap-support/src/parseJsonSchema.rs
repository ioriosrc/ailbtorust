```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct DatatypeDefinition {
    name: String,
    type_: String,
}

#[derive(Serialize, Deserialize)]
struct MessageDefinitionMap {
    definitions: Vec<DatatypeDefinition>,
}

fn parse_json_schema(
    root_schema: serde_json::Value,
    root_type_name: &str,
) -> (MessageDefinitionMap, fn(serde_json::Value) -> serde_json::Value) {
    let mut datatypes = MessageDefinitionMap { definitions: vec![] };
    let mut postprocess_object = |value| value;

    fn add_fields_recursive(
        schema: serde_json::Value,
        current_type_name: &str,
        key_path: Vec<&str>,
    ) -> fn(serde_json::Value) -> serde_json::Value {
        let mut postprocess_nested_object =
            move |value| add_fields_recursive(schema, current_type_name, key_path);

        match schema {
            serde_json::Value::Object(properties) => {
                for (field_name, field_schema) in properties.into_iter() {
                    if let serde_json::Value::Array(ref one_of) = field_schema.get("oneOf") {
                        if one_of.iter().all(|alt| alt.as_f64().is_some()) {
                            for alt in one_of {
                                datatypes.definitions.push(DatatypeDefinition {
                                    name: alt.to_string(),
                                    type_: "uint32".to_string(),
                                    is_constant: true,
                                    value: alt.as_f64().unwrap(),
                                });
                            }
                            datatypes.definitions.push(DatatypeDefinition {
                                name: field_name.to_string(),
                                type_: "uint32".to_string(),
                                is_complex: true,
                            });
                        } else {
                            panic!("Unsupported type for oneOf alternatives must have number values");
                        }
                    } else {
                        match field_schema.get("type").unwrap() {
                            serde_json::Value::String(type_str) => {
                                if &type_str.to_lowercase() == "boolean" {
                                    datatypes.definitions.push(DatatypeDefinition {
                                        name: field_name.to_string(),
                                        type_: "bool".to_string(),
                                    });
                                } else if &type_str.to_lowercase() == "string" {
                                    match field_schema.get("contentEncoding") {
                                        Some(serde_json::Value::String(content_encoding)) => {
                                            if content_encoding == "base64" {
                                                datatypes.definitions.push(DatatypeDefinition {
                                                    name: field_name.to_string(),
                                                    type_: "uint8".to_string(),
                                                    is_complex: true,
                                                    isArray: true,
                                                });
                                            } else {
                                                panic!("Unsupported contentEncoding {:?}", content_encoding);
                                            }
                                        } else {
                                            datatypes.definitions.push(DatatypeDefinition {
                                                name: field_name.to_string(),
                                                type_: "string".to_string(),
                                                is_complex: true,
                                            });
                                        }
                                    }
                                } else if &type_str.to_lowercase() == "number" {
                                    datatypes.definitions.push(DatatypeDefinition {
                                        name: field_name.to_string(),
                                        type_: "float64".to_string(),
                                    });
                                } else if &type_str.to_lowercase() == "integer" {
                                    let is_constant = field_schema.get("minimum").is_some()
                                        || field_schema
                                            .get("exclusiveMinimum")
                                            .is_some_and(|min| min >= 0.0);
                                    datatypes.definitions.push(DatatypeDefinition {
                                        name: field_name.to_string(),
                                        type_: if is_constant {
                                            "uint32".to_string()
                                        } else {
                                            "int32".to_string()
                                        },
                                    });
                                } else if &type_str.to_lowercase() == "object" || &type_str.to_lowercase() == "array" {
                                    let nested_type_name = format!("{}.{}", current_type_name, field_name);
                                    datatypes.definitions.push(DatatypeDefinition {
                                        name: field_name.to_string(),
                                        type_: nested_type_name.clone(),
                                        is_complex: true,
                                    });
                                    postprocess_object = move |value| {
                                        add_fields_recursive(field_schema, &nested_type_name, key_path);
                                    };
                                } else {
                                    panic!("Unsupported type {:?}", type_str);
                                }
                            }
                        }
                    }
                }
            }
            serde_json::Value::Null => {}
            _ => {
                panic!("Unsupported JSON schema type {:?}", schema);
            }
        }

        postprocess_object
    };

    let postprocess_value = add_fields_recursive(root_schema, root_type_name, vec![]);
    (datatypes, postprocess_value)
}
```
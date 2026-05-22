```rust
use std::collections::HashMap;

fn constants_by_datatype(datatypes: HashMap<&str, Vec<Definition>>) -> HashMap<&str, HashMap<String, String>> {
    let mut result = HashMap::new();

    for (datatype, definitions) in datatypes {
        let mut constant_map = HashMap::new();
        for definition in definitions {
            if let Definition {
                type_: ref t,
                name: ref n,
                is_constant: true,
                value: ref v,
            } = definition {
                match t.to_lowercase().as_str() {
                    "uint32" | "int32" => constant_map.insert(n.to_string(), v.clone()),
                    "uint64" | "int64" => constant_map.insert(n.to_string(), v.clone()),
                    _ => (),
                }
            }
        }

        result.insert(datatype, constant_map);
    }

    result
}

fn enum_values_by_datatype_and_field(datatypes: HashMap<&str, Vec<Definition>>, field_name: &str) -> Option<HashMap<String, String>> {
    let mut result = HashMap::new();

    for (datatype, definitions) in datatypes {
        let mut constant_map = HashMap::new();
        for definition in definitions {
            if let Definition {
                type_: ref t,
                name: ref n,
                is_constant: true,
                value: ref v,
                is_array: false,
                is_complex: false,
            } = definition {
                if field_name == n {
                    match t.to_lowercase().as_str() {
                        "uint32" | "int32" => constant_map.insert(n.to_string(), v.clone()),
                        "uint64" | "int64" => constant_map.insert(n.to_string(), v.clone()),
                        _ => (),
                    }
                }
            }
        }

        result.insert(datatype, constant_map);
    }

    if result.is_empty() {
        return None;
    }

    Some(result)
}

fn extract_type_from_studio_enum_annotation(field_name: &str) -> Option<String> {
    match field_name.to_lowercase().as_str() {
        "foo__foxglove_enum" | "foo__webviz_enum" => Some("Foo".to_string()),
        _ => None,
    }
}
```
```rust
use lighntblick::{Immutable, RosDatatypes};

// Exported for tests
pub fn constants_by_datatype(datatypes: &Immutable<RosDatatypes>) -> std::collections::HashMap<&str, std::collections::HashMap<&str, String>>> {
    type Result = std::collections::HashMap<&str, String>;
    let mut results: std::collections::HashMap<&str, Result> = std::collections::HashMap::new();
    for (datatype, value) in datatypes.iter() {
        let result: Result = (results.entry(datatype).or_insert_with(|| std::collections::HashMap::new()));
        for field in &value.definitions {
            if field.is_constant && field.value.is_some() && !field.value.unwrap().is_boolean() {
                if result.contains_key(field.value.as_deref()) {
                    result.insert(field.value.as_deref(), "<multiple constants match>");
                } else {
                    result.insert(field.value.as_deref(), field.name.clone());
                }
            }
        }
    }
    results
}

fn extract_type_from_studio_enum_annotation(name: &str) -> Option<&str> {
    let re = regex!(r"(.*)__(foxglove|webviz)_enum");
    if let Some(cap) = re.captures(name) {
        cap.get(1).map(|c| c.as_str())
    } else {
        None
    }
}

fn enum_values_by_datatype_and_field(datatypes: &Immutable<RosDatatypes>) -> std::collections::HashMap<&str, std::collections::HashMap<&str, std::collections::HashMap<&str, String>>>> {
    let datatype_constants = constants_by_datatype(datatypes);
    let mut results: std::collections::HashMap<&str, std::collections::HashMap<&str, std::collections::HashMap<&str, String>>> = std::collections::HashMap::new();
    for (datatype, value) in datatypes.iter() {
        let current_result: std::collections::HashMap<&str, std::collections::HashMap<&str, String>> = (results.entry(datatype).or_insert_with(|| std::collections::HashMap::new()));
        // keep track of parsed constants
        let mut constants: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        // constants' types
        let mut last_type: Option<&str> = None;
        for field in &value.definitions {
            if last_type != Some(field.type_) {
                // encountering new type resets the accumulated constants
                constants.clear();
                last_type = Some(&field.type_);
            }

            if field.is_constant && field.value.is_some() && !field.value.unwrap().is_boolean() {
                last_type = Some(&field.type_);
                if constants.contains_key(field.value.as_deref()) {
                    constants.insert(field.value.clone(), "<multiple constants match>");
                } else {
                    constants.insert(field.value.clone(), field.name.clone());
                }
            }

            // check if current field is annotation of the form: "Foo bar__foxglove_enum"
            // This means that "bar" is enum of type "Foo"
            let mut found_field = false;
            for &enum_type in datatype_constants.get(datatype).unwrap().keys() {
                if format!("{}_{}", enum_type, field.name) == name {
                    current_result.insert(enum_type.clone(), constants);
                    found_field = true;
                    break;
                }
            }

            if !found_field && !constants.is_empty() {
                current_result.insert(field.type_.clone(), constants);
            }

            // only assign result if we found non-empty mapping into constants
            if !current_result.is_empty() {
                results[datatype] = current_result;
            }

            // and start over - reset constants
            constants.clear();
        }
    }
    results
}
```
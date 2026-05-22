```rust
use std::collections::HashMap;

fn protobuf_scalar_to_ros_primitive(type_str: &str) -> String {
    match type_str {
        "double" => "f64",
        "float" => "f32",
        "int32" | "sint32" | "sfixed32" => "i32",
        "uint32" | "fixed32" => "u32",
        "int64" | "sint64" | "sfixed64" => "i64",
        "uint64" | "fixed64" => "u64",
        "bool" => "bool",
        "string" => "String",
        _ => panic!("Expected protobuf scalar type, got {}", type_str),
    }
}

fn strip_leading_dot(type_name: &str) -> String {
    type_name.trim_start_matches('.').to_string()
}

pub fn protobuf_definitions_to_datatypes(
    datatypes: &mut HashMap<String, HashMap<String, bool>>,
    type_: &protobufjs::Type,
) {
    let mut definitions = vec![];

    // The empty list reference is added to the map so a `.has` lookup below can prevent infinite recursion on cyclical types
    if !datatypes.contains_key(strip_leading_dot(type_.fullName())) {
        datatypes.insert(
            strip_leading_dot(type_.fullName()),
            HashMap::new(),
        );
    }
    for field in type_.fields_array() {
        if field.resolved_type().is_instance_of::<protobufjs::Enum>() {
            for (name, value) in field.resolved_type().values() {
                // Note: names from different enums might conflict. The player API will need to be updated
                // to associate fields with enums (similar to the __foxglove_enum annotation hack).
                // https://github.com/foxglove/studio/issues/2214
                definitions.push(protobuf_datatype_field::DataTypeField {
                    name: name.to_string(),
                    type_: protobuf_scalar_to_ros_primitive(field.resolved_type().name()),
                    is_constant: true,
                    value: Some(value.clone()),
                });
            }
            definitions.push(protobuf_datatype_field::DataTypeField {
                name: field.name().to_string(),
                type_: protobuf_scalar_to_ros_primitive(field.resolved_type().name()),
                is_constant: false,
                value: None,
            });
        } else if let Some(resolved_type) = field.resolved_type() {
            let full_name = strip_leading_dot(resolved_type.fullName());
            definitions.push(protobuf_datatype_field::DataTypeField {
                name: field.name().to_string(),
                type_: full_name.to_string(),
                is_complex: true,
                isArray: field.repeated(),
            });

            // If we've already processed this datatype we should skip it.
            // This avoid infinite recursion with datatypes that reference themselves.
            if !datatypes.contains_key(full_name) {
                protobuf_definitions_to_datatypes(datatypes, resolved_type);
            }
        } else if field.type() == "bytes" {
            if field.repeated() {
                panic!("Repeated bytes are not currently supported");
            }
            definitions.push(protobuf_datatype_field::DataTypeField {
                name: field.name().to_string(),
                type_: protobuf_scalar_to_ros_primitive(field.type()),
                is_constant: false,
                value: None,
            });
        } else if let Some(type_name) = field.type() {
            definitions.push(protobuf_datatype_field::DataTypeField {
                name: field.name().to_string(),
                type_: type_name.to_string(),
                is_complex: true,
                isArray: field.repeated(),
            });

            // If we've already processed this datatype we should skip it.
            // This avoid infinite recursion with datatypes that reference themselves.
            if !datatypes.contains_key(type_name) {
                protobuf_definitions_to_datatypes(datatypes, type_);
            }
        } else {
            definitions.push(protobuf_datatype_field::DataTypeField {
                name: field.name().to_string(),
                type_: protobuf_scalar_to_ros_primitive(field.type()),
                is_constant: false,
                value: None,
            });
        }
    }

    // Store the list of fields for this datatype
    datatypes.insert(
        strip_leading_dot(type_.fullName()),
        definitions.into_iter().collect(),
    );
}
```
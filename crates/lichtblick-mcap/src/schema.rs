// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use lichtblick_core::error::LichtblickError;
use lichtblick_core::types::{MessageDefinition, MessageField, RosDatatypes};
use std::collections::HashMap;

/// Parse a schema into RosDatatypes.
pub fn parse_schema(
    name: &str,
    encoding: &str,
    data: &[u8],
) -> Result<RosDatatypes, LichtblickError> {
    match encoding {
        "ros2msg" | "ros1msg" => parse_ros_msg_schema(name, data),
        "ros2idl" => parse_ros_idl_schema(name, data),
        "jsonschema" => parse_json_schema(name, data),
        "protobuf" => parse_protobuf_schema(name, data),
        "flatbuffer" => parse_flatbuffer_schema(name, data),
        _ => {
            log::warn!("Unknown schema encoding '{}' for '{}'", encoding, name);
            Ok(RosDatatypes::new())
        }
    }
}

/// Parse ROS .msg format schema.
fn parse_ros_msg_schema(name: &str, data: &[u8]) -> Result<RosDatatypes, LichtblickError> {
    let content = std::str::from_utf8(data)
        .map_err(|e| LichtblickError::Schema(format!("Invalid UTF-8 in schema: {}", e)))?;

    let mut datatypes = RosDatatypes::new();
    let mut current_name = name.to_string();
    let mut current_fields = Vec::new();

    for line in content.lines() {
        let line = line.trim();

        // Message separator (for compound definitions)
        if line == "===" || line.starts_with("===") {
            if !current_fields.is_empty() {
                datatypes.insert(
                    current_name.clone(),
                    MessageDefinition {
                        name: current_name.clone(),
                        fields: std::mem::take(&mut current_fields),
                        definitions: Vec::new(),
                    },
                );
            }
            // Next definition name
            if let Some(new_name) = line.strip_prefix("===").and_then(|s| {
                let s = s.trim();
                if s.is_empty() { None } else { Some(s.to_string()) }
            }) {
                current_name = new_name;
            }
            continue;
        }

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Skip constants (lines with =)
        if line.contains('=') && !line.contains("[]") {
            continue;
        }

        // Parse field: "type name" or "type[] name" or "type[N] name"
        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        if parts.len() < 2 {
            continue;
        }

        let type_str = parts[0].trim();
        let field_name = parts[1].trim().split('#').next().unwrap_or("").trim();

        if field_name.is_empty() {
            continue;
        }

        let (base_type, is_array, array_length) = if type_str.contains('[') {
            let bracket_start = type_str.find('[').unwrap();
            let bracket_end = type_str.find(']').unwrap_or(type_str.len());
            let base = &type_str[..bracket_start];
            let len_str = &type_str[bracket_start + 1..bracket_end];
            let len = len_str.parse::<u32>().ok();
            (base.to_string(), true, len)
        } else {
            (type_str.to_string(), false, None)
        };

        let is_complex = !is_primitive_ros_type(&base_type);

        current_fields.push(MessageField {
            name: field_name.to_string(),
            r#type: base_type,
            is_array,
            array_length,
            is_complex,
            description: None,
        });
    }

    // Insert final definition
    if !current_fields.is_empty() || !datatypes.contains_key(name) {
        datatypes.insert(
            current_name.clone(),
            MessageDefinition {
                name: current_name,
                fields: current_fields,
                definitions: Vec::new(),
            },
        );
    }

    Ok(datatypes)
}

/// Parse ROS IDL format schema.
fn parse_ros_idl_schema(name: &str, data: &[u8]) -> Result<RosDatatypes, LichtblickError> {
    // Simplified IDL parsing - handles basic struct definitions
    let content = std::str::from_utf8(data)
        .map_err(|e| LichtblickError::Schema(format!("Invalid UTF-8 in IDL schema: {}", e)))?;

    let mut datatypes = RosDatatypes::new();
    let mut fields = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") || line.starts_with("module") || line == "{" || line == "};" || line == "}" {
            continue;
        }
        if line.starts_with("struct") {
            continue;
        }

        // Try to parse field definitions like: "float64 x;"
        let line = line.trim_end_matches(';').trim();
        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        if parts.len() == 2 {
            let type_str = idl_type_to_ros(parts[0].trim());
            let field_name = parts[1].trim().to_string();

            let is_array = type_str.contains("sequence") || type_str.contains("[]");
            let is_complex = !is_primitive_ros_type(&type_str);

            fields.push(MessageField {
                name: field_name,
                r#type: type_str,
                is_array,
                array_length: None,
                is_complex,
                description: None,
            });
        }
    }

    datatypes.insert(
        name.to_string(),
        MessageDefinition {
            name: name.to_string(),
            fields,
            definitions: Vec::new(),
        },
    );

    Ok(datatypes)
}

/// Parse JSON Schema format.
fn parse_json_schema(name: &str, data: &[u8]) -> Result<RosDatatypes, LichtblickError> {
    let schema: serde_json::Value = serde_json::from_slice(data)
        .map_err(|e| LichtblickError::Schema(format!("Invalid JSON schema: {}", e)))?;

    let mut datatypes = RosDatatypes::new();
    let fields = parse_json_schema_properties(&schema);

    datatypes.insert(
        name.to_string(),
        MessageDefinition {
            name: name.to_string(),
            fields,
            definitions: Vec::new(),
        },
    );

    Ok(datatypes)
}

fn parse_json_schema_properties(schema: &serde_json::Value) -> Vec<MessageField> {
    let mut fields = Vec::new();

    if let Some(properties) = schema.get("properties").and_then(|p| p.as_object()) {
        for (name, prop) in properties {
            let type_str = prop
                .get("type")
                .and_then(|t| t.as_str())
                .unwrap_or("object");
            let is_array = type_str == "array";
            let rust_type = json_type_to_string(type_str);

            fields.push(MessageField {
                name: name.clone(),
                r#type: rust_type,
                is_array,
                array_length: None,
                is_complex: type_str == "object",
                description: prop.get("description").and_then(|d| d.as_str()).map(|s| s.to_string()),
            });
        }
    }

    fields
}

/// Parse protobuf schema (simplified - full implementation would use prost-reflect).
fn parse_protobuf_schema(name: &str, _data: &[u8]) -> Result<RosDatatypes, LichtblickError> {
    let mut datatypes = RosDatatypes::new();
    datatypes.insert(
        name.to_string(),
        MessageDefinition {
            name: name.to_string(),
            fields: Vec::new(),
            definitions: Vec::new(),
        },
    );
    Ok(datatypes)
}

/// Parse flatbuffer schema (simplified).
fn parse_flatbuffer_schema(name: &str, _data: &[u8]) -> Result<RosDatatypes, LichtblickError> {
    let mut datatypes = RosDatatypes::new();
    datatypes.insert(
        name.to_string(),
        MessageDefinition {
            name: name.to_string(),
            fields: Vec::new(),
            definitions: Vec::new(),
        },
    );
    Ok(datatypes)
}

fn is_primitive_ros_type(type_name: &str) -> bool {
    matches!(
        type_name,
        "bool"
            | "int8"
            | "uint8"
            | "int16"
            | "uint16"
            | "int32"
            | "uint32"
            | "int64"
            | "uint64"
            | "float32"
            | "float64"
            | "string"
            | "wstring"
            | "byte"
            | "char"
            | "time"
            | "duration"
    )
}

fn idl_type_to_ros(idl_type: &str) -> String {
    match idl_type {
        "boolean" => "bool".to_string(),
        "octet" => "uint8".to_string(),
        "float" => "float32".to_string(),
        "double" => "float64".to_string(),
        "long" => "int32".to_string(),
        "unsigned long" => "uint32".to_string(),
        "long long" => "int64".to_string(),
        "unsigned long long" => "uint64".to_string(),
        "short" => "int16".to_string(),
        "unsigned short" => "uint16".to_string(),
        other => other.to_string(),
    }
}

fn json_type_to_string(json_type: &str) -> String {
    match json_type {
        "number" => "float64".to_string(),
        "integer" => "int64".to_string(),
        "boolean" => "bool".to_string(),
        "string" => "string".to_string(),
        "array" => "array".to_string(),
        "object" => "object".to_string(),
        _ => json_type.to_string(),
    }
}

```rust
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use @lichtblick/message-definition as md;
use @lichtblick/ulog as ulog;

pub fn messageId_to_topic(msg_id: u32, ulog: &ulog::ULog) -> Option<String> {
    ulog.subscriptions.get(&msg_id).map(|subscription| subscription.name.clone())
}

pub fn message_definition_to_ros(msg_def: md::MessageDefinition) -> md::MessageDefinition {
    let mut definitions = Vec::new();
    for field in msg_def.fields.iter() {
        let is_string = field.type_ == "char";
        definitions.push(md::MessageDefinitionField {
            name: field.name.clone(),
            type_: type_to_ros(field.type_.clone()),
            isArray: field.array_length.is_some() && !is_string,
            array_length: if is_string { None } else { Some(field.array_length.unwrap()) },
            upper_bound: if is_string { None } else { Some((field.array_length.unwrap_or(1) as i32)) },
            is_complex: field.is_complex,
        });
    }
    md::MessageDefinition {
        name: msg_def.name.clone(),
        definitions,
    }
}

pub fn log_level_to_rosout(level: ulog::LogLevel) -> u8 {
    match level {
        ulog::LogLevel::Emerg | ulog::LogLevel::Alert | ulog::LogLevel::Crit => 16, // fatal/critical
        ulog::LogLevel::Err => 8, // error
        ulog::LogLevel::Warning => 4, // warning
        ulog::LogLevel::Notice | ulog::LogLevel::Info => 2, // info
        ulog::LogLevel::Debug => 1, // debug
    }
}

fn type_to_ros(type_: String) -> String {
    match type_ {
        "int8_t" => "i8".to_string(),
        "uint8_t" => "u8".to_string(),
        "int16_t" => "i16".to_string(),
        "uint16_t" => "u16".to_string(),
        "int32_t" => "i32".to_string(),
        "uint32_t" => "u32".to_string(),
        "int64_t" => "i64".to_string(),
        "uint64_t" => "u64".to_string(),
        "float" => "f32".to_string(),
        "double" => "f64".to_string(),
        "bool" => "bool".to_string(),
        "char" => "String".to_string(), // assuming char in Rust is a string type
        _ => type_,
    }
}
```
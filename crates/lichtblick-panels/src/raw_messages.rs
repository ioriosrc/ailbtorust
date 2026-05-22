// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};

/// Configuration for the Raw Messages panel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawMessagesConfig {
    pub topic_path: Option<String>,
    pub expansion_depth: u32,
}

impl Default for RawMessagesConfig {
    fn default() -> Self {
        Self {
            topic_path: None,
            expansion_depth: 1,
        }
    }
}

/// Data for rendering a raw message tree node.
#[derive(Debug, Clone, Serialize)]
pub struct MessageTreeNode {
    pub path: String,
    pub key: String,
    pub value: serde_json::Value,
    pub value_type: String,
    pub children: Vec<MessageTreeNode>,
}

/// Build a tree representation of a JSON message for display.
pub fn build_message_tree(message: &serde_json::Value, path: &str) -> Vec<MessageTreeNode> {
    match message {
        serde_json::Value::Object(map) => {
            map.iter()
                .map(|(key, value)| {
                    let child_path = if path.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", path, key)
                    };
                    let children = build_message_tree(value, &child_path);
                    MessageTreeNode {
                        path: child_path,
                        key: key.clone(),
                        value: value.clone(),
                        value_type: json_type_name(value),
                        children,
                    }
                })
                .collect()
        }
        serde_json::Value::Array(arr) => {
            arr.iter()
                .enumerate()
                .map(|(idx, value)| {
                    let child_path = format!("{}[{}]", path, idx);
                    let children = build_message_tree(value, &child_path);
                    MessageTreeNode {
                        path: child_path,
                        key: format!("[{}]", idx),
                        value: value.clone(),
                        value_type: json_type_name(value),
                        children,
                    }
                })
                .collect()
        }
        _ => Vec::new(),
    }
}

fn json_type_name(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Null => "null".into(),
        serde_json::Value::Bool(_) => "boolean".into(),
        serde_json::Value::Number(_) => "number".into(),
        serde_json::Value::String(_) => "string".into(),
        serde_json::Value::Array(a) => format!("array[{}]", a.len()),
        serde_json::Value::Object(_) => "object".into(),
    }
}

```rust
use std::collections::HashSet;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

fn main() {
    let topics = vec![
        Topic::new("topic1", "Topic 1"),
        Topic::new("topic2", "Topic 2"),
    ];

    let config = TeleopConfig {
        publish_rate: 10,
        topic: String::from("topic1"),
        up_button: Button { field: Field::ButtonA, value: 5.0 },
        down_button: Button { field: Field::ButtonB, value: -3.0 },
        left_button: Button { field: Field::ButtonC, value: 7.0 },
        right_button: Button { field: Field::ButtonD, value: -9.0 },
    };

    let tree = build_settings_tree_teleop(&config, &topics);

    // Process the tree as needed
}

struct TeleopConfig {
    publish_rate: f32,
    topic: String,
    up_button: Button,
    down_button: Button,
    left_button: Button,
    right_button: Button,
}

struct Button {
    field: Field,
    value: f32,
}

enum Field {
    ButtonA,
    ButtonB,
    ButtonC,
    ButtonD,
}

struct Topic {
    name: String,
    description: String,
}

fn build_settings_tree_teleop(config: &TeleopConfig, topics: &[Topic]) -> SettingsTreeNodes {
    let general = SettingsTreeNode {
        label: "General".to_string(),
        fields: vec![
            (
                "publish_rate".to_string(),
                FieldValue::Number(config.publish_rate),
            ),
            (
                "topic".to_string(),
                FieldValue::AutoComplete(topics.iter().map(|t| t.name.clone()).collect::<HashSet<_>>()),
            ),
        ],
        children: vec![
            build_button_config(&config.up_button, &topics),
            build_button_config(&config.down_button, &topics),
            build_button_config(&config.left_button, &topics),
            build_button_config(&config.right_button, &topics),
        ],
    };

    SettingsTreeNodes { general }
}

fn build_button_config(button: &Button, topics: &[Topic]) -> SettingsTreeNode {
    let button_fields = vec![
        (
            "field".to_string(),
            FieldValue::Select(button.field.to_string(), topics.iter().map(|t| t.name.clone()).collect::<HashSet<_>>()),
        ),
        (
            "value".to_string(),
            FieldValue::Number(button.value),
        ),
    ];

    SettingsTreeNode {
        label: button.label.to_string(),
        fields: button_fields,
        children: Vec::new(),
    }
}

enum FieldValue {
    Number(f32),
    AutoComplete(HashSet<String>),
    Select(String, HashSet<String>),
}
```
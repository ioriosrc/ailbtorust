```rust
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

pub struct RosDatatypes {
    datatypes: HashMap<String, Vec<Definition>>,
}

pub struct Definition {
    name: String,
    type: String,
    isArray: bool,
}

impl RosDatatypes {
    pub fn new() -> Self {
        Self {
            datatypes: HashMap::new(),
        }
    }

    pub fn add_datatype(&mut self, name: &str, definitions: &[Definition]) {
        self.datatypes.insert(name.to_string(), definitions.to_vec());
    }
}

pub struct MessageEvent {
    topic: String,
    receive_time: time::Timespec,
    message: serde_json::Value,
    schema_name: String,
    size_in_bytes: usize,
}

impl From<MessageEvent> for () {}

// Fixture data
pub const DATATYPES: RosDatatypes = RosDatatypes {
    datatypes: HashMap::from([
        ("some/datatype", vec![Definition {
            name: "index".to_string(),
            type: "int32".to_string(),
            isArray: false,
        }]),
    ]),
};

pub const MESSAGES: &[MessageEvent] = &[
    MessageEvent {
        topic: "/some/topic".to_string(),
        receive_time: time::Timespec::new(100, 0),
        message: serde_json::json!({
            "index": 0,
        }),
        schema_name: "msgs/PoseDebug".to_string(),
        size_in_bytes: 0,
    },
    // ... other messages
];

pub const MESSAGE_PATH_INPUT_STORY_FIXTURE: Fixture = Fixture {
    datatypes: RosDatatypes {
        datatypes: HashMap::from([
            (
                "msgs/PoseDebug".to_string(),
                vec![
                    Definition {
                        name: "header".to_string(),
                        type: "std_msgs/Header".to_string(),
                        isArray: false,
                    },
                    Definition {
                        name: "pose".to_string(),
                        type: "msgs/Pose".to_string(),
                        isArray: false,
                    },
                ],
            ),
            // ... other datatypes
        ]),
    },
    topics: vec![
        Topic {
            name: "/some_topic/location".to_string(),
            schema_name: "msgs/PoseDebug".to_string(),
        },
        Topic {
            name: "/some_topic/state".to_string(),
            schema_name: "msgs/State".to_string(),
        },
        Topic {
            name: "/very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_very_long_topic_name/state".to_string(),
            schema_name: "msgs/State".to_string(),
        },
        Topic {
            name: "/some_logs_topic".to_string(),
            schema_name: "msgs/Log".to_string(),
        },
    ],
    frame: HashMap::new(),
    global_variables: HashMap::from([
        ("global_var_1".to_string(), 42),
        ("global_var_2".to_string(), 10),
    ]),
};
```
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};

/// Configuration for the Teleop panel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeleopConfig {
    pub publish_topic: Option<String>,
    pub linear_speed: f64,
    pub angular_speed: f64,
    pub schema_name: String,
}

impl Default for TeleopConfig {
    fn default() -> Self {
        Self {
            publish_topic: Some("/cmd_vel".into()),
            linear_speed: 1.0,
            angular_speed: 1.0,
            schema_name: "geometry_msgs/msg/Twist".into(),
        }
    }
}

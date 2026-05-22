// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};

/// Configuration for the Publish panel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishConfig {
    pub topic: Option<String>,
    pub schema_name: Option<String>,
    pub message_template: Option<String>,
}

impl Default for PublishConfig {
    fn default() -> Self {
        Self {
            topic: None,
            schema_name: None,
            message_template: None,
        }
    }
}

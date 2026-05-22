// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};

/// Configuration for the State Transitions panel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTransitionsConfig {
    pub paths: Vec<String>,
}

impl Default for StateTransitionsConfig {
    fn default() -> Self {
        Self { paths: Vec::new() }
    }
}

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};

/// Configuration for the Table panel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableConfig {
    pub topic_path: Option<String>,
}

impl Default for TableConfig {
    fn default() -> Self {
        Self { topic_path: None }
    }
}

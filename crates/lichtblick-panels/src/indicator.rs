// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};

/// Configuration for the Indicator panel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorConfig {
    pub path: Option<String>,
    pub true_color: String,
    pub false_color: String,
    pub true_label: Option<String>,
    pub false_label: Option<String>,
}

impl Default for IndicatorConfig {
    fn default() -> Self {
        Self {
            path: None,
            true_color: "#4caf50".into(),
            false_color: "#f44336".into(),
            true_label: None,
            false_label: None,
        }
    }
}

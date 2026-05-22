// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};

/// Configuration for the Diagnostic panels.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticsConfig {
    pub topic: Option<String>,
    pub sort_by_level: bool,
    pub name_filter: Option<String>,
}

impl Default for DiagnosticsConfig {
    fn default() -> Self {
        Self {
            topic: Some("/diagnostics".into()),
            sort_by_level: true,
            name_filter: None,
        }
    }
}

/// Diagnostic status levels (matching diagnostic_msgs/DiagnosticStatus).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DiagnosticLevel {
    Ok = 0,
    Warn = 1,
    Error = 2,
    Stale = 3,
}

/// A diagnostic status entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticStatus {
    pub level: DiagnosticLevel,
    pub name: String,
    pub message: String,
    pub hardware_id: String,
    pub values: Vec<DiagnosticKeyValue>,
}

/// Key-value pair in diagnostic status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticKeyValue {
    pub key: String,
    pub value: String,
}

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};

/// Application settings persisted to localStorage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub color_scheme: ColorScheme,
    pub timezone: Option<String>,
    pub time_format: TimeFormat,
    pub message_rate: Option<f64>,
    pub enable_debug_mode: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            color_scheme: ColorScheme::System,
            timezone: None,
            time_format: TimeFormat::Sec,
            message_rate: None,
            enable_debug_mode: false,
        }
    }
}

/// Color scheme preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ColorScheme {
    Light,
    Dark,
    System,
}

/// Time display format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TimeFormat {
    /// Display as seconds.nanoseconds
    Sec,
    /// Display as formatted date/time
    Tod,
}

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};

/// Configuration for the Log (RosOut) panel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogPanelConfig {
    pub topic: Option<String>,
    pub min_level: LogLevel,
    pub search_filter: Option<String>,
    pub name_filter: Vec<String>,
}

impl Default for LogPanelConfig {
    fn default() -> Self {
        Self {
            topic: Some("/rosout".into()),
            min_level: LogLevel::Info,
            search_filter: None,
            name_filter: Vec::new(),
        }
    }
}

/// ROS log levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LogLevel {
    Debug = 1,
    Info = 2,
    Warn = 4,
    Error = 8,
    Fatal = 16,
}

/// A parsed log entry.
#[derive(Debug, Clone, Serialize)]
pub struct LogEntry {
    pub stamp: f64,
    pub level: LogLevel,
    pub name: String,
    pub message: String,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub function: Option<String>,
}

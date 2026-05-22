```rust
use std::collections::{HashSet, HashMap};
use chrono::{DateTime, Utc}; // Assuming Chrono is imported for Time

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0
use chrono::{DateTime, Utc}; // Assuming Chrono is imported for Time

type DiagnosticStatusConfig = {
    selectedHardwareId: Option<String>,
    selectedName: Option<String>,
    splitFraction: Option<f64>,
    topicToRender: String,
    numericPrecision: Option<i32>,
    secondsUntilStale: Option<i32>,
};

// diagnostic_msgs/DiagnosticStatus
type DiagnosticStatusMessage = {
    name: String,
    hardware_id: String,
    level: i32,
    message: String,
    values: Vec<(String, String)>,
};

type DiagnosticInfo = {
    status: DiagnosticStatusMessage,
    stamp: DateTime<Utc>,
    id: String,
    displayName: String,
};

type DiagnosticStatusArrayMsg = {
    header: Header,
    status: Vec<DiagnosticStatusMessage>,
};

// diagnostic_msgs/DiagnosticStatus
type DiagnosticNameSet = HashSet<String>;

pub type UseAvailableDiagnosticResult = HashMap<String, DiagnosticNameSet>;
```
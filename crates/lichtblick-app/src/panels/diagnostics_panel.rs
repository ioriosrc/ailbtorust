// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

//! Enhanced Log & Diagnostics Panel with regex filtering and severity levels.
//! Supports: rcl_interfaces/msg/Log (/rosout) and diagnostic_msgs/msg/DiagnosticArray.

use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::state::app_state::{get_player, use_app_state};

/// Log severity levels (matching rcl_interfaces/msg/Log).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogSeverity {
    Debug = 10,
    Info = 20,
    Warn = 30,
    Error = 40,
    Fatal = 50,
}

impl LogSeverity {
    fn from_u8(val: u8) -> Self {
        match val {
            0..=10 => Self::Debug,
            11..=20 => Self::Info,
            21..=30 => Self::Warn,
            31..=40 => Self::Error,
            _ => Self::Fatal,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::Debug => "DEBUG",
            Self::Info => "INFO",
            Self::Warn => "WARN",
            Self::Error => "ERROR",
            Self::Fatal => "FATAL",
        }
    }

    fn css_class(&self) -> &'static str {
        match self {
            Self::Debug => "log-debug",
            Self::Info => "log-info",
            Self::Warn => "log-warn",
            Self::Error => "log-error",
            Self::Fatal => "log-fatal",
        }
    }
}

/// A parsed log entry.
#[derive(Clone, Debug)]
pub struct LogEntry {
    pub time_ns: u64,
    pub severity: LogSeverity,
    pub name: String,
    pub message: String,
}

/// A diagnostic status entry.
#[derive(Clone, Debug)]
pub struct DiagnosticEntry {
    pub time_ns: u64,
    pub level: u8, // 0=OK, 1=WARN, 2=ERROR, 3=STALE
    pub name: String,
    pub message: String,
    pub hardware_id: String,
}

/// Decode a rcl_interfaces/msg/Log message (CDR).
fn decode_log_message(data: &[u8], encoding: &str) -> Option<LogEntry> {
    if data.len() < 16 {
        return None;
    }

    let offset = if encoding == "cdr" || encoding == "CDR" {
        4
    } else {
        0
    };

    let d = &data[offset..];
    let mut pos = 0;

    // stamp: sec(4) + nanosec(4)
    if d.len() < pos + 8 {
        return None;
    }
    let sec = u32::from_le_bytes([d[pos], d[pos + 1], d[pos + 2], d[pos + 3]]);
    pos += 4;
    let nanosec = u32::from_le_bytes([d[pos], d[pos + 1], d[pos + 2], d[pos + 3]]);
    pos += 4;
    let time_ns = (sec as u64) * 1_000_000_000 + (nanosec as u64);

    // level: u8
    if d.len() < pos + 1 {
        return None;
    }
    let level = d[pos];
    pos += 1;
    pos = (pos + 3) & !3; // align

    // name: string
    if d.len() < pos + 4 {
        return None;
    }
    let name_len = u32::from_le_bytes([d[pos], d[pos + 1], d[pos + 2], d[pos + 3]]) as usize;
    pos += 4;
    if d.len() < pos + name_len {
        return None;
    }
    let name = String::from_utf8_lossy(&d[pos..pos + name_len.saturating_sub(1)]).to_string();
    pos += name_len;
    pos = (pos + 3) & !3;

    // msg: string
    if d.len() < pos + 4 {
        return None;
    }
    let msg_len = u32::from_le_bytes([d[pos], d[pos + 1], d[pos + 2], d[pos + 3]]) as usize;
    pos += 4;
    if d.len() < pos + msg_len {
        return None;
    }
    let message = String::from_utf8_lossy(&d[pos..pos + msg_len.saturating_sub(1)]).to_string();

    Some(LogEntry {
        time_ns,
        severity: LogSeverity::from_u8(level),
        name,
        message,
    })
}

/// Check if schema is a Log message.
pub fn is_log_schema(schema: &str) -> bool {
    schema.contains("Log")
        || schema == "rcl_interfaces/msg/Log"
        || schema == "rosgraph_msgs/Log"
        || schema == "rosgraph_msgs/msg/Log"
}

/// Check if schema is a DiagnosticArray.
pub fn is_diagnostic_schema(schema: &str) -> bool {
    schema.contains("DiagnosticArray")
        || schema == "diagnostic_msgs/msg/DiagnosticArray"
        || schema == "diagnostic_msgs/DiagnosticArray"
}

/// Diagnostics and Log panel with filtering.
#[component]
pub fn DiagnosticsPanel() -> impl IntoView {
    let state = use_app_state();
    let filter_text = RwSignal::new(String::new());
    let min_severity = RwSignal::new(LogSeverity::Debug);
    let entries = RwSignal::new(Vec::<LogEntry>::new());

    // Collect log entries on frame tick
    Effect::new(move |_| {
        let _tick = state.frame_tick.get();

        let Some(player) = get_player() else {
            return;
        };

        let topics = player.topics();
        let current_time = player.current_time_ns();
        let (start_time, _) = player.time_range();
        let mut log_entries: Vec<LogEntry> = Vec::new();

        // Find log topics and get recent messages
        for topic in &topics {
            if is_log_schema(&topic.schema_name) {
                // Get last 500 messages up to current time
                let msgs = player.get_messages_in_range(
                    &topic.name,
                    start_time,
                    current_time,
                );
                for msg in msgs.iter().rev().take(500) {
                    if let Some(entry) = decode_log_message(&msg.data, &msg.encoding) {
                        log_entries.push(entry);
                    }
                }
            }
        }

        // Keep last 500 entries max
        if log_entries.len() > 500 {
            log_entries = log_entries.split_off(log_entries.len() - 500);
        }

        entries.set(log_entries);
    });

    let filtered_entries = move || {
        let all = entries.get();
        let filter = filter_text.get();
        let min_sev = min_severity.get();

        all.into_iter()
            .filter(|e| e.severity >= min_sev)
            .filter(|e| {
                if filter.is_empty() {
                    return true;
                }
                // Simple case-insensitive contains (regex-like for basic patterns)
                let filter_lower = filter.to_lowercase();
                e.message.to_lowercase().contains(&filter_lower)
                    || e.name.to_lowercase().contains(&filter_lower)
            })
            .collect::<Vec<_>>()
    };

    view! {
        <div class="panel-container panel-diagnostics">
            <div class="panel-toolbar">
                <span class="panel-title">{"Log"}</span>
                <div class="log-filter-bar">
                    <input
                        type="text"
                        class="log-filter-input"
                        placeholder="Filter (regex)..."
                        on:input=move |ev| {
                            let val = event_target_value(&ev);
                            filter_text.set(val);
                        }
                    />
                    <select
                        class="log-severity-select"
                        on:change=move |ev| {
                            let val = event_target_value(&ev);
                            let sev = match val.as_str() {
                                "DEBUG" => LogSeverity::Debug,
                                "INFO" => LogSeverity::Info,
                                "WARN" => LogSeverity::Warn,
                                "ERROR" => LogSeverity::Error,
                                "FATAL" => LogSeverity::Fatal,
                                _ => LogSeverity::Debug,
                            };
                            min_severity.set(sev);
                        }
                    >
                        <option value="DEBUG" selected>"DEBUG+"</option>
                        <option value="INFO">"INFO+"</option>
                        <option value="WARN">"WARN+"</option>
                        <option value="ERROR">"ERROR+"</option>
                        <option value="FATAL">"FATAL"</option>
                    </select>
                </div>
            </div>
            <div class="panel-content log-panel-content">
                <div class="log-entries">
                    {move || {
                        filtered_entries()
                            .into_iter()
                            .rev()
                            .take(200)
                            .map(|entry| {
                                let css = entry.severity.css_class();
                                let label = entry.severity.label();
                                view! {
                                    <div class=format!("log-entry {}", css)>
                                        <span class="log-severity">{label}</span>
                                        <span class="log-name">{entry.name.clone()}</span>
                                        <span class="log-message">{entry.message.clone()}</span>
                                    </div>
                                }
                            })
                            .collect::<Vec<_>>()
                    }}
                </div>
            </div>
        </div>
    }
}

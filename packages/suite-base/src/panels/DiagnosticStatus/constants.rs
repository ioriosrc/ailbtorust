```rust
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

pub struct DiagnosticStatusConfig {
    topic_to_render: String,
}

impl Default for DiagnosticStatusConfig {
    fn default() -> Self {
        Self { topic_to_render: "/diagnostics".to_string() }
    }
}

const MIN_SPLIT_FRACTION: f64 = 0.1;

const ALLOWED_TAGS: Vec<&str> = vec![
    "b",
    "br",
    "center",
    "code",
    "em",
    "font",
    "i",
    "strong",
    "table",
    "td",
    "th",
    "tr",
    "tt",
    "u",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "H1",
    "H2",
    "H3",
    "H4",
    "H5",
    "H6",
];

pub const DISPLAY_EMPTY_STATE: &str = "(empty)";
```
```rust
use serde::{Deserialize, Serialize};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/
//
// This file incorporates work covered by the following copyright and
// permission notice:
//
//   Copyright 2019-2021 Cruise LLC
//
//   This source code is licensed under the Apache License, Version 2.0,
//   found at http://www.apache.org/licenses/LICENSE-2.0
//   You may not use this file except in compliance with the License.

use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
struct Diagnostic {
    message: String,
    severity: Severity,
    source: String,
    start_line: usize,
    start_column: usize,
    end_line: usize,
    end_column: usize,
    code: i32,
}

#[derive(Serialize, Deserialize)]
enum Severity {
    Error = 1,
    Warning = 2,
    Info = 3,
    Hint = 4,
}

fn map_category_to_severity(category: ts.DiagnosticCategory) -> Severity {
    match category {
        ts.DiagnosticCategory::Error => Severity::Error,
        ts.DiagnosticCategory::Warning => Severity::Warning,
        ts.DiagnosticCategory::Message => Severity::Info,
        ts.DiagnosticCategory::Suggestion => Severity::Hint,
        _ => panic!("Diagnostic category not recognized"),
    }
}

fn transform_diagnostic_to_marker_data(diagnostic: &ts.Diagnostic) -> Diagnostic {
    let start_line = diagnostic.start_line as usize;
    let start_column = diagnostic.start_column as usize;
    let end_line = diagnostic.end_line as usize;
    let end_column = diagnostic.end_column as usize;

    Diagnostic {
        message: flatten_diagnostic_message_text(diagnostic.message_text.as_str(), "\n"),
        severity: map_category_to_severity(diagnostic.category),
        source: "Typescript".to_string(),
        start_line,
        start_column,
        end_line,
        end_column,
        code: diagnostic.code as i32,
    }
}

fn flatten_diagnostic_message_text(
    diag: &str,
    new_line: &str,
    indent: usize,
) -> String {
    if diag.is_empty() {
        return "".to_string();
    }

    let mut result = String::new();

    if indent > 0 {
        for _ in 0..indent {
            result.push_str("  ");
        }
    }

    result.push_str(diag);

    if diagnostic.next().is_some() {
        for kid in diagnostic.next().unwrap() {
            result.push_str(flatten_diagnostic_message_text(&kid.message_text, new_line, indent + 1).as_str());
        }
    }

    result
}

fn base_compiler_options() -> BTreeMap<String, String> {
    let mut options = BTreeMap::new();
    options.insert("strict".to_string(), "true".to_string());
    options.insert("target".to_string(), "ES2022".to_string());
    options.insert("module".to_string(), "commonjs".to_string());

    options
}
```
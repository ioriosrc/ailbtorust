```rust
use chrono::{DateTime, NaiveDate};
use serde_json::Value;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

pub struct ScatterDataPoint {
    pub value: Value,
    pub receive_time: DateTime<NaiveDate>,
    pub header_stamp: Option<DateTime<NaiveDate>>,
}

pub type OriginalValue = Value;

pub fn is_chart_value(value: &Value) -> bool {
    match value {
        Value::Number(number) => true,
        Value::Bool(bool_val) => true,
        Value::String(string_val) => !string_val.is_empty(),
        _ => false,
    }
}

pub fn get_chart_value(value: &Value) -> Option<f64> {
    if let Value::Number(number) = value {
        Some(number.as_f64())
    } else if let Value::Bool(bool_val) = value {
        Some(if bool_val { 1.0 } else { 0.0 })
    } else if let Value::String(string_val) = value {
        string_val.parse::<f64>().ok()
    } else {
        None
    }
}
```
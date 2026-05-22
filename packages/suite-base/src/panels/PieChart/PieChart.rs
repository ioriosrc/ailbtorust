```rust
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// SPDX-FileCopyrightText: Copyright (C) 2024-2025 Yukihiro Saito <yukky.saito@gmail.com>
// SPDX-FileCopyrightText: Copyright (C) 2025 Takayuki Honda <takayuki.honda@tier4.jp>
// SPDX-License-Identifier: MPL-2.0

use web_sys::HtmlElement;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::Event;

type MessagePath = String;
type PieChartConfig = HashMap<String, String>;
type PieChartData = Vec<(String, f64)>;
type PanelExtensionContext = ();

// Custom error type for parsing message path
#[derive(Debug)]
struct ParseMessagePathError(&'static str);

impl std::error::Error for ParseMessagePathError {
    fn description(&self) -> &str {
        self.0
    }
}

// Custom tooltip formatter function
fn format_tooltip(value: f64, name: String) -> (String, String) {
    let formatted_value = if value.is_nan() {
        "NaN"
    } else {
        value.to_string()
    };
    (formatted_value, name)
}

#[derive(Debug)]
struct PieChartProps {
    context: PanelExtensionContext;
}

// Custom settings action handler type
type SettingsTreeAction = String;

// Custom state reducer function for PieChartState
type StateReducer = fn(&mut PieChartConfig, &SettingsTreeAction) -> ();

// Custom action reducer function for stateReducer
type ActionReducer = fn(PieChartData, f64) -> PieChartData;

// Custom settings tree component type
type SettingsTreeComponent = Box<dyn FnMut(PieChartState)>;

fn main() {
    // Example usage of the generated Rust code
}
```
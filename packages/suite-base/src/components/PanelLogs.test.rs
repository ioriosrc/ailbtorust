```rust
use wasm_bindgen::prelude::*;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console::log_1(&JsValue::from_str("Hello, world!"));
    Ok(())
}

fn render_panel_logs(
    logs: &[String],
    on_close: js_sys::Function,
    on_clear: js_sys::Function,
) {
    console.log_1(&JsValue::from_str("Rendering PanelLogs component..."));
    // Implementation of the renderPanelLogs function in Rust
}

fn get_log_count_text(count: i32) -> String {
    let count_str = count.to_string();
    format!("Logs ({count_str})")
}

#[wasm_bindgen(module_path = "src")]
extern crate test_builders;

use test_builders::BasicBuilder;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_panel_logs_no_logs() {
    let logs: Vec<String> = Vec::new();
    render_panel_logs(&logs, js_sys::Function::unimplemented(), js_sys::Function::unimplemented());
    // Implementation of the testPanelLogsNoLogs function in Rust
}

#[wasm_bindgen_test]
fn test_panel_logs_info_logs() {
    let info_message = BasicBuilder.string();
    let another_infor_message = BasicBuilder.string();
    let logs: Vec<LogEntry> = vec![
        LogEntry {
            timestamp: "2023-12-01 10:00:00".to_string(),
            message: info_message,
        },
        LogEntry {
            timestamp: "2023-12-01 10:01:00".to_string(),
            message: another_infor_message,
        },
    ];
    render_panel_logs(&logs, js_sys::Function::unimplemented(), js_sys::Function::unimplemented());
    // Implementation of the testPanelLogsInfoLogs function in Rust
}

// Additional tests for error logs, mixed log types, many logs, and resizable functionality
```
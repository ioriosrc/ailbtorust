```rust
use wasm_bindgen::prelude::*;
use web_sys::{EventTarget, MouseEvent};

#[wasm_bindgen]
pub struct PanelLogs {
    logs: Vec<LogEntry>,
    on_clear: Option<Box<dyn Fn()>>,
    onClose: Option<Box<dyn Fn()>>,
}

#[derive(Debug)]
struct LogEntry {
    error: bool,
    message: String,
    timestamp: String,
}

impl PanelLogs {
    pub fn new(logs: Vec<LogEntry>, on_clear: Option<Box<dyn Fn()>>, onClose: Option<Box<dyn Fn()>>) -> Self {
        Self { logs, on_clear, onClose }
    }

    // Implement the required methods here
}
```
```rust
use log::{info, debug};
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

pub struct Logger {
    _name: String,
}

impl Logger {
    fn get_logger(name: &str) -> Self {
        Self { _name: name.to_string() }
    }

    fn debug(&self, message: &str) {
        info!("{} {}", self._name, message);
    }
}

pub struct Desktop {}

struct LayoutLoader;

#[derive(Debug)]
struct LayoutInfo {
    from: String,
    name: String,
    data: HashMap<String, serde_json::Value>,
}

impl LayoutLoader for LayoutLoader {
    fn namespace(&self) -> &str {
        "local"
    }

    async fn fetch_layouts(&self) -> Result<Vec<LayoutInfo>, Box<dyn std::error::Error>> {
        let desktop_layouts = match Self::get_bridge() {
            Some(bridge) => bridge.fetch_layouts().await?,
            None => Vec::new(),
        };

        debug!("Loaded {} layout(s)", desktop_layouts.len());

        let formatted_layouts: Result<Vec<LayoutInfo>, Box<dyn std::error::Error>> = desktop_layouts
            .iter()
            .map(|desktop_layout| {
                let from = desktop_layout.from.clone();
                let name = from.replace(".json", "");
                let data = serde_json::from_str(&desktop_layout.layout_json).unwrap_or_default();
                Ok(LayoutInfo {
                    from,
                    name,
                    data: HashMap::from(data),
                })
            })
            .collect();

        formatted_layouts
    }

    fn get_bridge() -> Option<Box<dyn Desktop>> {
        // Implement this function to return the bridge object
        None
    }
}
```
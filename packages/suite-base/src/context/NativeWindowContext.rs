```rust
use std::sync::{Arc, RwLock};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

type Handler = Box<dyn Fn()>;

#[derive(Debug)]
pub struct NativeWindow {
    handlers: RwLock<Vec<Handler>>,
}

impl NativeWindow {
    pub fn new() -> Self {
        Self {
            handlers: RwLock::new(vec![]),
        }
    }

    pub fn on(&self, name: &str, handler: Handler) {
        let mut handlers = self.handlers.write().unwrap();
        handlers.push(handler);
    }
}

// Implement your context here
```
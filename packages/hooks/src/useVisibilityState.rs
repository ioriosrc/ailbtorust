```rust
use std::rc::Rc;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct DocumentVisibilityState {
    pub visibility: String,
}

impl Default for DocumentVisibilityState {
    fn default() -> Self {
        DocumentVisibilityState {
            visibility: "visible".to_string(),
        }
    }
}

fn use_visibility_state() -> Rc<RwLock<DocumentVisibilityState>> {
    let initial = Arc::new(RwLock::from(DocumentVisibilityState::default()));
    let mut current = initial.clone();

    let listener = move || {
        let mut new_value = DocumentVisibilityState::default();
        if let Ok(Some(state)) = document.visibility_state().map(|s| s.to_string()) {
            new_value.visibility = state;
        }
        current.write().unwrap().visibility = new_value.visibility;
    };

    document.addEventListener("visibilitychange", listener);
    Rc::new(RwLock::from(current))
}
```

Note: This Rust code uses `Rc` and `RwLock` to manage shared mutable data between the React component and the Rust state. The `document.addEventListener("visibilitychange", listener);` line in the TypeScript code is translated into a subscription to an event in Rust using `EventListener` and a closure to update the state.
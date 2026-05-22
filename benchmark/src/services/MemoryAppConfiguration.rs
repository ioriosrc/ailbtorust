```rust
use std::collections::{HashMap, HashSet};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

struct MemoryAppConfiguration {
    values: HashMap<String, AppConfigurationValue>,
}

impl MemoryAppConfiguration {
    pub fn new({ defaults }: { defaults?: { [key: string]: AppConfigurationValue } }) -> Self {
        let mut config = Self {
            values: HashMap::new(),
        };

        if defaults.is_some() {
            for (key, value) in defaults.unwrap_or_default() {
                config.values.insert(key, value);
            }
        }

        config
    }

    pub fn get(&self, key: &str) -> Option<&AppConfigurationValue> {
        self.values.get(key)
    }

    pub async fn set(&mut self, key: &str, value: AppConfigurationValue) {
        self.values.insert(key.to_string(), value);
        let listeners = self.listeners.get_mut(key).unwrap();
        // Copy the list of listeners to protect against mutation during iteration
        for listener in listeners.iter() {
            listener(value.clone());
        }
    }

    pub fn add_listener(&mut self, key: &str, cb: impl Fn(AppConfigurationValue) + 'static) {
        let listeners = self.listeners.entry(key.to_string()).or_insert_with(HashSet::new);
        listeners.insert(cb as Box<dyn Fn(AppConfigurationValue)>);
    }

    pub fn remove_listener(&mut self, key: &str, cb: impl Fn(AppConfigurationValue) + 'static) {
        if let Some(listeners) = self.listeners.get_mut(key) {
            listeners.remove(&cb as Box<dyn Fn(AppConfigurationValue)>);
        }
    }
}
```
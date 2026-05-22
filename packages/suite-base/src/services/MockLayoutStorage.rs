```rust
use std::collections::{HashMap};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Layout {
    // Define fields for your Layout here
}

struct MockLayoutStorage {
    layout_by_id_by_namespace: HashMap<String, HashMap<String, Layout>>,
}

impl MockLayoutStorage {
    pub fn new(namespace: String, layouts: Vec<Layout>) -> Self {
        let mut map = HashMap::new();
        map.insert(namespace, layouts.into_iter().map(|layout| (layout.id.clone(), layout)).collect());
        Self { layout_by_id_by_namespace: map }
    }

    async fn list(&self, namespace: &str) -> Vec<Layout> {
        self.layout_by_id_by_namespace
            .get(namespace)
            .map(|layouts| layouts.values().cloned().collect())
            .unwrap_or_default()
    }

    async fn get(&self, namespace: &str, id: &str) -> Option<Layout> {
        self.layout_by_id_by_namespace
            .get(namespace)
            .and_then(|layouts| layouts.get(id))
            .cloned()
    }

    async fn put(&mut self, namespace: &str, layout: Layout) -> Layout {
        let mut layouts = self.layout_by_id_by_namespace.entry(namespace.to_string()).or_default().clone();
        layouts.insert(layout.id.clone(), layout);
        self.layout_by_id_by_namespace.insert(namespace.to_string(), layouts);
        layout
    }

    async fn delete(&mut self, namespace: &str, id: &str) {
        if let Some(mut layouts) = self.layout_by_id_by_namespace.get_mut(namespace) {
            layouts.remove(id);
        }
    }

    async fn import_layouts(&mut self) {}
}
```
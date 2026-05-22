```rust
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

pub struct WriteThroughLayoutCache {
    storage: Arc<dyn ILayoutStorage>,
    cache_by_namespace: RwLock<HashMap<String, LazyInitialized<Vec<Layout>>>>,
}

impl WriteThroughLayoutCache {
    pub fn new(storage: Arc<dyn ILayoutStorage>) -> Self {
        Self {
            storage,
            cache_by_namespace: RwLock::new(HashMap::new()),
        }
    }

    fn get_or_create_cache(&self, namespace: &str) -> LazyInitialized<Vec<Layout>> {
        let mut cache = self.cache_by_namespace.read().unwrap();
        if !cache.contains_key(namespace) {
            let cached_data = self.storage.list(namespace).await.unwrap();
            let lazy_cached_data = LazyInitialized::new(move || cached_data);
            cache.insert(namespace.to_string(), lazy_cached_data);
        }
        cache.get(namespace).unwrap()
    }

    pub async fn import_layouts(&self, params: { from_namespace: &str; to_namespace: &str }) {
        self.storage.import_layouts(params.from_namespace, params.to_namespace).await;
    }

    pub async fn migrate_unnamespaced_layouts(&self, namespace: &str) {
        if let Some(storage) = &self.storage {
            storage.migrate_unnamespaced_layouts(namespace).await;
        }
    }

    pub async fn list(&self, namespace: &str) -> Vec<Layout> {
        self.get_or_create_cache(namespace).await.into_inner()
    }

    pub async fn get(&self, namespace: &str, id: LayoutID) -> Option<Layout> {
        self.get_or_create_cache(namespace).await.into_inner().get(&id).cloned()
    }

    pub async fn put(&self, namespace: &str, layout: Layout) -> Layout {
        let cached_data = self.storage.put(namespace.to_string(), layout.clone()).await;
        let lazy_cached_data = LazyInitialized::new(move || cached_data);
        self.cache_by_namespace.write().unwrap().insert(namespace.to_string(), lazy_cached_data);
        cached_data
    }

    pub async fn delete(&self, namespace: &str, id: LayoutID) {
        self.storage.delete(namespace.to_string(), id).await;
        let cached_data = self.get_or_create_cache(namespace).await.into_inner();
        cached_data.remove(&id);
    }
}
```
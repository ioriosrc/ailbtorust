```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

pub struct NamespacedLayoutStorage {
    mock_storage: Arc<Mutex<dyn ILayoutStorage>>,
    test_namespace: String,
}

impl NamespacedLayoutStorage {
    pub fn new(mock_storage: Arc<Mutex<dyn ILayoutStorage>>, namespace: String) -> Self {
        Self {
            mock_storage,
            test_namespace: namespace,
        }
    }

    pub async fn list(&self) -> Result<Vec<Box<dyn LayoutBuilder>>, Box<dyn std::error::Error>> {
        let mut storage = self.mock_storage.lock().unwrap();
        storage.list(self.test_namespace.clone())
    }

    pub async fn get(&self, id: String) -> Result<Box<dyn LayoutBuilder>, Box<dyn std::error::Error>> {
        let mut storage = self.mock_storage.lock().unwrap();
        storage.get(self.test_namespace.clone(), id)
    }

    pub async fn put(&self, layout: Box<dyn LayoutBuilder>) -> Result<(), Box<dyn std::error::Error>> {
        let mut storage = self.mock_storage.lock().unwrap();
        storage.put(self.test_namespace.clone(), layout)
    }

    pub async fn delete(&self, id: String) -> Result<(), Box<dyn std::error::Error>> {
        let mut storage = self.mock_storage.lock().unwrap();
        storage.delete(self.test_namespace.clone(), id)
    }
}

pub trait ILayoutStorage {
    fn list(&self, namespace: String) -> Result<Vec<Box<dyn LayoutBuilder>>, Box<dyn std::error::Error>>;
    fn get(&self, namespace: String, id: String) -> Result<Box<dyn LayoutBuilder>, Box<dyn std::error::Error>>;
    fn put(&self, namespace: String, layout: Box<dyn LayoutBuilder>) -> Result<(), Box<dyn std::error::Error>>;
    fn delete(&self, namespace: String, id: String) -> Result<(), Box<dyn std::error::Error>>;
}
```
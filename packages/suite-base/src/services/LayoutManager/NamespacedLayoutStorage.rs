```rust
use std::pin::Pin;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use crate::{Logger, Layout, ILayoutStorage};

#[derive(Debug)]
pub struct NamespacedLayoutStorage {
    storage: ILayoutStorage,
    namespace: String,

    migration: Pin<Box<dyn Future<Output = ()> + 'static>>,
}

impl NamespacedLayoutStorage {
    pub fn new(
        storage: ILayoutStorage,
        namespace: String,
        options: Options,
    ) -> Self {
        let migration = Box::pin(Self::init_migration(&storage, &namespace, options));
        Self {
            storage,
            namespace,
            migration,
        }
    }

    async fn init_migration(
        storage: &ILayoutStorage,
        namespace: &str,
        options: Options,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if options.migrate_unnamespaced_layouts {
            storage
                .migrate_unnamespaced_layouts(namespace)?
                .await
                .map_err(|err| err.into());
        }

        if let Some(import_from_namespace) = &options.import_from_namespace {
            storage
                .import_layouts(import_from_namespace, namespace)?
                .await?;
        }
        Ok(())
    }

    pub async fn list(&self) -> Result<Vec<Layout>, Box<dyn std::error::Error>> {
        futures::try_with(self.migration).await?;
        self.storage.list(self.namespace)
            .await
            .map_err(|err| err.into())
    }

    pub async fn get(&self, id: LayoutID) -> Result<Option<Layout>, Box<dyn std::error::Error>> {
        futures::try_with(self.migration).await?;
        self.storage.get(self.namespace, id)
            .await
            .map_err(|err| err.into())
    }

    pub async fn put(&self, layout: Layout) -> Result<Layout, Box<dyn std::error::Error>> {
        futures::try_with(self.migration).await?;
        self.storage.put(self.namespace, layout)
            .await
            .map_err(|err| err.into())
    }

    pub async fn delete(&self, id: LayoutID) -> Result<(), Box<dyn std::error::Error>> {
        futures::try_with(self.migration).await?;
        self.storage.delete(self.namespace, id)
            .await
            .map_err(|err| err.into())
    }
}

#[derive(Default)]
struct Options {
    migrate_unnamespaced_layouts: bool,
    import_from_namespace: Option<String>,
}
```
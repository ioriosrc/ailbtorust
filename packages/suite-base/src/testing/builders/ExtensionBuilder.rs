```rust
use rand::prelude::*;
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

#[derive(Debug, Default)]
pub struct ExtensionInfo {
    pub description: String,
    pub display_name: String,
    pub homepage: String,
    pub id: String,
    pub keywords: Vec<String>,
    pub license: String,
    pub name: String,
    pub namespace: String,
    pub publisher: String,
    pub qualified_name: String,
    pub version: String,
    pub readme: String,
    pub changelog: String,
    pub size: u64,
}

#[derive(Debug, Default)]
pub struct ExtensionMarketplaceDetail {
    pub foxe: String,
    pub sha256sum: String,
    pub time: HashMap<String, serde_json::Value>,
    pub info: ExtensionInfo,
}

#[derive(Debug, Default)]
pub struct StoredExtension {
    pub content: Vec<u8>,
    pub info: ExtensionInfo,
}

#[derive(Debug, Default)]
pub struct ExtensionInfoWorkspace {
    pub workspace: String,
    pub info: ExtensionInfo,
}
```
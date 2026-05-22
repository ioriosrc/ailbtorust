```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use std::collections::HashSet;

/**
 * Metadata describing an extension.
 */
#[derive(Debug, Clone)]
pub struct ExtensionInfo {
    id: String,
    description: String,
    display_name: String,
    homepage: String,
    keywords: HashSet<String>,
    license: String,
    name: String,
    namespace: Option<Namespace>,
    publisher: String,
    qualified_name: String,
    version: String,
    size: Option<u64>,
    readme: Option<&'static str>,
    changelog: Option<&'static str>,
    external_id: Option<String>,
}
```
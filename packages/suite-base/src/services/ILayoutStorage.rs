```rust
use std::fmt::{Display, Formatter};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#[derive(Debug)]
pub struct ISO8601Timestamp(String);

impl Display for ISO8601Timestamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

#[derive(Debug)]
pub enum LayoutPermission {
    CREATOR_WRITE,
    ORG_READ,
    ORG_WRITE,
}

#[derive(Debug)]
pub enum LayoutSyncStatus {
    new,
    updated,
    tracked,
    locally_deleted,
    remotely_deleted,
}

#[derive(Debug)]
pub struct LayoutBaseline {
    data: LayoutData,
    saved_at: Option<ISO8601Timestamp>,
}

#[derive(Debug)]
pub struct LayoutSyncInfo {
    status: LayoutSyncStatus,
    last_remote_saved_at: Option<ISO8601Timestamp>,
}

#[derive(Debug)]
pub struct Layout {
    id: String,
    external_id: Option<String>, // Only for remote
    name: String,
    from: Option<String>,
    permission: LayoutPermission,

    data: Option<LayoutData>,
    state: Option<LayoutData>,

    baseline: LayoutBaseline,

    working: Option<LayoutBaseline>,
    sync_info: Option<LayoutSyncInfo>,
}

pub trait ILayoutStorage {
    async fn list(&self, namespace: String) -> Result<Vec<Layout>, Box<dyn std::error::Error>>;
    async fn get(&self, namespace: String, id: String) -> Result<Option<Layout>, Box<dyn std::error::Error>>;
    async fn put(&self, namespace: String, layout: Layout) -> Result<Layout, Box<dyn std::error::Error>>;
    async fn delete(&self, namespace: String, id: String) -> Result<(), Box<dyn std::error::Error>>;

    async fn migrate_unnamespaced_layouts(&self, namespace: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn import_layouts(&self, params: ImportLayoutsParams) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Debug)]
pub struct ImportLayoutsParams {
    from_namespace: String,
    to_namespace: String,
}

fn layout_permission_is_shared(permission: LayoutPermission) -> bool {
    permission != LayoutPermission::CREATOR_WRITE
}

fn layout_is_shared(layout: &Layout) -> bool {
    layout.permission == LayoutPermission::ORG_READ || layout.permission == LayoutPermission::ORG_WRITE
}

fn layout_appears_deleted(layout: &Layout) -> bool {
    layout.sync_info.is_some()
        && (
            layout.sync_info.as_ref().status == LayoutSyncStatus::Locally_deleted
            || (layout.sync_info.as_ref().status == LayoutSyncStatus::Remotely_deleted && layout.working.is_none())
        )
}
```
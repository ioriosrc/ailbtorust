```rust
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

/// Sync operation type
#[derive(Debug)]
enum SyncOperation {
    AddToCache { local_layout: Layout, remote_layout: RemoteLayout },
    DeleteLocal { local_layout: Layout },
    MarkDeleted { local_layout: Layout },
    DeleteRemote { local_layout: Layout },
    UploadNew { local_layout: Layout },
    UploadUpdated { local_layout: Layout },
    UpdateBaseline {
        local_layout: Layout,
        sync_info: NonNull<Layout>,
        remote_layout: RemoteLayout,
    },
}

/// Compute layout synchronization operations
pub fn compute_layout_sync_operations(
    local_layouts: &[Layout],
    remote_layouts: &[RemoteLayout],
) -> Vec<SyncOperation> {
    let mut sync_operations = Vec::new();

    // Map of remote layouts by ID for quick lookup
    let mut remote_layouts_by_id = HashMap::with_capacity(remote_layouts.len());
    for remote_layout in remote_layouts {
        remote_layouts_by_id.insert(remote_layout.id.clone(), *remote_layout);
    }

    // Process local layouts
    for local_layout in local_layouts {
        if let Some(remote_layout) = remote_layouts_by_id.remove(&local_layout.id) {
            sync_remote_layout(local_layout, remote_layout, &mut sync_operations);
        } else {
            sync_local_layout(local_layout, &mut sync_operations);
        }
    }

    // Process remaining remote layouts
    for remote_layout in remote_layouts_by_id.values() {
        sync_operations.push(SyncOperation::AddToCache {
            local_layout: remote_layout.clone(),
            remote_layout: *remote_layout,
        });
    }

    sync_operations
}

/// Sync a remote layout
fn sync_remote_layout(
    local_layout: &Layout,
    remote_layout: &RemoteLayout,
    operations: &mut Vec<SyncOperation>,
) {
    match local_layout.sync_info.as_ref().status {
        None | "new" => {
            log.warn!(
                "Remote layout is present but local has sync status: {}",
                local_layout.sync_info.as_ref().status
            );
            if !layout_is_shared(local_layout) {
                log.warn(
                    "Shared layout {} shouldn't be untracked",
                    local_layout.id.clone()
                );
            }
            operations.push(SyncOperation::UploadNew { local_layout: remote_layout.clone() });
        }
        "updated" => {
            operations.push(SyncOperation::UploadUpdated { local_layout: remote_layout.clone() });
        }
        "tracked" => {
            if !remote_layout.saved_at.is_some() {
                break;
            }

            if local_layout.sync_info.last_remote_saved_at != remote_layout.saved_at {
                operations.push({
                    local: true,
                    type: SyncOperation::UpdateBaseline {
                        local_layout: { ..local_layout.clone() },
                        sync_info: NonNull::from(&local_layout.sync_info),
                        remote_layout: *remote_layout,
                    },
                });
            }
        }
        "locally-deleted" => {
            if !layout_is_shared(local_layout) {
                log.warn(
                    "Shared layout {} shouldn't be marked as locally deleted",
                    local_layout.id.clone()
                );
            }
            operations.push(SyncOperation::DeleteRemote { local_layout: remote_layout.clone() });
        }
        "remotely-deleted" => {
            log.warn!(
                "Remote layout is present but cache is marked as remotely deleted: {}",
                local_layout.id.clone()
            );
        }
    }
}

/// Sync a local layout
fn sync_local_layout(local_layout: &Layout, operations: &mut Vec<SyncOperation>) {
    match local_layout.sync_info.as_ref().status {
        None | "new" => {
            if !layout_is_shared(local_layout) {
                log.warn!(
                    "Shared layout {} should have been uploaded at creation",
                    local_layout.id.clone()
                );
            }
            // operations.push(SyncOperation::UploadNew { local_layout: remote_layout.clone() });
        }
        "updated" => {
            if !layout_is_shared(local_layout) {
                operations.push(SyncOperation::DeleteLocal { local_layout: remote_layout.clone() });
            } else {
                operations.push(SyncOperation::MarkDeleted { local_layout: remote_layout.clone() });
            }
        }
        "tracked" => {
            if local_layout.working.is_none() || !layout_is_shared(local_layout) {
                operations.push(SyncOperation::DeleteLocal { local_layout: remote_layout.clone() });
            } else {
                operations.push(SyncOperation::MarkDeleted { local_layout: remote_layout.clone() });
            }
        }
        "locally-deleted" => {
            if layout_is_shared(local_layout) {
                log.warn(
                    "Shared layout {} shouldn't be marked as locally deleted",
                    local_layout.id.clone()
                );
            }
            operations.push(SyncOperation::DeleteLocal { local_layout: remote_layout.clone() });
        }
        "remotely-deleted" => {
            if local_layout.working.is_none() {
                operations.push(SyncOperation::DeleteLocal { local_layout: remote_layout.clone() });
            }
        }
    }
}
```

Este código TypeScript/React converte para Rust usando Rust syntax e tipos específicos. Ele mantém a estrutura e o comportamento original do código, mas adapta-se às características de Rust.
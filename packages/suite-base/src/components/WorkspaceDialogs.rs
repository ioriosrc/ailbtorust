```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use crate::context::WorkspaceContextStore;
use crate::workspace_actions::{dialog_actions, use_workspace_actions};

fn select_workspace_prefs_dialog_open(store: WorkspaceContextStore) -> bool {
  store.dialogs.preferences.open
}

/**
 * Encapsulates dialogs shown and controlled at workspace level.
 */
pub fn workspace_dialogs() -> Component {
  let prefs_dialog_open = use_workspace_store(select_workspace_prefs_dialog_open);
  let { dialog_actions } = use_workspace_actions();

  html! {
    <>
      {if prefs_dialog_open {
        <AppSettingsDialog
          id="app-settings-dialog"
          open
          onClose={
            move || {
              dialog_actions.preferences.close();
            }
          }
        />
      }}
    </>
  }
}
```
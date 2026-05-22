// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use leptos::prelude::*;

use crate::components::app_bar::AppBar;
use crate::components::sidebar::Sidebar;
use crate::components::panel_layout::PanelLayout;
use crate::components::playback_controls::PlaybackControls;
use crate::components::data_source_dialog::DataSourceDialog;
use crate::state::app_state::use_app_state;

/// Main workspace component containing the entire application layout.
#[component]
pub fn Workspace() -> impl IntoView {
    let state = use_app_state();
    let left_sidebar_open = state.left_sidebar_open;
    let right_sidebar_open = state.right_sidebar_open;

    // File drop handler
    let on_drop = move |ev: leptos::ev::DragEvent| {
        ev.prevent_default();
        // Handle dropped files (MCAP, bag, etc.)
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(data_transfer) = ev.data_transfer() {
                if let Some(files) = data_transfer.files() {
                    if files.length() > 0 {
                        log::info!("Files dropped: {}", files.length());
                        // TODO: Process dropped files through data source factory
                    }
                }
            }
        }
    };

    let on_dragover = move |ev: leptos::ev::DragEvent| {
        ev.prevent_default();
    };

    view! {
        <div
            class="workspace"
            on:drop=on_drop
            on:dragover=on_dragover
        >
            <AppBar />
            <div class="workspace-content">
                <Sidebar
                    side="left"
                    open=left_sidebar_open
                />
                <div class="workspace-main">
                    <PanelLayout />
                    <PlaybackControls />
                </div>
                <Sidebar
                    side="right"
                    open=right_sidebar_open
                />
            </div>
            <DataSourceDialog />
        </div>
    }
}

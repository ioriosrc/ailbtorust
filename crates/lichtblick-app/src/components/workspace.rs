// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::components::app_bar::AppBar;
use crate::components::sidebar::Sidebar;
use crate::components::panel_layout::PanelLayout;
use crate::components::playback_controls::PlaybackControls;
use crate::components::data_source_dialog::{DataSourceDialog, create_player_from_summary};
use crate::mcap_reader;
use crate::state::app_state::use_app_state;

/// Main workspace component containing the entire application layout.
#[component]
pub fn Workspace() -> impl IntoView {
    let state = use_app_state();
    let left_sidebar_open = state.left_sidebar_open;
    let right_sidebar_open = state.right_sidebar_open;

    // Drag-and-drop visual state
    let drag_over = RwSignal::new(false);
    let drop_loading = RwSignal::new(None::<String>);

    // File drop handler - loads MCAP files directly
    let on_drop = move |ev: leptos::ev::DragEvent| {
        ev.prevent_default();
        drag_over.set(false);

        if let Some(data_transfer) = ev.data_transfer() {
            if let Some(files) = data_transfer.files() {
                if files.length() > 0 {
                    let file = files.get(0).unwrap();
                    let file_name = file.name();

                    // Only accept .mcap and .bag files
                    if !file_name.ends_with(".mcap") && !file_name.ends_with(".bag") {
                        log::warn!("Unsupported file type: {}", file_name);
                        return;
                    }

                    let file_size = file.size();
                    log::info!("Dropped file: {} ({:.1} MB)", file_name, file_size / 1_048_576.0);

                    drop_loading.set(Some(format!("Loading {}...", file_name)));

                    // Same flow as DataSourceDialog: read footer → summary → create player
                    load_mcap_file(file, state, drop_loading);
                }
            }
        }
    };

    let on_dragover = move |ev: leptos::ev::DragEvent| {
        ev.prevent_default();
        // Check if files are being dragged
        if let Some(dt) = ev.data_transfer() {
            if let Some(items) = dt.items().length().checked_sub(0) {
                if items > 0 {
                    drag_over.set(true);
                }
            }
        }
    };

    let on_dragenter = move |ev: leptos::ev::DragEvent| {
        ev.prevent_default();
        drag_over.set(true);
    };

    let on_dragleave = move |ev: leptos::ev::DragEvent| {
        ev.prevent_default();
        // Only hide overlay if leaving the workspace element itself
        if let Some(target) = ev.target() {
            if let Some(related) = ev.related_target() {
                let target_el: web_sys::Element = target.dyn_into().unwrap_or_else(|_| {
                    web_sys::window().unwrap().document().unwrap().document_element().unwrap()
                });
                let related_el: web_sys::Element = related.dyn_into().unwrap_or_else(|_| {
                    web_sys::window().unwrap().document().unwrap().document_element().unwrap()
                });
                // If the related target is inside the workspace, don't dismiss
                if target_el.contains(Some(&related_el)) {
                    return;
                }
            }
        }
        drag_over.set(false);
    };

    view! {
        <div
            class="workspace"
            on:drop=on_drop
            on:dragover=on_dragover
            on:dragenter=on_dragenter
            on:dragleave=on_dragleave
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

            // Drag overlay
            <div class="drag-overlay" class:visible=move || drag_over.get()>
                <div class="drag-overlay-content">
                    <div class="drag-overlay-icon">{"📂"}</div>
                    <div class="drag-overlay-text">{"Drop MCAP file to open"}</div>
                </div>
            </div>

            // Loading overlay for drag-drop
            {move || drop_loading.get().map(|msg| view! {
                <div class="drop-loading-overlay">
                    <div class="loading-spinner"></div>
                    <p class="loading-text">{msg}</p>
                </div>
            })}
        </div>
    }
}

/// Load an MCAP file (same logic as DataSourceDialog but callable from drag-drop).
fn load_mcap_file(
    file: web_sys::File,
    state: crate::state::app_state::AppState,
    loading_status: RwSignal<Option<String>>,
) {
    let file_size = file.size();

    // Step 1: Read last 64 bytes to get the footer
    let footer_start = (file_size - 64.0).max(0.0);
    let footer_blob = file.slice_with_f64_and_f64(footer_start, file_size).unwrap();

    let reader = web_sys::FileReader::new().unwrap();
    let reader_clone = reader.clone();
    let file_clone = file.clone();

    let onload = Closure::once(move |_: web_sys::Event| {
        let array_buffer = reader_clone.result().unwrap();
        let uint8_array = js_sys::Uint8Array::new(&array_buffer);
        let footer_data = uint8_array.to_vec();

        match mcap_reader::get_summary_start_from_footer(&footer_data) {
            Ok(summary_start) => {
                loading_status.set(Some("Reading file index...".to_string()));

                // Step 2: Read from summary_start to EOF
                let summary_blob = file_clone
                    .slice_with_f64_and_f64(summary_start as f64, file_size)
                    .unwrap();

                let reader2 = web_sys::FileReader::new().unwrap();
                let reader2_clone = reader2.clone();
                let file_clone2 = file_clone.clone();

                let onload2 = Closure::once(move |_: web_sys::Event| {
                    let ab = reader2_clone.result().unwrap();
                    let arr = js_sys::Uint8Array::new(&ab);
                    let summary_data = arr.to_vec();

                    match mcap_reader::parse_summary_section(&summary_data) {
                        Ok(summary) => {
                            create_player_from_summary(
                                file_clone2, summary, state, loading_status,
                            );
                        }
                        Err(e) => {
                            log::error!("Failed to parse MCAP summary: {}", e);
                            loading_status.set(None);
                        }
                    }
                });

                reader2.set_onload(Some(onload2.as_ref().unchecked_ref()));
                onload2.forget();
                reader2.read_as_array_buffer(&summary_blob).unwrap();
            }
            Err(e) => {
                log::error!("Failed to read MCAP footer: {}", e);
                loading_status.set(None);
            }
        }
    });

    reader.set_onload(Some(onload.as_ref().unchecked_ref()));
    onload.forget();
    reader.read_as_array_buffer(&footer_blob).unwrap();
}

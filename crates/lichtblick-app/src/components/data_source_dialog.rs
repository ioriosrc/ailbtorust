// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::mcap_reader;
use crate::player::McapPlayer;
use crate::state::app_state::{use_app_state, set_player, get_player};

/// Data source selection dialog.
#[component]
pub fn DataSourceDialog() -> impl IntoView {
    let state = use_app_state();
    let is_open = state.data_source_dialog_open;
    let loading_status = RwSignal::new(None::<String>);

    let close = move |_| {
        state.data_source_dialog_open.set(false);
    };

    let open_file = move |_| {
        let document = web_sys::window().unwrap().document().unwrap();
        let input = document.create_element("input").unwrap();
        let input: web_sys::HtmlInputElement = input.dyn_into().unwrap();
        input.set_type("file");
        input.set_accept(".mcap,.bag");

        let state_clone = state;
        let input_clone = input.clone();

        let closure = Closure::once(move |_: web_sys::Event| {
            if let Some(files) = input_clone.files() {
                if files.length() > 0 {
                    let file = files.get(0).unwrap();
                    let file_name = file.name();
                    let file_size = file.size();
                    log::info!("Opening file: {} ({:.1} MB)", file_name, file_size / 1_048_576.0);

                    loading_status.set(Some("Reading file index...".to_string()));

                    // Step 1: Read just the last 64 bytes to get the footer
                    // (contains summary_start absolute offset)
                    let footer_start = (file_size - 64.0).max(0.0);
                    let footer_blob = file.slice_with_f64_and_f64(footer_start, file_size).unwrap();

                    let reader = web_sys::FileReader::new().unwrap();
                    let reader_clone = reader.clone();
                    let file_clone = file.clone();

                    let onload = Closure::once(move |_: web_sys::Event| {
                        let array_buffer = reader_clone.result().unwrap();
                        let uint8_array = js_sys::Uint8Array::new(&array_buffer);
                        let footer_data = uint8_array.to_vec();

                        // Extract summary_start from footer
                        match mcap_reader::get_summary_start_from_footer(&footer_data) {
                            Ok(summary_start) => {
                                log::info!(
                                    "Footer parsed: summary_start={} ({:.1} MB from start)",
                                    summary_start, summary_start as f64 / 1_048_576.0
                                );

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

                                    log::info!(
                                        "Summary section read: {:.1} KB",
                                        summary_data.len() as f64 / 1024.0
                                    );

                                    match mcap_reader::parse_summary_section(&summary_data) {
                                        Ok(summary) => {
                                            create_player_from_summary(
                                                file_clone2, summary, state_clone, loading_status
                                            );
                                        }
                                        Err(e) => {
                                            log::error!("Failed to parse summary section: {}", e);
                                            loading_status.set(Some(format!("Error: {}", e)));
                                        }
                                    }
                                });

                                reader2.set_onload(Some(onload2.as_ref().unchecked_ref()));
                                onload2.forget();
                                reader2.read_as_array_buffer(&summary_blob).unwrap();
                            }
                            Err(e) => {
                                log::error!("Failed to read MCAP footer: {}", e);
                                loading_status.set(Some(format!("Error: {}", e)));
                            }
                        }
                    });

                    reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                    onload.forget();
                    reader.read_as_array_buffer(&footer_blob).unwrap();
                }
            }
        });

        let body = document.body().unwrap();
        body.append_child(&input).unwrap();
        input.set_onchange(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
        input.click();

        // Cleanup input element after delay
        let input_clone = input.clone();
        let cleanup = Closure::once(move || { input_clone.remove(); });
        web_sys::window().unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                cleanup.as_ref().unchecked_ref(), 60000
            ).unwrap();
        cleanup.forget();
    };

    view! {
        <div class="dialog-overlay" class:hidden=move || !is_open.get()>
            <div class="dialog data-source-dialog">
                <div class="dialog-header">
                    <h2>{"Open data source"}</h2>
                    <button class="dialog-close" on:click=close>{"✕"}</button>
                </div>
                <div class="dialog-content">
                    {move || {
                        if let Some(status) = loading_status.get() {
                            view! {
                                <div class="loading-indicator">
                                    <div class="loading-spinner"></div>
                                    <p class="loading-text">{status}</p>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="data-source-options">
                                    <DataSourceOption
                                        title="Open local file"
                                        description="MCAP, ROS bag"
                                        icon="📁"
                                        on_click=open_file
                                    />
                                    <DataSourceOption
                                        title="Foxglove WebSocket"
                                        description="Connect to a live system via foxglove_bridge"
                                        icon="🔌"
                                        on_click=move |_| {
                                            log::info!("WebSocket connection not yet implemented");
                                        }
                                    />
                                </div>
                            }.into_any()
                        }
                    }}
                </div>
            </div>
        </div>
    }
}

/// Create player from parsed summary and set it as active.
pub fn create_player_from_summary(
    file: web_sys::File,
    summary: mcap_reader::McapSummary,
    state: crate::state::app_state::AppState,
    loading_status: RwSignal<Option<String>>,
) {
    let start_ns = summary.statistics.message_start_time;
    let end_ns = summary.statistics.message_end_time;
    let duration_secs = (end_ns - start_ns) as f64 / 1_000_000_000.0;
    let topic_count = summary.channels.len();
    let chunk_count = summary.chunk_indices.len();

    // Update UI state
    let mins = (duration_secs / 60.0).floor() as u32;
    let secs = duration_secs % 60.0;
    state.duration_display.set(format!("{}:{:05.3}", mins, secs));
    state.topic_count.set(topic_count);
    state.message_count.set(summary.statistics.message_count as usize);

    // Reset playback to the beginning
    state.is_playing.set(false);
    state.playback_progress.set(0.0);
    state.current_time_display.set("0:00.000".to_string());

    // Create lazy player - INSTANT, no message data loaded yet
    let player = McapPlayer::new_lazy(file, summary, state);
    set_player(player);

    // If URL has a time= parameter, seek to that position
    if let Some(time_ns) = crate::player::get_url_time_ns() {
        if let Some(p) = get_player() {
            p.seek_to_ns(time_ns);
        }
    }

    log::info!(
        "File opened: {} topics, {} chunks, {:.2}s duration",
        topic_count, chunk_count, duration_secs
    );

    loading_status.set(None);
    state.data_source_dialog_open.set(false);
}

/// A single data source option in the dialog.
#[component]
fn DataSourceOption(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    #[prop(into)] icon: String,
    on_click: impl Fn(leptos::ev::MouseEvent) + 'static,
) -> impl IntoView {
    view! {
        <button class="data-source-option" on:click=on_click>
            <span class="data-source-icon">{icon}</span>
            <div class="data-source-info">
                <span class="data-source-title">{title}</span>
                <span class="data-source-description">{description}</span>
            </div>
        </button>
    }
}

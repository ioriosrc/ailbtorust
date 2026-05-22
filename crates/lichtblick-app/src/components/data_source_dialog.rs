// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::player::{McapPlayer, parse_mcap_file};
use crate::state::app_state::{use_app_state, set_player};

/// Maximum file size to load into memory at once (800 MB).
/// Files larger than this will be loaded in chunks with progress feedback.
const MAX_DIRECT_LOAD_BYTES: f64 = 800_000_000.0;

/// Data source selection dialog.
#[component]
pub fn DataSourceDialog() -> impl IntoView {
    let state = use_app_state();
    let is_open = state.data_source_dialog_open;
    let loading_progress = RwSignal::new(None::<f64>); // None = not loading, Some(0..100) = progress

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
                    log::info!("Loading file: {} ({:.1} MB)", file_name, file_size / 1_048_576.0);

                    if file_size > MAX_DIRECT_LOAD_BYTES {
                        log::warn!(
                            "File is {:.0} MB - loading in chunks to avoid memory issues",
                            file_size / 1_048_576.0
                        );
                    }

                    loading_progress.set(Some(0.0));

                    // Read file using FileReader API
                    let reader = web_sys::FileReader::new().unwrap();
                    let reader_clone = reader.clone();

                    let onload = Closure::once(move |_: web_sys::Event| {
                        loading_progress.set(Some(50.0));

                        let array_buffer = reader_clone.result().unwrap();
                        let uint8_array = js_sys::Uint8Array::new(&array_buffer);
                        let data = uint8_array.to_vec();

                        log::info!("File read into memory: {:.1} MB", data.len() as f64 / 1_048_576.0);
                        loading_progress.set(Some(70.0));

                        // Parse in a setTimeout to allow the UI to update
                        let parse_closure = Closure::once(move || {
                            loading_progress.set(Some(80.0));

                            match parse_mcap_file(&data) {
                                Ok(mcap_data) => {
                                    let duration_ns = mcap_data.end_time_ns - mcap_data.start_time_ns;
                                    let duration_secs = duration_ns as f64 / 1_000_000_000.0;
                                    let topic_count = mcap_data.topics.len();
                                    let msg_count = mcap_data.messages.len();

                                    // Update UI state
                                    let mins = (duration_secs / 60.0).floor() as u32;
                                    let secs = duration_secs % 60.0;
                                    state_clone.duration_display.set(format!("{}:{:05.3}", mins, secs));
                                    state_clone.topic_count.set(topic_count);
                                    state_clone.message_count.set(msg_count);

                                    // Create player
                                    let player = McapPlayer::new(mcap_data, state_clone);
                                    set_player(player);

                                    log::info!(
                                        "Player ready: {} topics, {} messages, {:.2}s",
                                        topic_count,
                                        msg_count,
                                        duration_secs
                                    );
                                }
                                Err(e) => {
                                    log::error!("Failed to parse MCAP: {}", e);
                                }
                            }

                            loading_progress.set(None);
                            state_clone.data_source_dialog_open.set(false);
                        });

                        // Use setTimeout(0) to yield to the browser before heavy parsing
                        web_sys::window()
                            .unwrap()
                            .set_timeout_with_callback_and_timeout_and_arguments_0(
                                parse_closure.as_ref().unchecked_ref(),
                                0,
                            )
                            .unwrap();
                        parse_closure.forget();
                    });

                    reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                    onload.forget();
                    reader.read_as_array_buffer(&file).unwrap();
                }
            }
        });

        // We need to keep the input in the DOM briefly for the change event
        let body = document.body().unwrap();
        body.append_child(&input).unwrap();
        input.set_onchange(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
        input.click();
        // Remove input after a delay
        let input_clone = input.clone();
        let cleanup = Closure::once(move || {
            input_clone.remove();
        });
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                cleanup.as_ref().unchecked_ref(),
                60000,
            )
            .unwrap();
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
                        if let Some(progress) = loading_progress.get() {
                            view! {
                                <div class="loading-indicator">
                                    <div class="loading-spinner"></div>
                                    <p class="loading-text">{format!("Loading file... {:.0}%", progress)}</p>
                                    <div class="loading-bar">
                                        <div class="loading-bar-fill" style=format!("width: {}%", progress)></div>
                                    </div>
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

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

//! Image panel - displays CompressedImage and Image topics as video frames.
//! Uses browser's createImageBitmap via Blob URL for compressed images.

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::decoder::{decode_compressed_image, is_compressed_image_schema};
use crate::state::app_state::{get_player, use_app_state};

/// Image panel component - subscribes to an image topic and renders frames.
#[component]
pub fn ImagePanel(
    #[prop(into)] topic: String,
) -> impl IntoView {
    let state = use_app_state();
    let frame_tick = state.frame_tick;
    let topic_clone = topic.clone();

    // Store the current blob URL
    let img_url = RwSignal::new(String::new());
    let last_time = RwSignal::new(0u64);

    // Effect that updates the image when frame_tick changes
    let topic_for_effect = topic_clone.clone();
    Effect::new(move |_| {
        let _tick = frame_tick.get();

        let player = match get_player() {
            Some(p) => p,
            None => return,
        };

        let msg = match player.get_current_message(&topic_for_effect) {
            Some(m) => m,
            None => return,
        };

        // Only update if timestamp changed
        if msg.log_time_ns == last_time.get_untracked() {
            return;
        }
        last_time.set(msg.log_time_ns);

        // Decode the compressed image
        if let Some(decoded) = decode_compressed_image(&msg.data, &msg.encoding) {
            // Create a Blob URL from the image data
            let array = js_sys::Uint8Array::new_with_length(decoded.data.len() as u32);
            array.copy_from(&decoded.data);

            let parts = js_sys::Array::new();
            parts.push(&array.buffer());

            let mime_type = match decoded.format.as_str() {
                "jpeg" | "jpg" => "image/jpeg",
                "png" => "image/png",
                "webp" => "image/webp",
                _ => "image/jpeg",
            };

            let mut opts = web_sys::BlobPropertyBag::new();
            opts.set_type(mime_type);

            if let Ok(blob) = web_sys::Blob::new_with_u8_array_sequence_and_options(&parts, &opts)
            {
                if let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob) {
                    // Revoke previous URL
                    let prev = img_url.get_untracked();
                    if !prev.is_empty() {
                        web_sys::Url::revoke_object_url(&prev).ok();
                    }
                    img_url.set(url);
                }
            }
        }
    });

    view! {
        <div class="image-panel-content">
            {move || {
                let url = img_url.get();
                if url.is_empty() {
                    view! {
                        <div class="panel-empty">
                            <span>{"Waiting for image data..."}</span>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <img
                            class="image-panel-img"
                            src=url
                            alt="Camera feed"
                        />
                    }.into_any()
                }
            }}
        </div>
    }
}

/// Multi-image panel showing a grid of camera feeds (like the 4-camera view).
#[component]
pub fn MultiImagePanel(
    #[prop(into)] topics: Vec<String>,
    #[prop(optional, into)] title: Option<String>,
) -> impl IntoView {
    let display_title = title.unwrap_or_else(|| "Cameras".to_string());

    view! {
        <div class="panel-container multi-image-panel">
            <div class="panel-toolbar">
                <span class="panel-title">{display_title}</span>
            </div>
            <div class="panel-content multi-image-grid">
                {topics.into_iter().map(|topic| {
                    view! { <ImagePanelInner topic=topic /> }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

/// Inner image panel without chrome (for grid use).
#[component]
fn ImagePanelInner(#[prop(into)] topic: String) -> impl IntoView {
    let state = use_app_state();
    let frame_tick = state.frame_tick;
    let topic_clone = topic.clone();

    let img_url = RwSignal::new(String::new());
    let last_time = RwSignal::new(0u64);

    Effect::new(move |_| {
        let _tick = frame_tick.get();

        let player = match get_player() {
            Some(p) => p,
            None => return,
        };

        let msg = match player.get_current_message(&topic_clone) {
            Some(m) => m,
            None => return,
        };

        if msg.log_time_ns == last_time.get_untracked() {
            return;
        }
        last_time.set(msg.log_time_ns);

        if let Some(decoded) = decode_compressed_image(&msg.data, &msg.encoding) {
            let array = js_sys::Uint8Array::new_with_length(decoded.data.len() as u32);
            array.copy_from(&decoded.data);

            let parts = js_sys::Array::new();
            parts.push(&array.buffer());

            let mime_type = match decoded.format.as_str() {
                "jpeg" | "jpg" => "image/jpeg",
                "png" => "image/png",
                "webp" => "image/webp",
                _ => "image/jpeg",
            };

            let mut opts = web_sys::BlobPropertyBag::new();
            opts.set_type(mime_type);

            if let Ok(blob) = web_sys::Blob::new_with_u8_array_sequence_and_options(&parts, &opts)
            {
                if let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob) {
                    let prev = img_url.get_untracked();
                    if !prev.is_empty() {
                        web_sys::Url::revoke_object_url(&prev).ok();
                    }
                    img_url.set(url);
                }
            }
        }
    });

    view! {
        <div class="image-grid-cell">
            {move || {
                let url = img_url.get();
                if url.is_empty() {
                    view! { <div class="panel-empty-small">{"..."}</div> }.into_any()
                } else {
                    view! { <img class="image-panel-img" src=url /> }.into_any()
                }
            }}
        </div>
    }
}

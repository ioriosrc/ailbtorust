// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

//! Image panel - displays CompressedImage and Image topics as video frames.
//! Uses browser's createImageBitmap via Blob URL for compressed images.
//! Employs double-buffering: a hidden img loads the next frame, and only swaps
//! to visible once onload fires, preventing any visual flicker.

use std::cell::Cell;
use std::rc::Rc;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::decoder::decode_compressed_image;
use crate::state::app_state::{get_player, use_app_state, use_layout_state, NodeId};

/// Image panel component - subscribes to an image topic and renders frames.
/// Applies brightness, contrast, flip, and rotation from per-panel config.
///
/// Uses a stable <img> element in the DOM (never recreated) and an onload gate
/// to prevent setting a new src while the browser is still decoding the previous frame.
#[component]
pub fn ImagePanel(
    #[prop(into)] topic: String,
    node_id: NodeId,
) -> impl IntoView {
    let state = use_app_state();
    let layout = use_layout_state();
    let frame_tick = state.frame_tick;
    let topic_clone = topic.clone();

    let last_time = RwSignal::new(0u64);
    let has_image = RwSignal::new(false);

    // NodeRef to the actual <img> DOM element - we'll update src directly
    let img_ref = NodeRef::<leptos::html::Img>::new();

    // Gate: tracks whether the img element is ready for a new frame.
    // Only set a new src after the previous onload fired.
    let img_ready = Rc::new(Cell::new(true));

    // Store the previous blob URL for cleanup
    let prev_url: Rc<Cell<Option<String>>> = Rc::new(Cell::new(None));

    // Pending URL: if we decoded a frame while img was still loading, store it here
    let pending_url: Rc<Cell<Option<String>>> = Rc::new(Cell::new(None));

    // Set up the onload handler once the img element is mounted
    let img_ready_for_onload = Rc::clone(&img_ready);
    let prev_url_for_onload = Rc::clone(&prev_url);
    let pending_url_for_onload = Rc::clone(&pending_url);
    let img_ref_for_onload = img_ref;
    Effect::new(move |_| {
        let Some(el) = img_ref_for_onload.get() else { return; };
        let el: &web_sys::HtmlImageElement = &el;

        let ready = Rc::clone(&img_ready_for_onload);
        let prev = Rc::clone(&prev_url_for_onload);
        let pending = Rc::clone(&pending_url_for_onload);
        let el_clone = el.clone();

        let onload = Closure::<dyn FnMut()>::new(move || {
            // Revoke the URL that was being displayed before this one
            if let Some(old) = prev.take() {
                web_sys::Url::revoke_object_url(&old).ok();
            }

            // If a newer frame arrived while we were loading, apply it immediately
            if let Some(next_url) = pending.take() {
                // Store current src as the one to revoke on next load
                let current_src = el_clone.src();
                if !current_src.is_empty() && current_src.starts_with("blob:") {
                    prev.set(Some(current_src));
                }
                el_clone.set_src(&next_url);
                // Stay in "not ready" state - wait for this new load
            } else {
                ready.set(true);
            }
        });

        el.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget(); // Leak intentionally - lives as long as the element
    });

    // Effect that updates the image when frame_tick changes
    let topic_for_effect = topic_clone.clone();
    let img_ready_for_effect = Rc::clone(&img_ready);
    let prev_url_for_effect = Rc::clone(&prev_url);
    let pending_url_for_effect = Rc::clone(&pending_url);

    Effect::new(move |_| {
        let _tick = frame_tick.get();

        let player = match get_player() {
            Some(p) => p,
            None => return,
        };

        let msg = match player.get_current_message(&topic_for_effect) {
            Some(m) => m,
            None => {
                // No message available (chunks may not be loaded yet).
                // Reset last_time so when a message arrives, it won't be rejected.
                last_time.set(0);
                return;
            },
        };

        // Only update if timestamp actually changed (avoid redundant decodes).
        let prev_time = last_time.get_untracked();
        if msg.log_time_ns == prev_time {
            return;
        }

        last_time.set(msg.log_time_ns);

        // Decode the compressed image
        let decoded = match decode_compressed_image(&msg.data, &msg.encoding) {
            Some(d) => d,
            None => return,
        };

        // Create a Blob URL from the image data (zero-copy view into wasm memory)
        let array = unsafe { js_sys::Uint8Array::view(&decoded.data) };

        let parts = js_sys::Array::new();
        parts.push(&array);

        let mime_type = match decoded.format.as_str() {
            "jpeg" | "jpg" => "image/jpeg",
            "png" => "image/png",
            "webp" => "image/webp",
            _ => "image/jpeg",
        };

        let opts = web_sys::BlobPropertyBag::new();
        opts.set_type(mime_type);

        let blob = match web_sys::Blob::new_with_u8_array_sequence_and_options(&parts, &opts) {
            Ok(b) => b,
            Err(_) => return,
        };
        let url = match web_sys::Url::create_object_url_with_blob(&blob) {
            Ok(u) => u,
            Err(_) => return,
        };

        // If the img element isn't ready (still loading previous frame), queue this URL
        if !img_ready_for_effect.get() {
            // Revoke whatever was pending before (we're skipping that frame)
            if let Some(old_pending) = pending_url_for_effect.take() {
                web_sys::Url::revoke_object_url(&old_pending).ok();
            }
            pending_url_for_effect.set(Some(url));
            return;
        }

        // Apply the new frame directly
        if let Some(el) = img_ref.get() {
            let el: &web_sys::HtmlImageElement = &el;
            // Store current src for revocation after new one loads
            let current_src = el.src();
            if !current_src.is_empty() && current_src.starts_with("blob:") {
                if let Some(old) = prev_url_for_effect.take() {
                    web_sys::Url::revoke_object_url(&old).ok();
                }
                prev_url_for_effect.set(Some(current_src));
            }
            img_ready_for_effect.set(false);
            el.set_src(&url);
            if !has_image.get_untracked() {
                has_image.set(true);
            }
        }
    });

    // Compute CSS style from image config (brightness, contrast, flip, rotation)
    let img_style = move || {
        let cfg = layout.image_configs.with(|configs| {
            configs.get(&node_id).cloned().unwrap_or_default()
        });

        let brightness_css = 0.4 + (cfg.brightness / 100.0) * 1.2;
        let contrast_css = 0.1 + (cfg.contrast / 100.0) * 1.8;

        let filter = format!("brightness({:.2}) contrast({:.2})", brightness_css, contrast_css);

        let mut transforms = Vec::new();
        if cfg.rotation != 0 {
            transforms.push(format!("rotate({}deg)", cfg.rotation));
        }
        if cfg.flip_horizontal {
            transforms.push("scaleX(-1)".to_string());
        }
        if cfg.flip_vertical {
            transforms.push("scaleY(-1)".to_string());
        }

        let transform = if transforms.is_empty() {
            String::new()
        } else {
            format!("transform: {};", transforms.join(" "))
        };

        format!("filter: {}; {}", filter, transform)
    };

    // Stable DOM: the <img> is always present (hidden until first frame loads).
    // We never recreate it - only update src via the Effect above.
    view! {
        <div class="image-panel-content">
            <div class="panel-empty" style=move || {
                if has_image.get() { "display:none" } else { "" }
            }>
                <span>{"Waiting for image data..."}</span>
            </div>
            <img
                node_ref=img_ref
                class="image-panel-img"
                alt="Camera feed"
                style=img_style
                prop:hidden=move || !has_image.get()
            />
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
        <div class="multi-image-grid">
            {topics.into_iter().map(|topic| {
                view! { <ImagePanelInner topic=topic /> }
            }).collect::<Vec<_>>()}
        </div>
    }
}

/// Inner image panel without chrome (for grid use).
#[component]
fn ImagePanelInner(#[prop(into)] topic: String) -> impl IntoView {
    let state = use_app_state();
    let frame_tick = state.frame_tick;
    let topic_clone = topic.clone();

    let last_time = RwSignal::new(0u64);
    let has_image = RwSignal::new(false);
    let img_ref = NodeRef::<leptos::html::Img>::new();
    let img_ready = Rc::new(Cell::new(true));
    let prev_url: Rc<Cell<Option<String>>> = Rc::new(Cell::new(None));
    let pending_url: Rc<Cell<Option<String>>> = Rc::new(Cell::new(None));

    // Set up onload handler
    let img_ready_onload = Rc::clone(&img_ready);
    let prev_url_onload = Rc::clone(&prev_url);
    let pending_url_onload = Rc::clone(&pending_url);
    let img_ref_onload = img_ref;
    Effect::new(move |_| {
        let Some(el) = img_ref_onload.get() else { return; };
        let el: &web_sys::HtmlImageElement = &el;

        let ready = Rc::clone(&img_ready_onload);
        let prev = Rc::clone(&prev_url_onload);
        let pending = Rc::clone(&pending_url_onload);
        let el_clone = el.clone();

        let onload = Closure::<dyn FnMut()>::new(move || {
            if let Some(old) = prev.take() {
                web_sys::Url::revoke_object_url(&old).ok();
            }
            if let Some(next_url) = pending.take() {
                let current_src = el_clone.src();
                if !current_src.is_empty() && current_src.starts_with("blob:") {
                    prev.set(Some(current_src));
                }
                el_clone.set_src(&next_url);
            } else {
                ready.set(true);
            }
        });
        el.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget();
    });

    let img_ready_eff = Rc::clone(&img_ready);
    let prev_url_eff = Rc::clone(&prev_url);
    let pending_url_eff = Rc::clone(&pending_url);
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
        let prev_t = last_time.get_untracked();
        if msg.log_time_ns < prev_t {
            if prev_t - msg.log_time_ns < 100_000_000 {
                return;
            }
        }
        last_time.set(msg.log_time_ns);

        let decoded = match decode_compressed_image(&msg.data, &msg.encoding) {
            Some(d) => d,
            None => return,
        };

        let array = unsafe { js_sys::Uint8Array::view(&decoded.data) };

        let parts = js_sys::Array::new();
        parts.push(&array);

        let mime_type = match decoded.format.as_str() {
            "jpeg" | "jpg" => "image/jpeg",
            "png" => "image/png",
            "webp" => "image/webp",
            _ => "image/jpeg",
        };

        let opts = web_sys::BlobPropertyBag::new();
        opts.set_type(mime_type);

        let blob = match web_sys::Blob::new_with_u8_array_sequence_and_options(&parts, &opts) {
            Ok(b) => b,
            Err(_) => return,
        };
        let url = match web_sys::Url::create_object_url_with_blob(&blob) {
            Ok(u) => u,
            Err(_) => return,
        };

        if !img_ready_eff.get() {
            if let Some(old_pending) = pending_url_eff.take() {
                web_sys::Url::revoke_object_url(&old_pending).ok();
            }
            pending_url_eff.set(Some(url));
            return;
        }

        if let Some(el) = img_ref.get() {
            let el: &web_sys::HtmlImageElement = &el;
            let current_src = el.src();
            if !current_src.is_empty() && current_src.starts_with("blob:") {
                if let Some(old) = prev_url_eff.take() {
                    web_sys::Url::revoke_object_url(&old).ok();
                }
                prev_url_eff.set(Some(current_src));
            }
            img_ready_eff.set(false);
            el.set_src(&url);
            if !has_image.get_untracked() {
                has_image.set(true);
            }
        }
    });

    view! {
        <div class="image-grid-cell">
            <div class="panel-empty-small" style=move || {
                if has_image.get() { "display:none" } else { "" }
            }>{"..."}</div>
            <img
                node_ref=img_ref
                class="image-panel-img"
                prop:hidden=move || !has_image.get()
            />
        </div>
    }
}

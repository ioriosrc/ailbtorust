// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::state::app_state::{use_app_state, get_player};

/// Playback controls bar at the bottom of the workspace.
#[component]
pub fn PlaybackControls() -> impl IntoView {
    let state = use_app_state();
    let is_playing = state.is_playing;
    let current_time = state.current_time_display;
    let duration = state.duration_display;
    let progress = state.playback_progress;
    let has_data = state.has_active_layout;

    // Track when user is interacting with the slider to prevent progress overwrite
    let is_seeking = RwSignal::new(false);
    // Local slider value - syncs from progress unless user is dragging
    let slider_value = RwSignal::new(0.0f64);

    // Sync slider from progress when not seeking
    Effect::new(move |_| {
        let p = progress.get();
        if !is_seeking.get_untracked() {
            slider_value.set(p);
        }
    });

    let toggle_play = move |_| {
        if let Some(player) = get_player() {
            if is_playing.get_untracked() {
                player.pause();
            } else {
                player.play();
            }
        }
    };

    let on_seek = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(val) = input.value().parse::<f64>() {
                    slider_value.set(val);
                    if let Some(player) = get_player() {
                        player.seek(val / 100.0);
                    }
                }
            }
        }
    };

    let on_pointer_down = move |_: leptos::ev::PointerEvent| {
        is_seeking.set(true);
    };

    let on_pointer_up = move |_: leptos::ev::PointerEvent| {
        is_seeking.set(false);
    };

    let on_speed_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(select) = target.dyn_into::<web_sys::HtmlSelectElement>() {
                if let Ok(speed) = select.value().parse::<f64>() {
                    if let Some(player) = get_player() {
                        player.set_speed(speed);
                    }
                }
            }
        }
    };

    view! {
        <div class="playback-controls" class:hidden=move || !has_data.get()>
            <div class="playback-bar">
                <div class="playback-time start-time">
                    {move || current_time.get()}
                </div>
                <div class="playback-buttons">
                    <button class="playback-btn play-btn"
                        on:click=toggle_play
                        title=move || if is_playing.get() { "Pause" } else { "Play" }
                    >
                        {move || if is_playing.get() { "⏸" } else { "▶" }}
                    </button>
                </div>
                <input
                    type="range"
                    class="playback-slider"
                    min="0"
                    max="100"
                    step="0.01"
                    prop:value=move || slider_value.get().to_string()
                    on:input=on_seek
                    on:pointerdown=on_pointer_down
                    on:pointerup=on_pointer_up
                />
                <div class="playback-time end-time">
                    {move || duration.get()}
                </div>
                <div class="playback-speed">
                    <select class="speed-select" on:change=on_speed_change>
                        <option value="0.1">{"0.1x"}</option>
                        <option value="0.25">{"0.25x"}</option>
                        <option value="0.5">{"0.5x"}</option>
                        <option value="1" selected=true>{"1x"}</option>
                        <option value="2">{"2x"}</option>
                        <option value="5">{"5x"}</option>
                        <option value="10">{"10x"}</option>
                    </select>
                </div>
            </div>
        </div>
    }
}

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::player::{format_timestamp_full, format_timestamp_secs, format_date, format_time_only, format_elapsed_secs, format_duration_ns};
use crate::state::app_state::{use_app_state, get_player, TimeFormat};

/// Playback controls bar at the bottom of the workspace.
#[component]
pub fn PlaybackControls() -> impl IntoView {
    let state = use_app_state();
    let is_playing = state.is_playing;
    let current_time = state.current_time_display;
    let progress = state.playback_progress;
    let has_data = state.has_active_layout;
    let loop_playback = state.loop_playback;
    let time_format = state.time_format;

    // Track when user is interacting with the slider to prevent progress overwrite
    let is_seeking = RwSignal::new(false);
    // Local slider value - syncs from progress unless user is dragging
    let slider_value = RwSignal::new(0.0f64);

    // Info tooltip hover state
    let info_hover = RwSignal::new(false);

    // Timeline hover state for scrubber tooltip
    let timeline_hover = RwSignal::new(false);
    let timeline_hover_x = RwSignal::new(0i32);
    let timeline_hover_y = RwSignal::new(0i32);
    let timeline_hover_fraction = RwSignal::new(0.0f64);

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

    let on_seek_backward = move |_: leptos::ev::MouseEvent| {
        if let Some(player) = get_player() {
            player.seek_by_ms(-100);
        }
    };

    let on_seek_forward = move |_: leptos::ev::MouseEvent| {
        if let Some(player) = get_player() {
            player.seek_by_ms(100);
        }
    };

    let toggle_loop = move |_: leptos::ev::MouseEvent| {
        loop_playback.set(!loop_playback.get_untracked());
    };

    let on_time_format_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(select) = target.dyn_into::<web_sys::HtmlSelectElement>() {
                let new_format = TimeFormat::from_str(&select.value());
                time_format.set(new_format);
                // Persist to localStorage
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                        storage.set_item("lichtblick:time_format", new_format.as_str()).ok();
                    }
                }
                // Re-format the current time display immediately
                if let Some(player) = get_player() {
                    let ns = player.current_time_ns();
                    let formatted = match new_format {
                        TimeFormat::TOD => format_timestamp_full(ns),
                        TimeFormat::SEC => format_timestamp_secs(ns),
                    };
                    state.current_time_display.set(formatted);
                }
            }
        }
    };

    let on_info_enter = move |_: leptos::ev::MouseEvent| {
        info_hover.set(true);
    };

    let on_info_leave = move |_: leptos::ev::MouseEvent| {
        info_hover.set(false);
    };

    // Timeline scrubber hover handlers
    let on_slider_mousemove = move |ev: leptos::ev::MouseEvent| {
        if let Some(target) = ev.current_target() {
            let el: web_sys::HtmlElement = target.unchecked_into();
            let rect = el.get_bounding_client_rect();
            let x = ev.client_x() as f64 - rect.left();
            let fraction = (x / rect.width()).clamp(0.0, 1.0);
            timeline_hover.set(true);
            timeline_hover_x.set(ev.client_x());
            timeline_hover_y.set(rect.top() as i32 - 8);
            timeline_hover_fraction.set(fraction);
        }
    };

    let on_slider_mouseleave = move |_: leptos::ev::MouseEvent| {
        timeline_hover.set(false);
    };

    view! {
        <div class="playback-controls" class:hidden=move || !has_data.get()>
            // Top row: full-width scrubber/timeline
            <div class="playback-scrubber-row"
                on:mousemove=on_slider_mousemove
                on:mouseleave=on_slider_mouseleave
            >
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
            </div>
            // Timeline hover tooltip
            {move || {
                if timeline_hover.get() {
                    if let Some(player) = get_player() {
                        let fraction = timeline_hover_fraction.get();
                        let start = player.start_time_ns();
                        let end = player.end_time_ns();
                        let duration_ns = end - start;
                        let hover_ns = start + (fraction * duration_ns as f64) as u64;
                        let elapsed_ns = hover_ns - start;

                        let elapsed = format_elapsed_secs(elapsed_ns);

                        let x = timeline_hover_x.get();
                        let y = timeline_hover_y.get();
                        let style = format!("position:fixed;left:{}px;top:{}px;transform:translate(-50%, -100%);z-index:10002;", x, y);

                        let fmt = time_format.get();
                        let tooltip_view = match fmt {
                            TimeFormat::TOD => {
                                let date_str = format_date(hover_ns);
                                let time_str = format_time_only(hover_ns);
                                view! {
                                    <div class="scrubber-tooltip" style=style>
                                        <div class="scrubber-tooltip-arrow"></div>
                                        <div class="scrubber-tooltip-row">
                                            <span class="scrubber-tooltip-label">{"Date"}</span>
                                            <span class="scrubber-tooltip-value">{date_str}</span>
                                        </div>
                                        <div class="scrubber-tooltip-row">
                                            <span class="scrubber-tooltip-label">{"Time"}</span>
                                            <span class="scrubber-tooltip-value">{time_str}</span>
                                        </div>
                                        <div class="scrubber-tooltip-row">
                                            <span class="scrubber-tooltip-label">{"Elapsed"}</span>
                                            <span class="scrubber-tooltip-value">{elapsed}</span>
                                        </div>
                                    </div>
                                }.into_any()
                            }
                            TimeFormat::SEC => {
                                let sec_str = format_timestamp_secs(hover_ns);
                                view! {
                                    <div class="scrubber-tooltip" style=style>
                                        <div class="scrubber-tooltip-arrow"></div>
                                        <div class="scrubber-tooltip-row">
                                            <span class="scrubber-tooltip-label">{"SEC"}</span>
                                            <span class="scrubber-tooltip-value">{sec_str}</span>
                                        </div>
                                        <div class="scrubber-tooltip-row">
                                            <span class="scrubber-tooltip-label">{"Elapsed"}</span>
                                            <span class="scrubber-tooltip-value">{elapsed}</span>
                                        </div>
                                    </div>
                                }.into_any()
                            }
                        };
                        Some(tooltip_view)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }}
            // Bottom row: left | center | right
            <div class="playback-bar">
                // Left section: info + time
                <div class="playback-left">
                    <div class="playback-info-btn-wrapper"
                        on:mouseenter=on_info_enter
                        on:mouseleave=on_info_leave
                    >
                        <button class="playback-info-btn" title="Data source info">
                            <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
                                <circle cx="8" cy="8" r="7" fill="none" stroke="currentColor" stroke-width="1.5"/>
                                <text x="8" y="12" text-anchor="middle" font-size="10" font-weight="bold" fill="currentColor">{"i"}</text>
                            </svg>
                        </button>
                        {move || {
                            if info_hover.get() {
                                if let Some(player) = get_player() {
                                    let start = player.start_time_ns();
                                    let end = player.end_time_ns();
                                    let duration = end - start;
                                    Some(view! {
                                        <div class="playback-info-tooltip">
                                            <div class="info-tooltip-row">
                                                <span class="info-tooltip-label">{"Start time"}</span>
                                                <span class="info-tooltip-value">{format_timestamp_full(start)}</span>
                                            </div>
                                            <div class="info-tooltip-row">
                                                <span class="info-tooltip-label">{"End time"}</span>
                                                <span class="info-tooltip-value">{format_timestamp_full(end)}</span>
                                            </div>
                                            <div class="info-tooltip-row">
                                                <span class="info-tooltip-label">{"Duration"}</span>
                                                <span class="info-tooltip-value">{format_duration_ns(duration)}</span>
                                            </div>
                                        </div>
                                    })
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }}
                    </div>
                    <div class="playback-time-wrapper">
                        <div class="playback-time">
                            {move || current_time.get()}
                        </div>
                        <select class="time-format-select"
                            on:change=on_time_format_change
                            prop:value=move || time_format.get().as_str().to_string()
                        >
                            <option value="TOD" selected=move || time_format.get() == TimeFormat::TOD>
                                {"TOD"}
                            </option>
                            <option value="SEC" selected=move || time_format.get() == TimeFormat::SEC>
                                {"SEC"}
                            </option>
                        </select>
                    </div>
                </div>
                // Center section: transport controls
                <div class="playback-center">
                    <button class="playback-btn seek-btn"
                        on:click=on_seek_backward
                        title="Seek backward 100ms"
                    >
                        <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                            <path d="M11 18V6l-8.5 6 8.5 6zm.5-6l8.5 6V6l-8.5 6z"/>
                        </svg>
                    </button>
                    <button class="playback-btn play-btn"
                        on:click=toggle_play
                        title=move || if is_playing.get() { "Pause" } else { "Play" }
                    >
                        {move || if is_playing.get() { "⏸" } else { "▶" }}
                    </button>
                    <button class="playback-btn seek-btn"
                        on:click=on_seek_forward
                        title="Seek forward 100ms"
                    >
                        <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                            <path d="M4 18l8.5-6L4 6v12zm9-12v12l8.5-6L13 6z"/>
                        </svg>
                    </button>
                </div>
                // Right section: loop + speed
                <div class="playback-right">
                    <button
                        class="playback-btn loop-btn"
                        class:loop-active=move || loop_playback.get()
                        on:click=toggle_loop
                        title=move || if loop_playback.get() { "Loop: ON" } else { "Loop: OFF" }
                    >
                        {move || if loop_playback.get() {
                            view! {
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                                    <path d="M7 7h10v3l4-4-4-4v3H5v6h2V7zm10 10H7v-3l-4 4 4 4v-3h12v-6h-2v4z"/>
                                </svg>
                            }.into_any()
                        } else {
                            view! {
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                                    <path d="M7 7h10v3l4-4-4-4v3H5v6h2V7zm10 10H7v-3l-4 4 4 4v-3h12v-6h-2v4z" opacity="0.5"/>
                                    <line x1="4" y1="20" x2="20" y2="4" stroke="currentColor" stroke-width="2"/>
                                </svg>
                            }.into_any()
                        }}
                    </button>
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
        </div>
    }
}

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use std::cell::RefCell;
use std::rc::Rc;
use leptos::prelude::*;
use lichtblick_core::settings::ColorScheme;

use crate::player::McapPlayer;

thread_local! {
    static PLAYER: RefCell<Option<Rc<McapPlayer>>> = RefCell::new(None);
}

/// Set the global player instance.
pub fn set_player(player: McapPlayer) {
    PLAYER.with(|p| {
        *p.borrow_mut() = Some(Rc::new(player));
    });
}

/// Get a clone of the global player Rc (if any).
pub fn get_player() -> Option<Rc<McapPlayer>> {
    PLAYER.with(|p| p.borrow().clone())
}

/// Global application state, provided at the root level.
#[derive(Clone, Copy)]
pub struct AppState {
    // UI state
    pub left_sidebar_open: RwSignal<bool>,
    pub right_sidebar_open: RwSignal<bool>,
    pub data_source_dialog_open: RwSignal<bool>,

    // Playback state
    pub is_playing: RwSignal<bool>,
    pub playback_progress: RwSignal<f64>,
    pub playback_speed: RwSignal<f64>,
    pub current_time_display: RwSignal<String>,
    pub duration_display: RwSignal<String>,
    pub has_active_layout: RwSignal<bool>,
    pub topic_count: RwSignal<usize>,
    pub message_count: RwSignal<usize>,
    /// Frame tick counter - incremented every animation frame to trigger panel re-renders.
    pub frame_tick: RwSignal<u64>,

    // Settings
    pub color_scheme: RwSignal<ColorScheme>,
}

/// Provide the global app state to the component tree.
pub fn provide_app_state() {
    let state = AppState {
        left_sidebar_open: RwSignal::new(false),
        right_sidebar_open: RwSignal::new(false),
        data_source_dialog_open: RwSignal::new(false),
        is_playing: RwSignal::new(false),
        playback_progress: RwSignal::new(0.0),
        playback_speed: RwSignal::new(1.0),
        current_time_display: RwSignal::new("0:00.000".to_string()),
        duration_display: RwSignal::new("0:00.000".to_string()),
        has_active_layout: RwSignal::new(false),
        topic_count: RwSignal::new(0),
        message_count: RwSignal::new(0),
        frame_tick: RwSignal::new(0),
        color_scheme: RwSignal::new(ColorScheme::Dark),
    };

    provide_context(state);
}

/// Access the global app state from any component.
pub fn use_app_state() -> AppState {
    expect_context::<AppState>()
}

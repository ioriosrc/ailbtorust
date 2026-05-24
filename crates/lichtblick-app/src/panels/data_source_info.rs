// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

//! Data Source Info panel - shows info about the loaded MCAP.

use leptos::prelude::*;

use crate::state::app_state::{get_player, use_app_state};

/// Data source info panel.
#[component]
pub fn DataSourceInfoPanel() -> impl IntoView {
    let state = use_app_state();

    view! {
        <div class="datasource-info-content">
            <div class="info-grid">
                <div class="info-row">
                    <span class="info-label">{"Topics"}</span>
                    <span class="info-value">{move || state.topic_count.get().to_string()}</span>
                </div>
                <div class="info-row">
                    <span class="info-label">{"Messages"}</span>
                    <span class="info-value">{move || state.message_count.get().to_string()}</span>
                </div>
                <div class="info-row">
                    <span class="info-label">{"Duration"}</span>
                    <span class="info-value">{move || state.duration_display.get()}</span>
                </div>
                <div class="info-row">
                    <span class="info-label">{"Current Time"}</span>
                    <span class="info-value">{move || state.current_time_display.get()}</span>
                </div>
                <div class="info-row">
                    <span class="info-label">{"Speed"}</span>
                    <span class="info-value">{move || format!("{}x", state.playback_speed.get())}</span>
                </div>
            </div>
        </div>
    }
}

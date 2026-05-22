// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use leptos::prelude::*;
use crate::state::app_state::use_app_state;

/// Application bar at the top of the workspace.
#[component]
pub fn AppBar() -> impl IntoView {
    let state = use_app_state();

    let toggle_left_sidebar = move |_| {
        state.left_sidebar_open.update(|open| *open = !*open);
        log::info!("Left sidebar toggled: {}", state.left_sidebar_open.get_untracked());
    };

    let toggle_right_sidebar = move |_| {
        state.right_sidebar_open.update(|open| *open = !*open);
        log::info!("Right sidebar toggled: {}", state.right_sidebar_open.get_untracked());
    };

    let open_data_source = move |_| {
        state.data_source_dialog_open.set(true);
        log::info!("Data source dialog opened");
    };

    view! {
        <header class="app-bar">
            <div class="app-bar-left">
                <button
                    class="app-bar-button"
                    on:click=toggle_left_sidebar
                    title="Toggle left sidebar"
                >
                    <span class="icon">{"☰"}</span>
                </button>
                <div class="app-bar-logo">
                    <span class="logo-text">{"Lichtblick"}</span>
                </div>
            </div>
            <div class="app-bar-center">
                <button
                    class="app-bar-button data-source-button"
                    on:click=open_data_source
                >
                    <span class="icon">{"📂"}</span>
                    <span>{"Open data source"}</span>
                </button>
            </div>
            <div class="app-bar-right">
                <button
                    class="app-bar-button"
                    on:click=toggle_right_sidebar
                    title="Toggle right sidebar"
                >
                    <span class="icon">{"☰"}</span>
                </button>
            </div>
        </header>
    }
}

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
    };

    let toggle_right_sidebar = move |_| {
        state.right_sidebar_open.update(|open| *open = !*open);
    };

    let open_data_source = move |_| {
        state.data_source_dialog_open.set(true);
    };

    let left_title = move || {
        if state.left_sidebar_open.get() {
            "Hide left sidebar ["
        } else {
            "Show left sidebar ["
        }
    };

    let right_title = move || {
        if state.right_sidebar_open.get() {
            "Hide right sidebar ]"
        } else {
            "Show right sidebar ]"
        }
    };

    view! {
        <header class="app-bar">
            <div class="app-bar-left">
                <div class="app-bar-logo">
                    <span class="logo-text">{"Lichtblick"}</span>
                </div>
            </div>
            <div class="app-bar-center">
                <button
                    class="app-bar-button data-source-button"
                    on:click=open_data_source
                    title=move || {
                        state.current_file_name.get().unwrap_or_else(|| "Open data source".to_string())
                    }
                >
                    {move || {
                        if let Some(name) = state.current_file_name.get() {
                            view! {
                                <span class="icon">{"📄"}</span>
                                <span class="file-name-text">{name}</span>
                            }.into_any()
                        } else {
                            view! {
                                <span class="icon">{"📂"}</span>
                                <span>{"Open data source"}</span>
                            }.into_any()
                        }
                    }}
                </button>
            </div>
            <div class="app-bar-right">
                <button
                    class="app-bar-sidebar-toggle"
                    on:click=toggle_left_sidebar
                    title=left_title
                >
                    <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
                        <path d="M2 2h3v12H2V2zm5 0h7v1H7V2zm0 11h7v1H7v-1zm0-3h7v1H7v-1zm0-3h7v1H7V7zm0-3h7v1H7V4z"/>
                    </svg>
                </button>
                <button
                    class="app-bar-sidebar-toggle"
                    on:click=toggle_right_sidebar
                    title=right_title
                >
                    <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
                        <path d="M11 2h3v12h-3V2zM2 2h7v1H2V2zm0 11h7v1H2v-1zm0-3h7v1H2v-1zm0-3h7v1H2V7zm0-3h7v1H2V4z"/>
                    </svg>
                </button>
            </div>
        </header>
    }
}

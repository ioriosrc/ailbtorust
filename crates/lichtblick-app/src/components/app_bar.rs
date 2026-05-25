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

    // User menu dropdown state
    let user_menu_open = RwSignal::new(false);

    let toggle_user_menu = move |_: leptos::ev::MouseEvent| {
        user_menu_open.update(|open| *open = !*open);
    };

    let open_settings = move |_: leptos::ev::MouseEvent| {
        user_menu_open.set(false);
        state.settings_tab.set("general".to_string());
        state.settings_dialog_open.set(true);
    };

    let open_extensions = move |_: leptos::ev::MouseEvent| {
        user_menu_open.set(false);
        state.settings_tab.set("extensions".to_string());
        state.settings_dialog_open.set(true);
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
                // User profile button
                <div class="user-menu-container">
                    <button
                        class="app-bar-sidebar-toggle user-avatar-btn"
                        on:click=toggle_user_menu
                        title="Profile"
                    >
                        <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor">
                            <path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/>
                        </svg>
                    </button>
                    {move || {
                        if user_menu_open.get() {
                            Some(view! {
                                <div class="user-menu-dropdown">
                                    <button class="user-menu-item" on:click=open_settings>
                                        <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                                            <path d="M19.14 12.94c.04-.3.06-.61.06-.94 0-.32-.02-.64-.07-.94l2.03-1.58a.49.49 0 0 0 .12-.61l-1.92-3.32a.49.49 0 0 0-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54a.484.484 0 0 0-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96a.49.49 0 0 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.05.3-.09.63-.09.94s.02.64.07.94l-2.03 1.58a.49.49 0 0 0-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z"/>
                                        </svg>
                                        <span>{"Visualization settings"}</span>
                                    </button>
                                    <button class="user-menu-item" on:click=open_extensions>
                                        <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                                            <path d="M20.5 11H19V7c0-1.1-.9-2-2-2h-4V3.5C13 2.12 11.88 1 10.5 1S8 2.12 8 3.5V5H4c-1.1 0-1.99.9-1.99 2v3.8H3.5c1.49 0 2.7 1.21 2.7 2.7s-1.21 2.7-2.7 2.7H2V20c0 1.1.9 2 2 2h3.8v-1.5c0-1.49 1.21-2.7 2.7-2.7 1.49 0 2.7 1.21 2.7 2.7V22H17c1.1 0 2-.9 2-2v-4h1.5c1.38 0 2.5-1.12 2.5-2.5S21.88 11 20.5 11z"/>
                                        </svg>
                                        <span>{"Extensions"}</span>
                                    </button>
                                </div>
                            })
                        } else {
                            None
                        }
                    }}
                </div>
            </div>
        </header>
    }
}

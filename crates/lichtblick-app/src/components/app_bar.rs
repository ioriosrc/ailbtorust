// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use crate::state::app_state::{use_app_state, use_layout_state, PanelType};

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

    // App menu dropdown state
    let app_menu_open = RwSignal::new(false);
    let active_submenu = RwSignal::new(String::new());

    let toggle_app_menu = move |_: leptos::ev::MouseEvent| {
        app_menu_open.update(|open| *open = !*open);
        if !app_menu_open.get_untracked() {
            active_submenu.set(String::new());
        }
    };

    let close_app_menu = move || {
        app_menu_open.set(false);
        active_submenu.set(String::new());
    };

    // File menu actions
    let on_open = move |_: leptos::ev::MouseEvent| {
        close_app_menu();
        state.data_source_dialog_open.set(true);
    };

    let on_open_local = move |_: leptos::ev::MouseEvent| {
        close_app_menu();
        // Trigger native file picker
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            let input = document.create_element("input").ok();
            if let Some(input) = input {
                input.set_attribute("type", "file").ok();
                input.set_attribute("accept", ".mcap,.bag,.ulog").ok();
                input.set_attribute("multiple", "").ok();
                if let Ok(html_input) = input.dyn_into::<web_sys::HtmlInputElement>() {
                    html_input.click();
                }
            }
        }
    };

    let on_open_connection = move |_: leptos::ev::MouseEvent| {
        close_app_menu();
        state.data_source_dialog_open.set(true);
    };

    // View menu actions
    let on_toggle_left = move |_: leptos::ev::MouseEvent| {
        close_app_menu();
        state.left_sidebar_open.update(|open| *open = !*open);
    };

    let on_toggle_right = move |_: leptos::ev::MouseEvent| {
        close_app_menu();
        state.right_sidebar_open.update(|open| *open = !*open);
    };

    let on_import_layout = move |_: leptos::ev::MouseEvent| {
        close_app_menu();
        let layout = use_layout_state();
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            if let Some(input) = document.create_element("input").ok() {
                input.set_attribute("type", "file").ok();
                input.set_attribute("accept", ".json").ok();
                if let Ok(html_input) = input.dyn_into::<web_sys::HtmlInputElement>() {
                    let input_clone = html_input.clone();
                    let onchange = wasm_bindgen::closure::Closure::once(move |_: web_sys::Event| {
                        if let Some(files) = input_clone.files() {
                            if let Some(file) = files.get(0) {
                                let reader = web_sys::FileReader::new().unwrap();
                                let reader_clone = reader.clone();
                                let onload = wasm_bindgen::closure::Closure::once(move |_: web_sys::Event| {
                                    if let Ok(result) = reader_clone.result() {
                                        if let Some(text) = result.as_string() {
                                            crate::components::sidebar::import_layout_json(&text, &layout);
                                        }
                                    }
                                });
                                reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                                onload.forget();
                                reader.read_as_text(&file).ok();
                            }
                        }
                    });
                    html_input.set_onchange(Some(onchange.as_ref().unchecked_ref()));
                    onchange.forget();
                    html_input.click();
                }
            }
        }
    };

    // Help menu actions
    let on_about = move |_: leptos::ev::MouseEvent| {
        close_app_menu();
        state.settings_tab.set("about".to_string());
        state.settings_dialog_open.set(true);
    };

    let on_documentation = move |_: leptos::ev::MouseEvent| {
        close_app_menu();
        if let Some(window) = web_sys::window() {
            window.open_with_url_and_target("https://lichtblick-suite.github.io/docs/", "_blank").ok();
        }
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
                <div class="app-menu-container">
                    <button class="app-menu-trigger" on:click=toggle_app_menu title="Menu">
                        <span class="logo-text">{"Lichtblick"}</span>
                        <svg class="app-menu-arrow" width="10" height="10" viewBox="0 0 10 10" fill="currentColor">
                            <path d="M2 3.5L5 7L8 3.5z"/>
                        </svg>
                    </button>
                    {move || {
                        if app_menu_open.get() {
                            Some(view! {
                                <div class="app-menu-backdrop" on:click=move |_: leptos::ev::MouseEvent| { close_app_menu(); }></div>
                                <nav class="app-menu-dropdown">
                                    // File submenu
                                    <div
                                        class="app-menu-item has-submenu"
                                        on:mouseenter=move |_: leptos::ev::MouseEvent| { active_submenu.set("file".to_string()); }
                                    >
                                        <span>{"File"}</span>
                                        <svg width="8" height="8" viewBox="0 0 8 8" fill="currentColor">
                                            <path d="M2 1L6 4L2 7z"/>
                                        </svg>
                                        {move || {
                                            if active_submenu.get() == "file" {
                                                Some(view! {
                                                    <div class="app-submenu">
                                                        <button class="app-submenu-item" on:click=on_open>
                                                            <span class="submenu-label">{"Open..."}</span>
                                                        </button>
                                                        <button class="app-submenu-item" on:click=on_open_local>
                                                            <span class="submenu-label">{"Open local file(s)..."}</span>
                                                            <span class="submenu-shortcut">{"\u{2318}O"}</span>
                                                        </button>
                                                        <button class="app-submenu-item" on:click=on_open_connection>
                                                            <span class="submenu-label">{"Open connection..."}</span>
                                                            <span class="submenu-shortcut">{"\u{21E7}\u{2318}O"}</span>
                                                        </button>
                                                        <div class="app-menu-separator"></div>
                                                        <div class="app-submenu-header">{"Recent data sources"}</div>
                                                    </div>
                                                })
                                            } else {
                                                None
                                            }
                                        }}
                                    </div>
                                    // View submenu
                                    <div
                                        class="app-menu-item has-submenu"
                                        on:mouseenter=move |_: leptos::ev::MouseEvent| { active_submenu.set("view".to_string()); }
                                    >
                                        <span>{"View"}</span>
                                        <svg width="8" height="8" viewBox="0 0 8 8" fill="currentColor">
                                            <path d="M2 1L6 4L2 7z"/>
                                        </svg>
                                        {move || {
                                            if active_submenu.get() == "view" {
                                                Some(view! {
                                                    <div class="app-submenu">
                                                        <button class="app-submenu-item" on:click=on_toggle_left>
                                                            <span class="submenu-label">
                                                                {move || if state.left_sidebar_open.get() { "Hide left sidebar" } else { "Show left sidebar" }}
                                                            </span>
                                                            <span class="submenu-shortcut">{"["}</span>
                                                        </button>
                                                        <button class="app-submenu-item" on:click=on_toggle_right>
                                                            <span class="submenu-label">
                                                                {move || if state.right_sidebar_open.get() { "Hide right sidebar" } else { "Show right sidebar" }}
                                                            </span>
                                                            <span class="submenu-shortcut">{"]"}</span>
                                                        </button>
                                                        <div class="app-menu-separator"></div>
                                                        <button class="app-submenu-item" on:click=on_import_layout>
                                                            <span class="submenu-label">{"Import layout from file..."}</span>
                                                        </button>
                                                    </div>
                                                })
                                            } else {
                                                None
                                            }
                                        }}
                                    </div>
                                    // Help submenu
                                    <div
                                        class="app-menu-item has-submenu"
                                        on:mouseenter=move |_: leptos::ev::MouseEvent| { active_submenu.set("help".to_string()); }
                                    >
                                        <span>{"Help"}</span>
                                        <svg width="8" height="8" viewBox="0 0 8 8" fill="currentColor">
                                            <path d="M2 1L6 4L2 7z"/>
                                        </svg>
                                        {move || {
                                            if active_submenu.get() == "help" {
                                                Some(view! {
                                                    <div class="app-submenu">
                                                        <button class="app-submenu-item" on:click=on_about>
                                                            <span class="submenu-label">{"About"}</span>
                                                        </button>
                                                        <div class="app-menu-separator"></div>
                                                        <button class="app-submenu-item" on:click=on_documentation>
                                                            <span class="submenu-label">{"Documentation"}</span>
                                                            <svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor" style="margin-left: auto; opacity: 0.5">
                                                                <path d="M19 19H5V5h7V3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2v-7h-2v7zM14 3v2h3.59l-9.83 9.83 1.41 1.41L19 6.41V10h2V3h-7z"/>
                                                            </svg>
                                                        </button>
                                                    </div>
                                                })
                                            } else {
                                                None
                                            }
                                        }}
                                    </div>
                                </nav>
                            })
                        } else {
                            None
                        }
                    }}
                </div>
                // Add Panel button
                <AddPanelButton/>
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

/// "Add Panel" button with dropdown panel list and search.
#[component]
fn AddPanelButton() -> impl IntoView {
    let layout = use_layout_state();
    let panel_list_open = RwSignal::new(false);
    let search_query = RwSignal::new(String::new());

    let toggle_panel_list = move |_: leptos::ev::MouseEvent| {
        panel_list_open.update(|open| *open = !*open);
        search_query.set(String::new());
    };

    let close_panel_list = move || {
        panel_list_open.set(false);
        search_query.set(String::new());
    };

    let on_search_input = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                search_query.set(input.value());
            }
        }
    };

    view! {
        <div class="add-panel-container">
            <button
                class="app-bar-sidebar-toggle add-panel-btn"
                on:click=toggle_panel_list
                title="Add panel"
            >
                <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
                    <path d="M14 7H9V2H7v5H2v2h5v5h2V9h5z"/>
                </svg>
            </button>
            {move || {
                if panel_list_open.get() {
                    let panels = PanelType::all();
                    Some(view! {
                        <div class="add-panel-backdrop" on:click=move |_: leptos::ev::MouseEvent| { close_panel_list(); }></div>
                        <div class="add-panel-dropdown">
                            <div class="add-panel-search">
                                <input
                                    type="text"
                                    class="add-panel-search-input"
                                    placeholder="Search panels..."
                                    on:input=on_search_input
                                    prop:value=move || search_query.get()
                                />
                            </div>
                            <div class="add-panel-list">
                                {panels.iter().map(|pt| {
                                    let panel_type = pt.clone();
                                    let display_name = pt.display_name().to_string();
                                    let name_for_filter = display_name.clone();
                                    let query = search_query;
                                    let layout_clone = layout;
                                    view! {
                                        <button
                                            class="add-panel-item"
                                            style:display=move || {
                                                let q = query.get().to_lowercase();
                                                if q.is_empty() || name_for_filter.to_lowercase().contains(&q) {
                                                    "flex"
                                                } else {
                                                    "none"
                                                }
                                            }
                                            on:click=move |_: leptos::ev::MouseEvent| {
                                                layout_clone.add_panel(panel_type.clone());
                                                close_panel_list();
                                            }
                                        >
                                            <span>{display_name.clone()}</span>
                                        </button>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>
                    })
                } else {
                    None
                }
            }}
        </div>
    }
}

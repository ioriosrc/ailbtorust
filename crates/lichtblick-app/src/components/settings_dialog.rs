// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::state::app_state::{use_app_state, TimeFormat};

/// Settings dialog with tabs: General, Extensions, Experimental, About.
#[component]
pub fn SettingsDialog() -> impl IntoView {
    let state = use_app_state();
    let is_open = state.settings_dialog_open;
    let active_tab = state.settings_tab;

    let close = move |_: leptos::ev::MouseEvent| {
        state.settings_dialog_open.set(false);
    };

    let set_tab_general = move |_: leptos::ev::MouseEvent| {
        state.settings_tab.set("general".to_string());
    };
    let set_tab_extensions = move |_: leptos::ev::MouseEvent| {
        state.settings_tab.set("extensions".to_string());
    };
    let set_tab_experimental = move |_: leptos::ev::MouseEvent| {
        state.settings_tab.set("experimental".to_string());
    };
    let set_tab_about = move |_: leptos::ev::MouseEvent| {
        state.settings_tab.set("about".to_string());
    };

    let on_done = move |_: leptos::ev::MouseEvent| {
        // Persist all settings to localStorage
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                // Color scheme
                let scheme_str = match state.color_scheme.get() {
                    lichtblick_core::settings::ColorScheme::Light => "light",
                    lichtblick_core::settings::ColorScheme::Dark => "dark",
                    lichtblick_core::settings::ColorScheme::System => "system",
                };
                storage.set_item("lichtblick:color_scheme", scheme_str).ok();

                // Timezone
                storage.set_item("lichtblick:timezone", &state.timezone.get()).ok();

                // Time format
                storage.set_item("lichtblick:time_format", state.time_format.get().as_str()).ok();

                // Message rate
                storage.set_item("lichtblick:message_rate", &state.message_rate.get().to_string()).ok();

                // Step size
                storage.set_item("lichtblick:step_size_ms", &state.step_size_ms.get().to_string()).ok();

                // Debug panels
                storage.set_item("lichtblick:debug_panels", if state.debug_panels_enabled.get() { "true" } else { "false" }).ok();

                // Memory indicator
                storage.set_item("lichtblick:memory_indicator", if state.memory_indicator_enabled.get() { "true" } else { "false" }).ok();
            }
        }
        state.settings_dialog_open.set(false);
    };

    view! {
        <div
            class="dialog-overlay"
            class:hidden=move || !is_open.get()
            on:click=close
        >
            <div class="settings-dialog" on:click=|e: leptos::ev::MouseEvent| e.stop_propagation()>
                <div class="settings-header">
                    <h2 class="settings-title">{"Settings"}</h2>
                    <button class="settings-close-btn" on:click=close>
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                            <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
                        </svg>
                    </button>
                </div>
                <div class="settings-body">
                    <nav class="settings-sidebar">
                        <button
                            class="settings-tab-btn"
                            class:active=move || active_tab.get() == "general"
                            on:click=set_tab_general
                        >
                            {"General"}
                        </button>
                        <button
                            class="settings-tab-btn"
                            class:active=move || active_tab.get() == "extensions"
                            on:click=set_tab_extensions
                        >
                            {"Extensions"}
                        </button>
                        <button
                            class="settings-tab-btn"
                            class:active=move || active_tab.get() == "experimental"
                            on:click=set_tab_experimental
                        >
                            {"Experimental features"}
                        </button>
                        <button
                            class="settings-tab-btn"
                            class:active=move || active_tab.get() == "about"
                            on:click=set_tab_about
                        >
                            {"About"}
                        </button>
                    </nav>
                    <div class="settings-content">
                        {move || {
                            let tab = active_tab.get();
                            match tab.as_str() {
                                "general" => view! { <GeneralSettings/> }.into_any(),
                                "extensions" => view! { <ExtensionsSettings/> }.into_any(),
                                "experimental" => view! { <ExperimentalSettings/> }.into_any(),
                                "about" => view! { <AboutSettings/> }.into_any(),
                                _ => view! { <GeneralSettings/> }.into_any(),
                            }
                        }}
                    </div>
                </div>
                <div class="settings-footer">
                    <button class="settings-done-btn" on:click=on_done>
                        {"Done"}
                    </button>
                </div>
            </div>
        </div>
    }
}

/// General settings tab content.
#[component]
fn GeneralSettings() -> impl IntoView {
    let state = use_app_state();

    let on_color_scheme_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(select) = target.dyn_into::<web_sys::HtmlSelectElement>() {
                let scheme = match select.value().as_str() {
                    "light" => lichtblick_core::settings::ColorScheme::Light,
                    "system" => lichtblick_core::settings::ColorScheme::System,
                    _ => lichtblick_core::settings::ColorScheme::Dark,
                };
                state.color_scheme.set(scheme);
            }
        }
    };

    let on_timezone_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(select) = target.dyn_into::<web_sys::HtmlSelectElement>() {
                state.timezone.set(select.value());
            }
        }
    };

    let on_time_format_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(select) = target.dyn_into::<web_sys::HtmlSelectElement>() {
                let new_format = TimeFormat::from_str(&select.value());
                state.time_format.set(new_format);
            }
        }
    };

    let on_message_rate_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(select) = target.dyn_into::<web_sys::HtmlSelectElement>() {
                if let Ok(rate) = select.value().parse::<u32>() {
                    state.message_rate.set(rate);
                }
            }
        }
    };

    let on_step_size_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(val) = input.value().parse::<u32>() {
                    state.step_size_ms.set(val);
                }
            }
        }
    };

    let on_debug_toggle = move |_: leptos::ev::Event| {
        state.debug_panels_enabled.update(|v| *v = !*v);
    };

    // Detect system timezone for display
    let system_tz = {
        let tz = js_sys::Intl::DateTimeFormat::new(&js_sys::Array::new(), &js_sys::Object::new())
            .resolved_options();
        js_sys::Reflect::get(&tz, &"timeZone".into())
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_else(|| "UTC".to_string())
    };
    let detect_label = format!("Detect from system: {}", system_tz);

    view! {
        <div class="settings-section">
            <h3 class="settings-section-title">{"Color scheme"}</h3>
            <div class="settings-row">
                <select class="settings-select" on:change=on_color_scheme_change>
                    <option value="dark" selected=move || matches!(state.color_scheme.get(), lichtblick_core::settings::ColorScheme::Dark)>
                        {"Dark"}
                    </option>
                    <option value="light" selected=move || matches!(state.color_scheme.get(), lichtblick_core::settings::ColorScheme::Light)>
                        {"Light"}
                    </option>
                    <option value="system" selected=move || matches!(state.color_scheme.get(), lichtblick_core::settings::ColorScheme::System)>
                        {"Follow system"}
                    </option>
                </select>
            </div>
        </div>
        <div class="settings-section">
            <h3 class="settings-section-title">{"Display timestamps in"}</h3>
            <div class="settings-row">
                <select class="settings-select" on:change=on_timezone_change>
                    <option value="Detect from system" selected=move || state.timezone.get() == "Detect from system">
                        {detect_label.clone()}
                    </option>
                    <option value="UTC" selected=move || state.timezone.get() == "UTC">
                        {"UTC"}
                    </option>
                    <option value="Europe/Lisbon" selected=move || state.timezone.get() == "Europe/Lisbon">
                        {"Europe/Lisbon (WEST, +01:00)"}
                    </option>
                    <option value="Europe/Berlin" selected=move || state.timezone.get() == "Europe/Berlin">
                        {"Europe/Berlin (CEST, +02:00)"}
                    </option>
                    <option value="America/New_York" selected=move || state.timezone.get() == "America/New_York">
                        {"America/New_York (EDT, -04:00)"}
                    </option>
                    <option value="America/Chicago" selected=move || state.timezone.get() == "America/Chicago">
                        {"America/Chicago (CDT, -05:00)"}
                    </option>
                    <option value="America/Los_Angeles" selected=move || state.timezone.get() == "America/Los_Angeles">
                        {"America/Los_Angeles (PDT, -07:00)"}
                    </option>
                    <option value="Asia/Tokyo" selected=move || state.timezone.get() == "Asia/Tokyo">
                        {"Asia/Tokyo (JST, +09:00)"}
                    </option>
                    <option value="Asia/Shanghai" selected=move || state.timezone.get() == "Asia/Shanghai">
                        {"Asia/Shanghai (CST, +08:00)"}
                    </option>
                </select>
            </div>
        </div>
        <div class="settings-section">
            <h3 class="settings-section-title">{"Timestamp format"}</h3>
            <div class="settings-row">
                <select class="settings-select" on:change=on_time_format_change>
                    <option value="TOD" selected=move || state.time_format.get() == TimeFormat::TOD>
                        {"Time of day (TOD)"}
                    </option>
                    <option value="SEC" selected=move || state.time_format.get() == TimeFormat::SEC>
                        {"Seconds (SEC)"}
                    </option>
                </select>
            </div>
        </div>
        <div class="settings-section">
            <h3 class="settings-section-title">{"Message rate (Hz)"}</h3>
            <div class="settings-row">
                <select class="settings-select" on:change=on_message_rate_change>
                    <option value="1" selected=move || state.message_rate.get() == 1>{"1"}</option>
                    <option value="3" selected=move || state.message_rate.get() == 3>{"3"}</option>
                    <option value="5" selected=move || state.message_rate.get() == 5>{"5"}</option>
                    <option value="10" selected=move || state.message_rate.get() == 10>{"10"}</option>
                    <option value="15" selected=move || state.message_rate.get() == 15>{"15"}</option>
                    <option value="20" selected=move || state.message_rate.get() == 20>{"20"}</option>
                    <option value="30" selected=move || state.message_rate.get() == 30>{"30"}</option>
                    <option value="60" selected=move || state.message_rate.get() == 60>{"60"}</option>
                </select>
            </div>
        </div>
        <div class="settings-section">
            <h3 class="settings-section-title">{"Step size (ms)"}</h3>
            <div class="settings-row">
                <input
                    type="number"
                    class="settings-input"
                    min="1"
                    max="10000"
                    prop:value=move || state.step_size_ms.get().to_string()
                    on:change=on_step_size_change
                />
            </div>
        </div>
        <div class="settings-section">
            <h3 class="settings-section-title">{"Language"}</h3>
            <div class="settings-row">
                <select class="settings-select">
                    <option value="en" selected=true>{"English"}</option>
                </select>
            </div>
        </div>
        <div class="settings-section">
            <h3 class="settings-section-title">{"Advanced"}</h3>
            <label class="settings-checkbox-row">
                <input
                    type="checkbox"
                    prop:checked=move || state.debug_panels_enabled.get()
                    on:change=on_debug_toggle
                />
                <span>{"Enable panels and features for debugging Lichtblick"}</span>
            </label>
        </div>
    }
}

/// Extensions settings tab content.
#[component]
fn ExtensionsSettings() -> impl IntoView {
    let search_query = RwSignal::new(String::new());

    let on_search_input = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                search_query.set(input.value());
            }
        }
    };

    let extensions = vec![
        ("URDF Viewer", "1.0.0", "Lichtblick", "Visualize URDF robot models"),
        ("ROS 2", "1.0.0", "Lichtblick", "ROS 2 connectivity and message support"),
        ("PointCloud", "1.0.0", "Lichtblick", "Point cloud visualization"),
        ("Image Annotations", "1.0.0", "Lichtblick", "Image annotation overlays"),
        ("Map", "1.0.0", "Lichtblick", "Map tile visualization panel"),
    ];

    view! {
        <div class="extensions-search">
            <input
                type="text"
                class="extensions-search-input"
                placeholder="Search extensions..."
                on:input=on_search_input
                prop:value=move || search_query.get()
            />
        </div>
        <div class="extensions-table">
            <div class="extensions-table-header">
                <span class="ext-col-name">{"Name"}</span>
                <span class="ext-col-version">{"Version"}</span>
                <span class="ext-col-publisher">{"Publisher"}</span>
                <span class="ext-col-description">{"Description"}</span>
            </div>
            {extensions.into_iter().map(|(name, version, publisher, desc)| {
                let name_s = name.to_string();
                let query = search_query;
                view! {
                    <div class="extensions-table-row"
                        style:display=move || {
                            let q = query.get().to_lowercase();
                            if q.is_empty() || name_s.to_lowercase().contains(&q) {
                                "grid"
                            } else {
                                "none"
                            }
                        }
                    >
                        <span class="ext-col-name">{name}</span>
                        <span class="ext-col-version">{version}</span>
                        <span class="ext-col-publisher">{publisher}</span>
                        <span class="ext-col-description">{desc}</span>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

/// Experimental features settings tab content.
#[component]
fn ExperimentalSettings() -> impl IntoView {
    let state = use_app_state();

    let on_memory_toggle = move |_: leptos::ev::Event| {
        state.memory_indicator_enabled.update(|v| *v = !*v);
    };

    view! {
        <div class="settings-section">
            <h3 class="settings-section-title">{"Experimental features"}</h3>
            <div class="settings-warning">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor" class="warning-icon">
                    <path d="M1 21h22L12 2 1 21zm12-3h-2v-2h2v2zm0-4h-2v-4h2v4z"/>
                </svg>
                <span>{"These features are unstable and not recommended for daily use."}</span>
            </div>
            <div class="settings-experimental-list">
                <label class="settings-checkbox-row">
                    <input
                        type="checkbox"
                        prop:checked=move || state.memory_indicator_enabled.get()
                        on:change=on_memory_toggle
                    />
                    <div class="settings-checkbox-label">
                        <span class="settings-checkbox-title">{"Memory use indicator"}</span>
                        <span class="settings-checkbox-desc">{"Show the app memory use in the sidebar."}</span>
                    </div>
                </label>
            </div>
        </div>
    }
}

/// About settings tab content.
#[component]
fn AboutSettings() -> impl IntoView {
    view! {
        <div class="settings-about">
            <div class="about-logo">
                <svg width="48" height="48" viewBox="0 0 24 24" fill="var(--accent-color, #5b9bd5)">
                    <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 17.93c-3.95-.49-7-3.85-7-7.93 0-.62.08-1.21.21-1.79L9 15v1c0 1.1.9 2 2 2v1.93zm6.9-2.54c-.26-.81-1-1.39-1.9-1.39h-1v-3c0-.55-.45-1-1-1H8v-2h2c.55 0 1-.45 1-1V7h2c1.1 0 2-.9 2-2v-.41c2.93 1.19 5 4.06 5 7.41 0 2.08-.8 3.97-2.1 5.39z"/>
                </svg>
            </div>
            <h2 class="about-title">{"Lichtblick"}</h2>
            <p class="about-version">{format!("Version {} (Rust WASM)", env!("CARGO_PKG_VERSION"))}</p>
            <div class="about-section">
                <h4>{"Documentation"}</h4>
                <p class="about-link">{"Check out the Lichtblick documentation for guides and API references."}</p>
            </div>
            <div class="about-section">
                <h4>{"Legal"}</h4>
                <p class="about-link">{"Licensed under the Mozilla Public License 2.0 (MPL-2.0)"}</p>
            </div>
        </div>
    }
}

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use crate::decoder::is_compressed_image_schema;
use crate::components::topic_list::TopicList;
use crate::state::app_state::{
    get_player, use_app_state, use_layout_state,
    AppState, LayoutNode, LayoutState, NodeId, PanelType, SplitDirection,
    parse_layout_node_internal,
};
use lichtblick_panels::three_dee::TopicDisplayConfig;
use crate::panels::three_dee_panel::get_tf_frame_metadata;

/// Sidebar component (left or right) with drag-resizable width.
#[component]
pub fn Sidebar(
    #[prop(into)] side: String,
    open: RwSignal<bool>,
) -> impl IntoView {
    let side_for_class = side.clone();
    let is_left = side == "left";
    let _state = use_app_state();

    // Custom width (None = use CSS default)
    let custom_width = RwSignal::new(None::<f64>);
    let is_dragging = RwSignal::new(false);

    let class = move || {
        let base = format!("sidebar sidebar-{}", side_for_class);
        if open.get() {
            format!("{} open", base)
        } else {
            base
        }
    };

    let style = move || {
        if open.get() {
            if let Some(w) = custom_width.get() {
                format!("width: {}px;", w)
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    };

    // Resize handle drag logic
    let side_is_left = is_left;
    let on_mousedown = move |ev: web_sys::MouseEvent| {
        ev.prevent_default();
        is_dragging.set(true);

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        // Add dragging class to body to prevent text selection
        let _ = document.body().unwrap().class_list().add_1("sidebar-resizing");

        let move_cb: Closure<dyn FnMut(web_sys::MouseEvent)> = Closure::new(move |e: web_sys::MouseEvent| {
            if !is_dragging.get_untracked() {
                return;
            }
            let x = e.client_x() as f64;
            let new_width = if side_is_left {
                x
            } else {
                let win_width = web_sys::window().unwrap().inner_width().unwrap().as_f64().unwrap();
                win_width - x
            };
            // Clamp between 200 and 600px
            let clamped = new_width.max(200.0).min(600.0);
            custom_width.set(Some(clamped));
        });

        let move_cb_ref = move_cb.as_ref().unchecked_ref::<js_sys::Function>().clone();
        let _ = document.add_event_listener_with_callback("mousemove", &move_cb_ref);

        // Store closures so they stay alive
        let up_cb: Closure<dyn FnMut(web_sys::MouseEvent)> = Closure::once(move |_e: web_sys::MouseEvent| {
            is_dragging.set(false);
            let doc = web_sys::window().unwrap().document().unwrap();
            let _ = doc.body().unwrap().class_list().remove_1("sidebar-resizing");
            let _ = doc.remove_event_listener_with_callback("mousemove", &move_cb_ref);
            drop(move_cb); // prevent premature drop
        });

        let mut opts = web_sys::AddEventListenerOptions::new();
        opts.once(true);
        let _ = document.add_event_listener_with_callback_and_add_event_listener_options(
            "mouseup",
            up_cb.as_ref().unchecked_ref(),
            &opts,
        );
        up_cb.forget(); // safe: runs once then removed
    };

    view! {
        <aside class=class style=style>
            <div class="sidebar-content">
                {if is_left {
                    view! {
                        <div class="sidebar-panel">
                            <LeftSidebarContent />
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="sidebar-panel">
                            <VariablesPanel />
                        </div>
                    }.into_any()
                }}
            </div>
            {if is_left {
                view! {
                    <div class="sidebar-resize-handle" on:mousedown=on_mousedown></div>
                }.into_any()
            } else {
                view! {
                    <div class="sidebar-resize-handle sidebar-resize-left" on:mousedown=on_mousedown></div>
                }.into_any()
            }}
        </aside>
    }
}

/// Left sidebar content: tabs (Panel | Topics | Alerts | Layouts).
#[component]
fn LeftSidebarContent() -> impl IntoView {
    let state = use_app_state();
    let layout = use_layout_state();
    let active_tab = state.left_sidebar_tab;

    // When a panel's settings gear is clicked, switch to Panel tab
    Effect::new(move |_| {
        if layout.active_settings_panel.get().is_some() {
            active_tab.set(0);
        }
    });

    // Compute alert count reactively
    let alert_count = move || {
        let _tick = state.frame_tick.get();
        let player = match get_player() {
            Some(p) => p,
            None => return 0usize,
        };
        let topics = player.topics();
        let stats = player.topic_stats();
        let (start_ns, end_ns) = player.time_range();
        let duration_secs = (end_ns.saturating_sub(start_ns)) as f64 / 1_000_000_000.0;
        let mut count = 0usize;
        if duration_secs > 0.0 {
            let has_high_freq = topics.iter().any(|t| {
                if t.schema_name.is_empty() || matches!(t.schema_name.as_str(),
                    "rosgraph_msgs/Log" | "rosgraph_msgs/msg/Log" |
                    "rcl_interfaces/msg/Log" | "foxglove.Log"
                ) {
                    return false;
                }
                if let Some(&(_count, hz)) = stats.get(&t.name) {
                    return hz > 60.0;
                }
                false
            });
            if has_high_freq {
                count += 1;
            }
        }
        count
    };

    view! {
        <div class="sidebar-tabs">
            <div class="sidebar-tab-bar">
                <button
                    class="sidebar-tab-btn"
                    class:active=move || active_tab.get() == 0
                    on:click=move |_| active_tab.set(0)
                >{"Panel"}</button>
                <button
                    class="sidebar-tab-btn"
                    class:active=move || active_tab.get() == 1
                    on:click=move |_| active_tab.set(1)
                >{"Topics"}</button>
                <button
                    class="sidebar-tab-btn sidebar-tab-alerts"
                    class:active=move || active_tab.get() == 2
                    on:click=move |_| active_tab.set(2)
                >
                    {"Alerts"}
                    {move || {
                        let count = alert_count();
                        if count > 0 {
                            Some(view! {
                                <span class="alert-badge">{count.to_string()}</span>
                            })
                        } else {
                            None
                        }
                    }}
                </button>
                <button
                    class="sidebar-tab-btn"
                    class:active=move || active_tab.get() == 3
                    on:click=move |_| active_tab.set(3)
                >{"Layouts"}</button>
            </div>
            <div class="sidebar-tab-content">
                {move || match active_tab.get() {
                    0 => view! { <PanelTabContent /> }.into_any(),
                    1 => view! { <TopicsTabContent /> }.into_any(),
                    2 => view! { <AlertsTabContent /> }.into_any(),
                    3 => view! { <LayoutsTabContent /> }.into_any(),
                    _ => view! { <div></div> }.into_any(),
                }}
            </div>
        </div>
    }
}

/// Panel tab: shows "Select a panel" or the panel's settings.
#[component]
fn PanelTabContent() -> impl IntoView {
    let layout = use_layout_state();

    view! {
        {move || {
            let settings_panel = layout.active_settings_panel.get();
            if let Some(panel_id) = settings_panel {
                let tree = layout.tree.get();
                if let Some(node) = find_panel_in_tree(&tree, panel_id) {
                    view! { <PanelSettingsView node_id=panel_id node=node /> }.into_any()
                } else {
                    layout.active_settings_panel.set(None);
                    view! {
                        <div class="sidebar-empty">
                            <p>{"Select a panel to edit its settings."}</p>
                        </div>
                    }.into_any()
                }
            } else {
                view! {
                    <div class="sidebar-empty">
                        <p>{"Select a panel to edit its settings."}</p>
                    </div>
                }.into_any()
            }
        }}
    }
}

/// Topics tab: topic list with Hz and message count.
#[component]
fn TopicsTabContent() -> impl IntoView {
    let state = use_app_state();

    view! {
        {move || {
            if state.has_active_layout.get() {
                view! { <TopicList /> }.into_any()
            } else {
                view! {
                    <div class="sidebar-empty">
                        <p>{"Open a data source to see topics"}</p>
                    </div>
                }.into_any()
            }
        }}
    }
}

/// Alerts tab: shows performance and data alerts.
#[component]
fn AlertsTabContent() -> impl IntoView {
    let state = use_app_state();

    // Log schemas that are excluded from the high-frequency check
    let is_log_schema = |schema: &str| -> bool {
        matches!(schema,
            "rosgraph_msgs/Log" | "rosgraph_msgs/msg/Log" |
            "rcl_interfaces/msg/Log" | "foxglove.Log"
        )
    };

    view! {
        <div class="alerts-list">
            {move || {
                let _tick = state.frame_tick.get();
                let player = match get_player() {
                    Some(p) => p,
                    None => return view! {
                        <div class="sidebar-empty">
                            <p>{"No alerts."}</p>
                        </div>
                    }.into_any(),
                };

                let topics = player.topics();
                let stats = player.topic_stats();
                let (start_ns, end_ns) = player.time_range();
                let duration_secs = (end_ns.saturating_sub(start_ns)) as f64 / 1_000_000_000.0;

                let mut alerts: Vec<(String, String, String)> = Vec::new(); // (severity, title, message)

                // Check for high-frequency topics (> 60 Hz)
                if duration_secs > 0.0 {
                    let has_high_freq = topics.iter().any(|t| {
                        if t.schema_name.is_empty() || is_log_schema(&t.schema_name) {
                            return false;
                        }
                        if let Some(&(_count, hz)) = stats.get(&t.name) {
                            return hz > 60.0;
                        }
                        false
                    });

                    if has_high_freq {
                        alerts.push((
                            "warn".to_string(),
                            "High frequency topics detected".to_string(),
                            "The current data source has one or more topics with message frequency higher than 60Hz, which may impact performance and application memory.".to_string(),
                        ));
                    }
                }

                if alerts.is_empty() {
                    return view! {
                        <div class="sidebar-empty">
                            <p>{"No alerts."}</p>
                        </div>
                    }.into_any();
                }

                view! {
                    <div class="alerts-items">
                        {alerts.into_iter().map(|(severity, title, message)| {
                            let icon = match severity.as_str() {
                                "warn" => "⚠️",
                                "error" => "❌",
                                _ => "ℹ️",
                            };
                            view! {
                                <div class={format!("alert-item alert-{}", severity)}>
                                    <div class="alert-header">
                                        <span class="alert-icon">{icon}</span>
                                        <span class="alert-title">{title}</span>
                                    </div>
                                    <p class="alert-message">{message}</p>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                }.into_any()
            }}
        </div>
    }
}

/// Layouts tab: manage saved layouts with dirty state, save/revert/rename/export/delete.
#[component]
fn LayoutsTabContent() -> impl IntoView {
    let layout = use_layout_state();
    let _state = use_app_state();

    let popover_open = RwSignal::new(false);
    let rename_mode = RwSignal::new(false);
    let rename_value = RwSignal::new(String::new());
    let menu_open_for = RwSignal::new(Option::<String>::None);
    let menu_rename_mode = RwSignal::new(Option::<String>::None);
    let menu_rename_value = RwSignal::new(String::new());
    let menu_pos_top = RwSignal::new(0i32);
    let menu_pos_left = RwSignal::new(0i32);

    let on_save = move |_: leptos::ev::MouseEvent| {
        layout.save_current();
        popover_open.set(false);
    };

    let on_revert = move |_: leptos::ev::MouseEvent| {
        layout.revert();
        popover_open.set(false);
    };

    let on_rename_start = move |_: leptos::ev::MouseEvent| {
        rename_value.set(layout.current_layout_name.get_untracked());
        rename_mode.set(true);
        popover_open.set(false);
    };

    let on_rename_confirm = move |_: leptos::ev::MouseEvent| {
        let new_name = rename_value.get_untracked();
        if !new_name.trim().is_empty() {
            layout.rename_current(new_name.trim().to_string());
        }
        rename_mode.set(false);
    };

    let on_rename_cancel = move |_: leptos::ev::MouseEvent| {
        rename_mode.set(false);
    };

    let on_export = move |_: leptos::ev::MouseEvent| {
        let json = layout.export_json();
        let blob_parts = js_sys::Array::new();
        blob_parts.push(&wasm_bindgen::JsValue::from_str(&json));
        let opts = web_sys::BlobPropertyBag::new();
        opts.set_type("application/json");
        if let Ok(blob) = web_sys::Blob::new_with_str_sequence_and_options(&blob_parts, &opts) {
            if let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob) {
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                let a = document.create_element("a").unwrap();
                a.set_attribute("href", &url).ok();
                let name = layout.current_layout_name.get_untracked();
                a.set_attribute("download", &format!("{}.json", name)).ok();
                let a: web_sys::HtmlElement = a.dyn_into().unwrap();
                a.click();
                web_sys::Url::revoke_object_url(&url).ok();
            }
        }
        popover_open.set(false);
    };

    let on_delete = move |_: leptos::ev::MouseEvent| {
        let name = layout.current_layout_name.get_untracked();
        layout.delete_layout(&name);
        popover_open.set(false);
    };

    let on_create_new = move |_: leptos::ev::MouseEvent| {
        // Prompt for name via window.prompt
        let window = web_sys::window().unwrap();
        if let Ok(Some(name)) = window.prompt_with_message("Layout name:") {
            if !name.trim().is_empty() {
                // Save current as the new name
                layout.current_layout_name.set(name.trim().to_string());
                layout.save_current();
            }
        }
    };

    let on_import = move |_: leptos::ev::MouseEvent| {
        let document = web_sys::window().unwrap().document().unwrap();
        let input = document.create_element("input").unwrap();
        let input: web_sys::HtmlInputElement = input.dyn_into().unwrap();
        input.set_type("file");
        input.set_attribute("accept", ".json").ok();

        let layout_clone = layout;
        let input_clone = input.clone();
        let onchange = wasm_bindgen::closure::Closure::once(move |_: web_sys::Event| {
            let files = input_clone.files().unwrap();
            if let Some(file) = files.get(0) {
                let reader = web_sys::FileReader::new().unwrap();
                let reader_clone = reader.clone();
                let layout_for_load = layout_clone;
                let onload = wasm_bindgen::closure::Closure::once(move |_: web_sys::Event| {
                    if let Ok(result) = reader_clone.result() {
                        if let Some(text) = result.as_string() {
                            import_layout_json(&text, &layout_for_load);
                        }
                    }
                });
                reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                onload.forget();
                reader.read_as_text(&file).ok();
            }
        });
        input.set_onchange(Some(onchange.as_ref().unchecked_ref()));
        onchange.forget();
        input.click();
    };

    view! {
        <div class="layouts-list">
            // Current layout header with rename support
            <div class="layout-current-header">
                {move || {
                    if rename_mode.get() {
                        view! {
                            <div class="layout-rename-row">
                                <input
                                    class="layout-rename-input"
                                    type="text"
                                    prop:value=move || rename_value.get()
                                    on:input=move |ev| {
                                        let val = event_target_value(&ev);
                                        rename_value.set(val);
                                    }
                                    on:keydown=move |ev: leptos::ev::KeyboardEvent| {
                                        if ev.key() == "Enter" {
                                            let new_name = rename_value.get_untracked();
                                            if !new_name.trim().is_empty() {
                                                layout.rename_current(new_name.trim().to_string());
                                            }
                                            rename_mode.set(false);
                                        } else if ev.key() == "Escape" {
                                            rename_mode.set(false);
                                        }
                                    }
                                />
                                <button class="layout-rename-btn" on:click=on_rename_confirm>{"✓"}</button>
                                <button class="layout-rename-btn" on:click=on_rename_cancel>{"✕"}</button>
                            </div>
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }
                }}
            </div>

            <hr class="layouts-divider" />

            // Actions row
            <div class="layouts-actions">
                <button class="layout-action-btn" on:click=on_create_new>
                    <span class="layout-action-icon">{"+"}</span>
                    {"Create new layout"}
                </button>
                <button class="layout-action-btn" on:click=on_import>
                    <span class="layout-action-icon">{"📁"}</span>
                    {"Import from file"}
                </button>
            </div>

            <hr class="layouts-divider" />

            // Saved layouts list
            <div class="layouts-saved">
                {move || {
                    let names = layout.saved_layout_names.get();
                    let current = layout.current_layout_name.get();
                    let is_dirty = layout.is_dirty.get();
                    let inline_rename = menu_rename_mode.get();
                    names.into_iter().map(|name| {
                        let is_active = name == current;
                        let show_dirty = is_active && is_dirty;
                        let name_for_click = name.clone();
                        let name_for_menu = name.clone();
                        let layout_c = layout;
                        let on_switch = move |_: leptos::ev::MouseEvent| {
                            if !is_active {
                                layout_c.switch_to_layout(&name_for_click);
                            }
                        };
                        let on_dots_click = move |ev: leptos::ev::MouseEvent| {
                            ev.stop_propagation();
                            let target = ev.current_target().unwrap();
                            let el: web_sys::HtmlElement = target.dyn_into().unwrap();
                            let rect = el.get_bounding_client_rect();
                            menu_pos_top.set(rect.bottom() as i32);
                            menu_pos_left.set((rect.right() as i32) - 160);
                            let n = name_for_menu.clone();
                            menu_open_for.update(|v| {
                                if v.as_deref() == Some(&n) {
                                    *v = None;
                                } else {
                                    *v = Some(n);
                                }
                            });
                            popover_open.set(false);
                        };
                        let on_dirty_dot_click = move |ev: leptos::ev::MouseEvent| {
                            ev.stop_propagation();
                            let target = ev.current_target().unwrap();
                            let el: web_sys::HtmlElement = target.dyn_into().unwrap();
                            let rect = el.get_bounding_client_rect();
                            menu_pos_top.set(rect.bottom() as i32);
                            menu_pos_left.set((rect.right() as i32) - 160);
                            popover_open.update(|v| *v = !*v);
                            menu_open_for.set(None);
                        };
                        let active_class = if is_active { " layout-item-active" } else { "" };
                        let is_renaming = inline_rename.as_deref() == Some(&name);
                        view! {
                            <div class="layout-item-wrapper">
                                <div
                                    class=format!("layout-item{}", active_class)
                                    on:click=on_switch
                                >
                                    <span class="layout-item-icon">{"📐"}</span>
                                    {if is_renaming {
                                        let name_orig = name.clone();
                                        let on_rename_confirm_item = move |_: leptos::ev::MouseEvent| {
                                            let new_name = menu_rename_value.get_untracked();
                                            if !new_name.trim().is_empty() && new_name.trim() != name_orig {
                                                let current_name = layout.current_layout_name.get_untracked();
                                                if current_name != name_orig {
                                                    layout.switch_to_layout(&name_orig);
                                                }
                                                layout.rename_current(new_name.trim().to_string());
                                            }
                                            menu_rename_mode.set(None);
                                        };
                                        let on_rename_cancel_item = move |_: leptos::ev::MouseEvent| {
                                            menu_rename_mode.set(None);
                                        };
                                        view! {
                                            <input
                                                class="layout-rename-input"
                                                type="text"
                                                prop:value=move || menu_rename_value.get()
                                                on:input=move |ev| {
                                                    menu_rename_value.set(event_target_value(&ev));
                                                }
                                                on:keydown=move |ev: leptos::ev::KeyboardEvent| {
                                                    if ev.key() == "Enter" {
                                                        let new_name = menu_rename_value.get_untracked();
                                                        if !new_name.trim().is_empty() {
                                                            let current_name = layout.current_layout_name.get_untracked();
                                                            let orig = menu_rename_mode.get_untracked().unwrap_or_default();
                                                            if current_name != orig {
                                                                layout.switch_to_layout(&orig);
                                                            }
                                                            layout.rename_current(new_name.trim().to_string());
                                                        }
                                                        menu_rename_mode.set(None);
                                                    } else if ev.key() == "Escape" {
                                                        menu_rename_mode.set(None);
                                                    }
                                                }
                                                on:click=move |ev: leptos::ev::MouseEvent| { ev.stop_propagation(); }
                                            />
                                            <button class="layout-rename-btn" on:click=on_rename_confirm_item>{"✓"}</button>
                                            <button class="layout-rename-btn" on:click=on_rename_cancel_item>{"✕"}</button>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <span class="layout-item-name">{name.clone()}</span>
                                            {if show_dirty {
                                                view! {
                                                    <span
                                                        class="layout-dirty-dot"
                                                        title="This layout has unsaved changes"
                                                        on:click=on_dirty_dot_click
                                                    >{"●"}</span>
                                                }.into_any()
                                            } else {
                                                view! {
                                                    <span
                                                        class="layout-dots-menu"
                                                        title="Layout options"
                                                        on:click=on_dots_click
                                                    >{"⋮"}</span>
                                                }.into_any()
                                            }}
                                        }.into_any()
                                    }}
                                </div>
                            </div>
                        }
                    }).collect_view()
                }}
            </div>

            // Fixed-position floating menu (rendered outside scroll container via fixed pos)
            {move || {
                let open_menu = menu_open_for.get();
                let is_popover = popover_open.get();
                let top = menu_pos_top.get();
                let left = menu_pos_left.get();
                let style = format!("position:fixed;top:{}px;left:{}px;z-index:10000;", top, left);

                if open_menu.is_some() {
                    let menu_name = open_menu.unwrap();
                    let name_for_rename = menu_name.clone();
                    let name_for_dup = menu_name.clone();
                    let name_for_export = menu_name.clone();
                    let name_for_del = menu_name.clone();

                    let close_menu = move |_: leptos::ev::MouseEvent| {
                        menu_open_for.set(None);
                    };
                    let do_rename = move |ev: leptos::ev::MouseEvent| {
                        ev.stop_propagation();
                        menu_rename_value.set(name_for_rename.clone());
                        menu_rename_mode.set(Some(name_for_rename.clone()));
                        menu_open_for.set(None);
                    };
                    let do_duplicate = move |ev: leptos::ev::MouseEvent| {
                        ev.stop_propagation();
                        layout.duplicate_layout(&name_for_dup);
                        menu_open_for.set(None);
                    };
                    let do_export = move |ev: leptos::ev::MouseEvent| {
                        ev.stop_propagation();
                        let current_name = layout.current_layout_name.get_untracked();
                        if current_name != name_for_export {
                            layout.switch_to_layout(&name_for_export);
                        }
                        let json = layout.export_json();
                        let blob_parts = js_sys::Array::new();
                        blob_parts.push(&wasm_bindgen::JsValue::from_str(&json));
                        let opts = web_sys::BlobPropertyBag::new();
                        opts.set_type("application/json");
                        if let Ok(blob) = web_sys::Blob::new_with_str_sequence_and_options(&blob_parts, &opts) {
                            if let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob) {
                                let window = web_sys::window().unwrap();
                                let document = window.document().unwrap();
                                let a = document.create_element("a").unwrap();
                                a.set_attribute("href", &url).ok();
                                a.set_attribute("download", &format!("{}.json", name_for_export)).ok();
                                let a: web_sys::HtmlElement = a.dyn_into().unwrap();
                                a.click();
                                web_sys::Url::revoke_object_url(&url).ok();
                            }
                        }
                        menu_open_for.set(None);
                    };
                    let do_delete = move |ev: leptos::ev::MouseEvent| {
                        ev.stop_propagation();
                        layout.delete_layout(&name_for_del);
                        menu_open_for.set(None);
                    };

                    view! {
                        <div class="layout-menu-backdrop" on:click=close_menu>
                            <div class="layout-item-menu" style=style on:click=move |ev: leptos::ev::MouseEvent| { ev.stop_propagation(); }>
                                <button class="layout-menu-btn" on:click=do_rename>{"Rename"}</button>
                                <button class="layout-menu-btn" on:click=do_duplicate>{"Duplicate"}</button>
                                <button class="layout-menu-btn" on:click=do_export>{"Export…"}</button>
                                <hr class="layout-menu-divider" />
                                <button class="layout-menu-btn layout-menu-btn-danger" on:click=do_delete>{"Delete"}</button>
                            </div>
                        </div>
                    }.into_any()
                } else if is_popover {
                    let close_popover = move |_: leptos::ev::MouseEvent| {
                        popover_open.set(false);
                    };
                    view! {
                        <div class="layout-menu-backdrop" on:click=close_popover>
                            <div class="layout-item-menu" style=style on:click=move |ev: leptos::ev::MouseEvent| { ev.stop_propagation(); }>
                                <div class="layout-menu-header">{"This layout has unsaved changes"}</div>
                                <button class="layout-menu-btn" on:click=on_save>{"Save changes"}</button>
                                <button class="layout-menu-btn" on:click=on_revert>{"Revert"}</button>
                                <hr class="layout-menu-divider" />
                                <button class="layout-menu-btn" on:click=on_rename_start>{"Rename"}</button>
                                <button class="layout-menu-btn" on:click=on_export>{"Export…"}</button>
                                <hr class="layout-menu-divider" />
                                <button class="layout-menu-btn layout-menu-btn-danger" on:click=on_delete>{"Delete"}</button>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}
        </div>
    }
}

/// Import a layout from JSON string (supports both internal and Lichtblick format).
pub fn import_layout_json(json: &str, layout: &LayoutState) {
    // Try internal format first (has "type":"panel" or "type":"split")
    if json.contains(r#""type":"panel"#) || json.contains(r#""type":"split"#) {
        // Extract the layout field if wrapped, or use directly
        let layout_json = if let Some(inner) = extract_json_field(json, "layout") {
            inner
        } else {
            json
        };
        if let Some(tree) = parse_layout_node_internal(layout_json, &mut 1) {
            let next_id = count_nodes(&tree) as u32 + 1;
            layout.tree.set(tree);
            layout.next_id.set(next_id);
            // Extract name if present, or generate one
            let name = if let Some(n) = extract_string_value(json, "name") {
                n.to_string()
            } else {
                generate_unique_import_name(layout)
            };
            layout.current_layout_name.set(name);
            layout.save_current();
            return;
        }
    }

    // Fallback: Lichtblick format (has "layout": "PanelType!id" or split objects)
    if let Some(layout_value) = extract_json_field(json, "layout") {
        if let Some(tree) = parse_layout_node(layout_value, &mut 1) {
            let next_id = count_nodes(&tree) as u32 + 1;
            layout.tree.set(tree);
            layout.next_id.set(next_id);
            // Generate a name for the imported layout
            let name = generate_unique_import_name(layout);
            layout.current_layout_name.set(name);
            layout.save_current();
        }
    }
}

/// Generate a unique name for an imported layout (e.g. "Imported", "Imported 2", etc.)
fn generate_unique_import_name(layout: &LayoutState) -> String {
    let names = layout.saved_layout_names.get_untracked();
    let base = "Imported";
    if !names.contains(&base.to_string()) {
        return base.to_string();
    }
    let mut i = 2;
    loop {
        let candidate = format!("{} {}", base, i);
        if !names.contains(&candidate) {
            return candidate;
        }
        i += 1;
    }
}

fn extract_string_value<'a>(json: &'a str, field: &str) -> Option<&'a str> {
    let pattern = format!(r#""{}":""#, field);
    let start = json.find(&pattern)? + pattern.len();
    let remaining = &json[start..];
    let end = remaining.find('"')?;
    Some(&remaining[..end])
}

fn count_nodes(node: &LayoutNode) -> usize {
    match node {
        LayoutNode::Panel { .. } => 1,
        LayoutNode::Split { first, second, .. } => 1 + count_nodes(first) + count_nodes(second),
    }
}

/// Very simple JSON field extractor (finds "field": value at top level).
fn extract_json_field<'a>(json: &'a str, field: &str) -> Option<&'a str> {
    let pattern = format!(r#""{}":"#, field);
    let start = json.find(&pattern)? + pattern.len();
    let remaining = &json[start..];
    // Find the balanced end
    if remaining.starts_with('"') {
        // String value
        let end = remaining[1..].find('"')? + 2;
        Some(&remaining[..end])
    } else if remaining.starts_with('{') {
        // Object value - find matching brace
        let mut depth = 0;
        for (i, c) in remaining.chars().enumerate() {
            match c {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(&remaining[..=i]);
                    }
                }
                _ => {}
            }
        }
        None
    } else {
        // Primitive
        let end = remaining.find(&[',', '}'][..]).unwrap_or(remaining.len());
        Some(&remaining[..end])
    }
}

/// Parse a layout node from JSON (simplified).
fn parse_layout_node(json: &str, next_id: &mut u32) -> Option<LayoutNode> {
    let json = json.trim();
    if json.starts_with('"') {
        // Leaf panel: "PanelType!id"
        let panel_str = json.trim_matches('"');
        let panel_type = if panel_str.starts_with("Image") {
            PanelType::Image
        } else if panel_str.starts_with("3D") {
            PanelType::ThreeDee
        } else if panel_str.starts_with("RawMessages") {
            PanelType::RawMessages
        } else if panel_str.starts_with("RosOut") || panel_str.starts_with("Log") {
            PanelType::Log
        } else if panel_str.starts_with("Plot") {
            PanelType::Plot
        } else if panel_str.starts_with("DataSourceInfo") {
            PanelType::DataSourceInfo
        } else {
            PanelType::RawMessages
        };
        let id = *next_id;
        *next_id += 1;
        Some(LayoutNode::Panel { id, panel_type, topic: None })
    } else if json.starts_with('{') {
        // Split node
        let first_json = extract_json_field(json, "first")?;
        let second_json = extract_json_field(json, "second")?;
        let direction = if json.contains(r#""direction":"row"#) {
            SplitDirection::Horizontal
        } else {
            SplitDirection::Vertical
        };
        // Extract splitPercentage
        let ratio = extract_json_field(json, "splitPercentage")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(50.0);

        let first = parse_layout_node(first_json, next_id)?;
        let second = parse_layout_node(second_json, next_id)?;
        let id = *next_id;
        *next_id += 1;
        Some(LayoutNode::Split {
            id,
            direction,
            ratio,
            first: Box::new(first),
            second: Box::new(second),
        })
    } else {
        None
    }
}

fn find_panel_in_tree(node: &LayoutNode, target_id: NodeId) -> Option<LayoutNode> {
    match node {
        LayoutNode::Panel { id, .. } => {
            if *id == target_id { Some(node.clone()) } else { None }
        }
        LayoutNode::Split { first, second, .. } => {
            find_panel_in_tree(first, target_id)
                .or_else(|| find_panel_in_tree(second, target_id))
        }
    }
}

/// Panel settings view - shows panel-specific configuration.
#[component]
fn PanelSettingsView(node_id: NodeId, node: LayoutNode) -> impl IntoView {
    let layout = use_layout_state();
    let (panel_type, current_topic) = match &node {
        LayoutNode::Panel { panel_type, topic, .. } => (panel_type.clone(), topic.clone()),
        _ => return view! { <div></div> }.into_any(),
    };

    let panel_name = panel_type.display_name().to_string();
    let panel_menu_open = RwSignal::new(false);

    let on_close = move |_: leptos::ev::MouseEvent| {
        layout.active_settings_panel.set(None);
    };

    let on_panel_menu = move |_: leptos::ev::MouseEvent| {
        panel_menu_open.update(|v| *v = !*v);
    };

    let on_reset = {
        let pt = panel_type.clone();
        move |_: leptos::ev::MouseEvent| {
            match pt {
                PanelType::ThreeDee => layout.reset_three_dee_config(node_id),
                PanelType::Image => layout.reset_image_config(node_id),
                _ => {}
            }
            panel_menu_open.set(false);
        }
    };

    let on_import_export_header = move |_: leptos::ev::MouseEvent| {
        // TODO: implement import/export dialog
        panel_menu_open.set(false);
        web_sys::console::log_1(&"Import/export settings not yet implemented".into());
    };

    let is_three_dee = panel_type == PanelType::ThreeDee;

    view! {
        <div class="panel-settings">
            <div class="panel-settings-header">
                <span class="panel-settings-title">{format!("{} panel", panel_name)}</span>
                <div class="panel-settings-header-actions">
                    <div class="panel-settings-menu-wrapper">
                        <button class="panel-settings-menu-btn" on:click=on_panel_menu title="Options">{"⋮"}</button>
                        <div class="panel-settings-dropdown" class:open=move || panel_menu_open.get()>
                            {if is_three_dee {
                                Some(view! {
                                    <div class="panel-menu-item" on:mousedown=on_import_export_header>
                                        <span>{"Import/export settings..."}</span>
                                    </div>
                                })
                            } else {
                                None
                            }}
                            <div class="panel-menu-item" on:mousedown=on_reset>
                                <span>{"Reset to defaults"}</span>
                            </div>
                        </div>
                    </div>
                    <button class="panel-settings-close" on:click=on_close title="Close settings">{"✕"}</button>
                </div>
            </div>
            <div class="panel-settings-body">
                {match panel_type {
                    PanelType::Image => view! { <ImageSettings node_id=node_id topic=current_topic /> }.into_any(),
                    PanelType::ThreeDee => view! { <ThreeDeeSettings node_id=node_id /> }.into_any(),
                    PanelType::RawMessages => view! { <RawMessagesSettings node_id=node_id topic=current_topic /> }.into_any(),
                    PanelType::Log => view! { <TopicSelectSettings node_id=node_id topic=current_topic label="Topic" /> }.into_any(),
                    PanelType::Plot => view! { <TopicSelectSettings node_id=node_id topic=current_topic label="Topic" /> }.into_any(),
                    PanelType::StateTransitions => view! { <TopicSelectSettings node_id=node_id topic=current_topic label="Topic" /> }.into_any(),
                    _ => view! {
                        <div class="settings-section">
                            <p class="text-muted">{"No settings available for this panel type."}</p>
                        </div>
                    }.into_any(),
                }}
            </div>
        </div>
    }.into_any()
}

/// Image panel settings - full settings matching Lichtblick original.
#[component]
fn ImageSettings(node_id: NodeId, topic: Option<String>) -> impl IntoView {
    let layout = use_layout_state();
    let current_topic = topic.unwrap_or_default();

    // Get config signal - we track image_configs signal for reactivity
    let config = move || {
        layout.image_configs.with(|configs| {
            configs.get(&node_id).cloned().unwrap_or_default()
        })
    };

    // Get available image topics from the player
    let image_topics = move || -> Vec<String> {
        get_player().map(|p| {
            p.topics().iter()
                .filter(|t| is_compressed_image_schema(&t.schema_name))
                .map(|t| t.name.clone())
                .collect()
        }).unwrap_or_default()
    };

    // Get available calibration topics
    let calibration_topics = move || -> Vec<String> {
        get_player().map(|p| {
            p.topics().iter()
                .filter(|t| {
                    t.schema_name.contains("CameraInfo")
                        || t.schema_name.contains("CameraCalibration")
                })
                .map(|t| t.name.clone())
                .collect()
        }).unwrap_or_default()
    };

    let current_topic_for_view = current_topic.clone();

    let on_topic_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(select) = target.dyn_into::<web_sys::HtmlSelectElement>() {
                let val = select.value();
                let new_topic = if val.is_empty() { None } else { Some(val) };
                layout.set_panel_topic(node_id, new_topic);
            }
        }
    };

    let on_calibration_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(select) = target.dyn_into::<web_sys::HtmlSelectElement>() {
                let val = select.value();
                let new_cal = if val.is_empty() { None } else { Some(val) };
                layout.update_image_config(node_id, move |cfg| {
                    cfg.calibration_topic = new_cal;
                });
            }
        }
    };

    let on_sync_change = move |val: bool| {
        layout.update_image_config(node_id, move |cfg| { cfg.synchronize = val; });
    };

    let on_flip_h_change = move |val: bool| {
        layout.update_image_config(node_id, move |cfg| { cfg.flip_horizontal = val; });
    };

    let on_flip_v_change = move |val: bool| {
        layout.update_image_config(node_id, move |cfg| { cfg.flip_vertical = val; });
    };

    let on_rotation_change = move |val: u16| {
        layout.update_image_config(node_id, move |cfg| { cfg.rotation = val; });
    };

    let on_brightness_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(val) = input.value().parse::<f64>() {
                    layout.update_image_config(node_id, move |cfg| { cfg.brightness = val; });
                }
            }
        }
    };

    let on_contrast_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(val) = input.value().parse::<f64>() {
                    layout.update_image_config(node_id, move |cfg| { cfg.contrast = val; });
                }
            }
        }
    };

    // Scene settings
    let on_render_stats_change = move |val: bool| {
        layout.update_image_config(node_id, move |cfg| { cfg.scene_render_stats = val; });
    };

    let on_bg_color_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                let val = input.value();
                layout.update_image_config(node_id, move |cfg| { cfg.scene_background = val; });
            }
        }
    };

    let on_label_scale_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(val) = input.value().parse::<f64>() {
                    layout.update_image_config(node_id, move |cfg| { cfg.scene_label_scale = val; });
                }
            }
        }
    };

    let on_ignore_collada_change = move |val: bool| {
        layout.update_image_config(node_id, move |cfg| { cfg.scene_ignore_collada_up_axis = val; });
    };

    let on_mesh_up_axis_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(select) = target.dyn_into::<web_sys::HtmlSelectElement>() {
                let val = select.value();
                layout.update_image_config(node_id, move |cfg| { cfg.scene_mesh_up_axis = val; });
            }
        }
    };

    // Reset handlers
    let on_reset_general = move |_: leptos::ev::MouseEvent| {
        layout.update_image_config(node_id, |cfg| {
            cfg.calibration_topic = None;
            cfg.synchronize = false;
            cfg.flip_horizontal = false;
            cfg.flip_vertical = false;
            cfg.rotation = 0;
            cfg.brightness = 50.0;
            cfg.contrast = 50.0;
        });
    };

    let on_reset_scene = move |_: leptos::ev::MouseEvent| {
        layout.update_image_config(node_id, |cfg| {
            cfg.scene_render_stats = false;
            cfg.scene_background = "#000000".to_string();
            cfg.scene_label_scale = 1.0;
            cfg.scene_ignore_collada_up_axis = false;
            cfg.scene_mesh_up_axis = "y_up".to_string();
        });
    };

    // Collapsible state
    let general_open = RwSignal::new(true);
    let scene_open = RwSignal::new(false);
    let annotations_open = RwSignal::new(false);
    let transforms_open = RwSignal::new(false);
    let topics_open = RwSignal::new(false);
    let custom_layers_open = RwSignal::new(false);

    view! {
        // General section
        <div class="settings-section">
            <div class="settings-section-header">
                <h4 class="settings-section-title" on:click=move |_| general_open.update(|v| *v = !*v)>
                    {move || if general_open.get() { "▼" } else { "▶" }}
                    " General"
                </h4>
                <button class="settings-section-menu-btn" title="Reset" on:click=on_reset_general>{"⋮"}</button>
            </div>
            <div class="settings-section-body" class:collapsed=move || !general_open.get()>
                // Topic
                <div class="settings-row">
                    <label class="settings-label">{"Topic"}</label>
                    <select class="settings-select" on:change=on_topic_change>
                        <option value="" selected=current_topic_for_view.is_empty()>{""}</option>
                        {move || image_topics().into_iter().map(|t| {
                            let selected = t == current_topic;
                            let t_val = t.clone();
                            view! { <option value=t_val selected=selected>{t}</option> }
                        }).collect::<Vec<_>>()}
                    </select>
                </div>
                // Calibration
                <div class="settings-row">
                    <label class="settings-label">{"Calibration"}</label>
                    <select class="settings-select" on:change=on_calibration_change>
                        <option value="" selected=move || config().calibration_topic.is_none()>{"None"}</option>
                        {move || calibration_topics().into_iter().map(|t| {
                            let selected = config().calibration_topic.as_deref() == Some(t.as_str());
                            let t_val = t.clone();
                            view! { <option value=t_val selected=selected>{t}</option> }
                        }).collect::<Vec<_>>()}
                    </select>
                </div>
                // Sync annotations
                <div class="settings-row">
                    <label class="settings-label">{"Sync annotations"}</label>
                    {toggle_button_view(move || config().synchronize, on_sync_change)}
                </div>
                // Flip horizontal
                <div class="settings-row">
                    <label class="settings-label">{"Flip horizontal"}</label>
                    {toggle_button_view(move || config().flip_horizontal, on_flip_h_change)}
                </div>
                // Flip vertical
                <div class="settings-row">
                    <label class="settings-label">{"Flip vertical"}</label>
                    {toggle_button_view(move || config().flip_vertical, on_flip_v_change)}
                </div>
                // Rotation
                <div class="settings-row">
                    <label class="settings-label">{"Rotation"}</label>
                    {rotation_toggle_view(move || config().rotation, on_rotation_change)}
                </div>
                // Brightness
                <div class="settings-row">
                    <label class="settings-label">{"Brightness"}</label>
                    <input
                        type="range"
                        class="settings-slider"
                        min="0" max="100" step="5"
                        prop:value=move || config().brightness.to_string()
                        on:input=on_brightness_change
                    />
                </div>
                // Contrast
                <div class="settings-row">
                    <label class="settings-label">{"Contrast"}</label>
                    <input
                        type="range"
                        class="settings-slider"
                        min="0" max="100" step="5"
                        prop:value=move || config().contrast.to_string()
                        on:input=on_contrast_change
                    />
                </div>
            </div>
        </div>

        // Scene section
        <div class="settings-section">
            <div class="settings-section-header">
                <h4 class="settings-section-title" on:click=move |_| scene_open.update(|v| *v = !*v)>
                    {move || if scene_open.get() { "▼" } else { "▶" }}
                    " Scene"
                </h4>
                <button class="settings-section-menu-btn" title="Reset" on:click=on_reset_scene>{"⋮"}</button>
            </div>
            <div class="settings-section-body" class:collapsed=move || !scene_open.get()>
                // Render stats
                <div class="settings-row">
                    <label class="settings-label">{"Render stats"}</label>
                    {toggle_button_view(move || config().scene_render_stats, on_render_stats_change)}
                </div>
                // Background color
                <div class="settings-row">
                    <label class="settings-label">{"Background"}</label>
                    <input
                        type="color"
                        class="settings-color-input"
                        prop:value=move || config().scene_background.clone()
                        on:input=on_bg_color_change
                    />
                </div>
                // Label scale
                <div class="settings-row">
                    <label class="settings-label">{"Label scale"}</label>
                    <input
                        type="number"
                        class="settings-number-input"
                        min="0" step="0.1"
                        prop:value=move || config().scene_label_scale.to_string()
                        on:change=on_label_scale_change
                    />
                </div>
                // Ignore COLLADA up axis
                <div class="settings-row">
                    <label class="settings-label">{"Ignore COLLADA <up_axis>"}</label>
                    {toggle_button_view(move || config().scene_ignore_collada_up_axis, on_ignore_collada_change)}
                </div>
                // Mesh up axis
                <div class="settings-row">
                    <label class="settings-label">{"Mesh up axis"}</label>
                    <select class="settings-select" on:change=on_mesh_up_axis_change>
                        <option value="y_up" selected=move || config().scene_mesh_up_axis == "y_up">{"Y-up"}</option>
                        <option value="z_up" selected=move || config().scene_mesh_up_axis == "z_up">{"Z-up"}</option>
                    </select>
                </div>
            </div>
        </div>

        // Image annotations section
        <div class="settings-section">
            <div class="settings-section-header">
                <h4 class="settings-section-title" on:click=move |_| annotations_open.update(|v| *v = !*v)>
                    {move || if annotations_open.get() { "▼" } else { "▶" }}
                    " Image annotations"
                </h4>
            </div>
            <div class="settings-section-body" class:collapsed=move || !annotations_open.get()>
                <p class="text-muted settings-placeholder">{"No annotation topics available"}</p>
            </div>
        </div>

        // Transforms section
        <div class="settings-section">
            <div class="settings-section-header">
                <h4 class="settings-section-title" on:click=move |_| transforms_open.update(|v| *v = !*v)>
                    {move || if transforms_open.get() { "▼" } else { "▶" }}
                    " Transforms"
                </h4>
                <button class="settings-section-menu-btn" title="Options">{"⋮"}</button>
            </div>
            <div class="settings-section-body" class:collapsed=move || !transforms_open.get()>
                <p class="text-muted settings-placeholder">{"No transforms available"}</p>
            </div>
        </div>

        // Topics section
        <div class="settings-section">
            <div class="settings-section-header">
                <h4 class="settings-section-title" on:click=move |_| topics_open.update(|v| *v = !*v)>
                    {move || if topics_open.get() { "▼" } else { "▶" }}
                    " Topics"
                </h4>
                <button class="settings-section-menu-btn" title="Options">{"⋮"}</button>
            </div>
            <div class="settings-section-body" class:collapsed=move || !topics_open.get()>
                <p class="text-muted settings-placeholder">{"No topic layers configured"}</p>
            </div>
        </div>

        // Custom layers section
        <div class="settings-section">
            <div class="settings-section-header">
                <h4 class="settings-section-title" on:click=move |_| custom_layers_open.update(|v| *v = !*v)>
                    {move || if custom_layers_open.get() { "▼" } else { "▶" }}
                    " Custom layers"
                </h4>
                <button class="settings-section-menu-btn" title="Options">{"⋮"}</button>
            </div>
            <div class="settings-section-body" class:collapsed=move || !custom_layers_open.get()>
                <p class="text-muted settings-placeholder">{"No custom layers"}</p>
            </div>
        </div>
    }
}

/// Off/On toggle button widget - inline helper returns view fragment.
fn toggle_button_view(
    value: impl Fn() -> bool + 'static + Copy + Send + Sync,
    on_change: impl Fn(bool) + 'static + Copy + Send + Sync,
) -> impl IntoView {
    view! {
        <div class="toggle-group">
            <button
                class="toggle-btn"
                class:active=move || !value()
                on:click=move |_| on_change(false)
            >{"Off"}</button>
            <button
                class="toggle-btn"
                class:active=move || value()
                on:click=move |_| on_change(true)
            >{"On"}</button>
        </div>
    }
}

/// Rotation toggle with 4 options - inline helper.
fn rotation_toggle_view(
    value: impl Fn() -> u16 + 'static + Copy + Send + Sync,
    on_change: impl Fn(u16) + 'static + Copy + Send + Sync,
) -> impl IntoView {
    view! {
        <div class="toggle-group rotation-toggle">
            <button class="toggle-btn" class:active=move || value() == 0 on:click=move |_| on_change(0)>{"0°"}</button>
            <button class="toggle-btn" class:active=move || value() == 90 on:click=move |_| on_change(90)>{"90°"}</button>
            <button class="toggle-btn" class:active=move || value() == 180 on:click=move |_| on_change(180)>{"180°"}</button>
            <button class="toggle-btn" class:active=move || value() == 270 on:click=move |_| on_change(270)>{"270°"}</button>
        </div>
    }
}

/// 3D panel settings.
#[component]
fn ThreeDeeSettings(node_id: NodeId) -> impl IntoView {
    let state = use_app_state();
    let layout = use_layout_state();

    // Search filter
    let search_filter = RwSignal::new(String::new());
    // Topic filter: "all", "visible", "invisible"
    let topic_filter = RwSignal::new("all".to_string());

    // Section collapsed states
    let frame_open = RwSignal::new(true);
    let scene_open = RwSignal::new(false);
    let view_open = RwSignal::new(false);
    let transforms_open = RwSignal::new(false);
    let topics_open = RwSignal::new(false);
    let custom_layers_open = RwSignal::new(false);
    let publish_open = RwSignal::new(false);

    // Reactive config reader
    let config = move || layout.get_three_dee_config(node_id);

    let frames = move || state.tf_frames.get();
    let current_display_frame = move || state.display_frame.get();
    let current_follow_mode = move || state.follow_mode.get();

    // --- Header handlers ---
    let on_title_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                let val = input.value();
                layout.update_three_dee_config(node_id, |c| c.title = val);
            }
        }
    };

    let on_search_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                search_filter.set(input.value().to_lowercase());
            }
        }
    };

    // --- Frame handlers ---
    let on_display_frame_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(select) = target.dyn_into::<web_sys::HtmlSelectElement>() {
                let val = select.value();
                state.display_frame.set(val.clone());
                layout.update_three_dee_config(node_id, |c| c.display_frame = val);
            }
        }
    };

    let on_follow_mode_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(select) = target.dyn_into::<web_sys::HtmlSelectElement>() {
                let val = select.value();
                state.follow_mode.set(val.clone());
                layout.update_three_dee_config(node_id, |c| c.follow_mode = val);
            }
        }
    };

    // --- Scene handlers ---
    let on_bg_color_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                let val = input.value();
                layout.update_three_dee_config(node_id, |c| c.scene.background_color = val);
            }
        }
    };

    let on_label_scale_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(v) = input.value().parse::<f64>() {
                    layout.update_three_dee_config(node_id, |c| c.scene.label_scale = v);
                }
            }
        }
    };

    let on_enable_stats_change = move |val: bool| {
        layout.update_three_dee_config(node_id, move |c| c.scene.enable_stats = val);
    };

    let on_ignore_collada_change = move |val: bool| {
        layout.update_three_dee_config(node_id, move |c| c.scene.ignore_collada_up_axis = val);
    };

    let on_mesh_up_axis_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(select) = target.dyn_into::<web_sys::HtmlSelectElement>() {
                let val = select.value();
                layout.update_three_dee_config(node_id, |c| c.scene.mesh_up_axis = val);
            }
        }
    };

    // --- View handlers ---
    let on_distance_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(v) = input.value().parse::<f64>() {
                    layout.update_three_dee_config(node_id, |c| c.view.distance = v);
                }
            }
        }
    };

    let on_perspective_change = move |val: bool| {
        layout.update_three_dee_config(node_id, move |c| c.view.perspective = val);
    };

    let on_sync_camera_change = move |val: bool| {
        layout.update_three_dee_config(node_id, move |c| c.view.sync_camera = val);
    };

    let on_fovy_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(v) = input.value().parse::<f64>() {
                    layout.update_three_dee_config(node_id, |c| c.view.fovy = v);
                }
            }
        }
    };

    let on_near_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(v) = input.value().parse::<f64>() {
                    layout.update_three_dee_config(node_id, |c| c.view.near = v);
                }
            }
        }
    };

    let on_far_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(v) = input.value().parse::<f64>() {
                    layout.update_three_dee_config(node_id, |c| c.view.far = v);
                }
            }
        }
    };

    // --- Transforms handlers ---
    let on_show_labels_change = move |val: bool| {
        layout.update_three_dee_config(node_id, move |c| c.transforms.show_labels = val);
    };

    let on_axis_scale_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(v) = input.value().parse::<f64>() {
                    layout.update_three_dee_config(node_id, |c| c.transforms.axis_scale = v);
                }
            }
        }
    };

    let on_line_width_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(v) = input.value().parse::<f64>() {
                    layout.update_three_dee_config(node_id, |c| c.transforms.line_width = v);
                }
            }
        }
    };

    let on_line_color_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                let val = input.value();
                layout.update_three_dee_config(node_id, |c| c.transforms.line_color = val);
            }
        }
    };

    let on_editable_change = move |val: bool| {
        layout.update_three_dee_config(node_id, move |c| c.transforms.editable = val);
    };

    let on_preloading_change = move |val: bool| {
        layout.update_three_dee_config(node_id, move |c| c.transforms.enable_preloading = val);
    };

    // --- Publish handlers ---
    let on_publish_type_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(select) = target.dyn_into::<web_sys::HtmlSelectElement>() {
                let val = select.value();
                layout.update_three_dee_config(node_id, |c| c.publish.publish_type = val);
            }
        }
    };

    let on_publish_topic_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                let val = input.value();
                layout.update_three_dee_config(node_id, |c| c.publish.topic = val);
            }
        }
    };

    // --- Topics: list all topics from player ---
    let all_topics = move || -> Vec<String> {
        get_player().map(|p| {
            p.topics().iter().map(|t| t.name.clone()).collect()
        }).unwrap_or_default()
    };

    // Helper: returns "display:none" when search filter doesn't match section keywords
    let section_style = move |section_keywords: &'static [&'static str]| {
        move || {
            let filter = search_filter.get();
            if filter.is_empty() {
                return String::new();
            }
            let matches = section_keywords.iter().any(|kw| kw.contains(filter.as_str()));
            if matches { String::new() } else { "display:none;".to_string() }
        }
    };

    let frame_style = section_style(&["frame", "display frame", "follow mode", "follow"]);
    let scene_style = section_style(&["scene", "background", "label scale", "render stats", "collada", "mesh up axis"]);
    let view_style = section_style(&["view", "perspective", "distance", "fov", "near clip", "far clip", "sync camera"]);
    let transforms_style = section_style(&["transforms", "transform", "show labels", "axis scale", "line width", "line color", "editable", "preloading"]);
    let topics_style = section_style(&["topics", "topic", "visible", "color", "outlines", "caching", "axes", "lanes", "bounding box", "3d models"]);
    let custom_layers_style = section_style(&["custom layers", "grid", "urdf", "layers"]);
    let publish_style = section_style(&["publish", "publish type", "topic"]);

    view! {
        // === SETTINGS HEADER (search, title) ===
        <div class="settings-section" style="padding: 8px 12px;">
            // Search field
            <div style="display:flex; align-items:center; gap:6px; margin-bottom:8px;">
                <span style="font-size:14px; color:var(--text-secondary);">{"🔍"}</span>
                <input type="text" class="settings-select" style="flex:1; font-size:12px;"
                    placeholder="Search panel settings..."
                    on:input=on_search_change
                />
            </div>
            // Title field
            <div class="settings-row">
                <label class="settings-label">{"Title"}</label>
                <input type="text" class="settings-select" style="flex:1;"
                    value=move || config().title
                    on:input=on_title_change
                />
            </div>
        </div>

        // === FRAME ===
        <div class="settings-section" style=frame_style>
            <div class="settings-section-header">
                <h4 class="settings-section-title" on:click=move |_| frame_open.update(|v| *v = !*v)>
                    {move || if frame_open.get() { "▼ Frame" } else { "▶ Frame" }}
                </h4>
            </div>
            <div class="settings-section-body" class:collapsed=move || !frame_open.get()>
                <div class="settings-row">
                    <label class="settings-label">{"Display frame"}</label>
                    <select class="settings-select" on:change=on_display_frame_change>
                        {move || {
                            let f = frames();
                            let current = current_display_frame();
                            if f.is_empty() {
                                vec![view! { <option selected=true>{"(no frames)"}</option> }.into_any()]
                            } else {
                                f.into_iter().map(|frame| {
                                    let selected = frame == current;
                                    let val = frame.clone();
                                    view! { <option value=val selected=selected>{frame}</option> }.into_any()
                                }).collect::<Vec<_>>()
                            }
                        }}
                    </select>
                </div>
                <div class="settings-row">
                    <label class="settings-label">{"Follow mode"}</label>
                    <select class="settings-select" on:change=on_follow_mode_change>
                        <option value="pose" selected=move || current_follow_mode() == "pose">{"Pose"}</option>
                        <option value="position" selected=move || current_follow_mode() == "position">{"Position"}</option>
                        <option value="fixed" selected=move || current_follow_mode() == "fixed">{"Fixed"}</option>
                    </select>
                </div>
            </div>
        </div>

        // === SCENE ===
        <div class="settings-section" style=scene_style>
            <div class="settings-section-header">
                <h4 class="settings-section-title" on:click=move |_| scene_open.update(|v| *v = !*v)>
                    {move || if scene_open.get() { "▼ Scene" } else { "▶ Scene" }}
                </h4>
            </div>
            <div class="settings-section-body" class:collapsed=move || !scene_open.get()>
                <div class="settings-row">
                    <label class="settings-label">{"Background"}</label>
                    <input type="color" class="settings-color-input"
                        prop:value=move || config().scene.background_color
                        on:input=on_bg_color_change
                    />
                </div>
                <div class="settings-row">
                    <label class="settings-label">{"Label scale"}</label>
                    <input type="number" class="settings-number-input"
                        step="0.1" min="0.1" max="5.0"
                        value=move || format!("{:.1}", config().scene.label_scale)
                        on:change=on_label_scale_change
                    />
                </div>
                <div class="settings-row">
                    <label class="settings-label">{"Render stats"}</label>
                    <div class="toggle-group">
                        <button class="toggle-btn" class:active=move || !config().scene.enable_stats
                            on:click=move |_| on_enable_stats_change(false)>{"Off"}</button>
                        <button class="toggle-btn" class:active=move || config().scene.enable_stats
                            on:click=move |_| on_enable_stats_change(true)>{"On"}</button>
                    </div>
                </div>
                <div class="settings-row">
                    <label class="settings-label">{"Ignore COLLADA up"}</label>
                    <div class="toggle-group">
                        <button class="toggle-btn" class:active=move || !config().scene.ignore_collada_up_axis
                            on:click=move |_| on_ignore_collada_change(false)>{"Off"}</button>
                        <button class="toggle-btn" class:active=move || config().scene.ignore_collada_up_axis
                            on:click=move |_| on_ignore_collada_change(true)>{"On"}</button>
                    </div>
                </div>
                <div class="settings-row">
                    <label class="settings-label">{"Mesh up axis"}</label>
                    <select class="settings-select" on:change=on_mesh_up_axis_change>
                        <option value="y_up" selected=move || config().scene.mesh_up_axis == "y_up">{"Y-up"}</option>
                        <option value="z_up" selected=move || config().scene.mesh_up_axis == "z_up">{"Z-up"}</option>
                    </select>
                </div>
            </div>
        </div>

        // === VIEW ===
        <div class="settings-section" style=view_style>
            <div class="settings-section-header">
                <h4 class="settings-section-title" on:click=move |_| view_open.update(|v| *v = !*v)>
                    {move || if view_open.get() { "▼ View" } else { "▶ View" }}
                </h4>
            </div>
            <div class="settings-section-body" class:collapsed=move || !view_open.get()>
                <div class="settings-row">
                    <label class="settings-label">{"Perspective"}</label>
                    <div class="toggle-group">
                        <button class="toggle-btn" class:active=move || !config().view.perspective
                            on:click=move |_| on_perspective_change(false)>{"Ortho"}</button>
                        <button class="toggle-btn" class:active=move || config().view.perspective
                            on:click=move |_| on_perspective_change(true)>{"Persp"}</button>
                    </div>
                </div>
                <div class="settings-row">
                    <label class="settings-label">{"Sync camera"}</label>
                    <div class="toggle-group">
                        <button class="toggle-btn" class:active=move || !config().view.sync_camera
                            on:click=move |_| on_sync_camera_change(false)>{"Off"}</button>
                        <button class="toggle-btn" class:active=move || config().view.sync_camera
                            on:click=move |_| on_sync_camera_change(true)>{"On"}</button>
                    </div>
                </div>
                <div class="settings-row">
                    <label class="settings-label">{"Distance"}</label>
                    <input type="number" class="settings-number-input"
                        step="1" min="1"
                        value=move || format!("{:.0}", config().view.distance)
                        on:change=on_distance_change
                    />
                </div>
                <div class="settings-row">
                    <label class="settings-label">{"FOV (°)"}</label>
                    <input type="number" class="settings-number-input"
                        step="1" min="10" max="120"
                        value=move || format!("{:.0}", config().view.fovy)
                        on:change=on_fovy_change
                    />
                </div>
                <div class="settings-row">
                    <label class="settings-label">{"Near clip"}</label>
                    <input type="number" class="settings-number-input"
                        step="0.1" min="0.01"
                        value=move || format!("{:.2}", config().view.near)
                        on:change=on_near_change
                    />
                </div>
                <div class="settings-row">
                    <label class="settings-label">{"Far clip"}</label>
                    <input type="number" class="settings-number-input"
                        step="100" min="100"
                        value=move || format!("{:.0}", config().view.far)
                        on:change=on_far_change
                    />
                </div>
            </div>
        </div>

        // === TRANSFORMS ===
        <div class="settings-section" style=transforms_style>
            <div class="settings-section-header">
                <h4 class="settings-section-title" on:click=move |_| transforms_open.update(|v| *v = !*v)>
                    {move || if transforms_open.get() { "▼ Transforms" } else { "▶ Transforms" }}
                </h4>
            </div>
            <div class="settings-section-body" class:collapsed=move || !transforms_open.get()>
                <div class="settings-row">
                    <label class="settings-label">{"Show labels"}</label>
                    <div class="toggle-group">
                        <button class="toggle-btn" class:active=move || !config().transforms.show_labels
                            on:click=move |_| on_show_labels_change(false)>{"Off"}</button>
                        <button class="toggle-btn" class:active=move || config().transforms.show_labels
                            on:click=move |_| on_show_labels_change(true)>{"On"}</button>
                    </div>
                </div>
                <div class="settings-row">
                    <label class="settings-label">{"Axis scale"}</label>
                    <input type="number" class="settings-number-input"
                        step="0.1" min="0.1" max="10.0"
                        value=move || format!("{:.1}", config().transforms.axis_scale)
                        on:change=on_axis_scale_change
                    />
                </div>
                <div class="settings-row">
                    <label class="settings-label">{"Line width"}</label>
                    <input type="number" class="settings-number-input"
                        step="0.5" min="0.5" max="10.0"
                        value=move || format!("{:.1}", config().transforms.line_width)
                        on:change=on_line_width_change
                    />
                </div>
                <div class="settings-row">
                    <label class="settings-label">{"Line color"}</label>
                    <input type="color" class="settings-color-input"
                        prop:value=move || config().transforms.line_color
                        on:input=on_line_color_change
                    />
                </div>
                <div class="settings-row">
                    <label class="settings-label">{"Editable"}</label>
                    <div class="toggle-group">
                        <button class="toggle-btn" class:active=move || !config().transforms.editable
                            on:click=move |_| on_editable_change(false)>{"Off"}</button>
                        <button class="toggle-btn" class:active=move || config().transforms.editable
                            on:click=move |_| on_editable_change(true)>{"On"}</button>
                    </div>
                </div>
                <div class="settings-row">
                    <label class="settings-label">{"Preloading"}</label>
                    <div class="toggle-group">
                        <button class="toggle-btn" class:active=move || !config().transforms.enable_preloading
                            on:click=move |_| on_preloading_change(false)>{"Off"}</button>
                        <button class="toggle-btn" class:active=move || config().transforms.enable_preloading
                            on:click=move |_| on_preloading_change(true)>{"On"}</button>
                    </div>
                </div>
                // Frame list: always visible with metadata; offsets only when editable
                {move || {
                    let _tick = state.frame_tick.get(); // re-render on tick for age updates
                    let cfg_val = config();
                    let is_editable = cfg_val.transforms.editable;
                    let tf_list = frames();
                    let current_time = get_player().map(|p| p.current_time_ns()).unwrap_or(0);

                    tf_list.into_iter().map(|frame_name| {
                        // Get metadata (parent, history, age)
                        let meta = get_tf_frame_metadata(&frame_name, current_time);
                        let (parent_str, history_size, age_str) = meta.unwrap_or_else(|| ("(root)".to_string(), 0, "—".to_string()));

                        let offset = cfg_val.transforms.offsets.get(&frame_name).cloned()
                            .unwrap_or_default();
                        let fn1 = frame_name.clone();
                        let fn2 = frame_name.clone();
                        let fn3 = frame_name.clone();
                        let fn4 = frame_name.clone();
                        let fn5 = frame_name.clone();
                        let fn6 = frame_name.clone();
                        let tx = offset.translation[0];
                        let ty = offset.translation[1];
                        let tz = offset.translation[2];
                        let rx = offset.rotation[0].to_degrees();
                        let ry = offset.rotation[1].to_degrees();
                        let rz = offset.rotation[2].to_degrees();
                        view! {
                            <div style="padding: 4px 0; border-top: 1px solid var(--border-color);">
                                <div style="font-size:11px; color:var(--text-primary); margin-bottom:2px; font-weight:500;">
                                    {frame_name.clone()}
                                </div>
                                <div style="font-size:10px; color:var(--text-secondary); margin-bottom:4px;">
                                    {format!("Parent: {} | History: {} | Age: {}", parent_str, history_size, age_str)}
                                </div>
                                {if is_editable {
                                    Some(view! {
                                        <div class="settings-row">
                                            <label class="settings-label" style="min-width:20px;">{"X"}</label>
                                            <input type="number" class="settings-number-input" step="0.1"
                                                value=format!("{:.2}", tx)
                                                on:change={
                                                    let f = fn1.clone();
                                                    move |ev: leptos::ev::Event| {
                                                        if let Some(t) = ev.target() {
                                                            if let Ok(inp) = t.dyn_into::<web_sys::HtmlInputElement>() {
                                                                if let Ok(v) = inp.value().parse::<f64>() {
                                                                    let f = f.clone();
                                                                    layout.update_three_dee_config(node_id, move |c| {
                                                                        let o = c.transforms.offsets.entry(f).or_default();
                                                                        o.translation[0] = v;
                                                                    });
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            />
                                            <label class="settings-label" style="min-width:20px;">{"Y"}</label>
                                            <input type="number" class="settings-number-input" step="0.1"
                                                value=format!("{:.2}", ty)
                                                on:change={
                                                    let f = fn2.clone();
                                                    move |ev: leptos::ev::Event| {
                                                        if let Some(t) = ev.target() {
                                                            if let Ok(inp) = t.dyn_into::<web_sys::HtmlInputElement>() {
                                                                if let Ok(v) = inp.value().parse::<f64>() {
                                                                    let f = f.clone();
                                                                    layout.update_three_dee_config(node_id, move |c| {
                                                                        let o = c.transforms.offsets.entry(f).or_default();
                                                                        o.translation[1] = v;
                                                                    });
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            />
                                            <label class="settings-label" style="min-width:20px;">{"Z"}</label>
                                            <input type="number" class="settings-number-input" step="0.1"
                                                value=format!("{:.2}", tz)
                                                on:change={
                                                    let f = fn3.clone();
                                                    move |ev: leptos::ev::Event| {
                                                        if let Some(t) = ev.target() {
                                                            if let Ok(inp) = t.dyn_into::<web_sys::HtmlInputElement>() {
                                                                if let Ok(v) = inp.value().parse::<f64>() {
                                                                    let f = f.clone();
                                                                    layout.update_three_dee_config(node_id, move |c| {
                                                                        let o = c.transforms.offsets.entry(f).or_default();
                                                                        o.translation[2] = v;
                                                                    });
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            />
                                        </div>
                                        <div class="settings-row">
                                            <label class="settings-label" style="min-width:20px;">{"R"}</label>
                                            <input type="number" class="settings-number-input" step="1"
                                                value=format!("{:.1}", rx)
                                                on:change={
                                                    let f = fn4.clone();
                                                    move |ev: leptos::ev::Event| {
                                                        if let Some(t) = ev.target() {
                                                            if let Ok(inp) = t.dyn_into::<web_sys::HtmlInputElement>() {
                                                                if let Ok(v) = inp.value().parse::<f64>() {
                                                                    let f = f.clone();
                                                                    layout.update_three_dee_config(node_id, move |c| {
                                                                        let o = c.transforms.offsets.entry(f).or_default();
                                                                        o.rotation[0] = v.to_radians();
                                                                    });
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            />
                                            <label class="settings-label" style="min-width:20px;">{"P"}</label>
                                            <input type="number" class="settings-number-input" step="1"
                                                value=format!("{:.1}", ry)
                                                on:change={
                                                    let f = fn5.clone();
                                                    move |ev: leptos::ev::Event| {
                                                        if let Some(t) = ev.target() {
                                                            if let Ok(inp) = t.dyn_into::<web_sys::HtmlInputElement>() {
                                                                if let Ok(v) = inp.value().parse::<f64>() {
                                                                    let f = f.clone();
                                                                    layout.update_three_dee_config(node_id, move |c| {
                                                                        let o = c.transforms.offsets.entry(f).or_default();
                                                                        o.rotation[1] = v.to_radians();
                                                                    });
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            />
                                            <label class="settings-label" style="min-width:20px;">{"Y"}</label>
                                            <input type="number" class="settings-number-input" step="1"
                                                value=format!("{:.1}", rz)
                                                on:change={
                                                    let f = fn6.clone();
                                                    move |ev: leptos::ev::Event| {
                                                        if let Some(t) = ev.target() {
                                                            if let Ok(inp) = t.dyn_into::<web_sys::HtmlInputElement>() {
                                                                if let Ok(v) = inp.value().parse::<f64>() {
                                                                    let f = f.clone();
                                                                    layout.update_three_dee_config(node_id, move |c| {
                                                                        let o = c.transforms.offsets.entry(f).or_default();
                                                                        o.rotation[2] = v.to_radians();
                                                                    });
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            />
                                        </div>
                                    })
                                } else {
                                    None
                                }}
                            </div>
                        }.into_any()
                    }).collect::<Vec<_>>()
                }}
            </div>
        </div>

        // === TOPICS ===
        <div class="settings-section" style=topics_style>
            <div class="settings-section-header">
                <h4 class="settings-section-title" on:click=move |_| topics_open.update(|v| *v = !*v)>
                    {move || if topics_open.get() { "▼ Topics" } else { "▶ Topics" }}
                </h4>
            </div>
            <div class="settings-section-body" class:collapsed=move || !topics_open.get()>
                // Topic filter dropdown
                <div class="settings-row" style="margin-bottom:6px;">
                    <label class="settings-label">{"Filter"}</label>
                    <select class="settings-select"
                        on:change=move |ev: leptos::ev::Event| {
                            if let Some(target) = ev.target() {
                                if let Ok(select) = target.dyn_into::<web_sys::HtmlSelectElement>() {
                                    topic_filter.set(select.value());
                                }
                            }
                        }
                    >
                        <option value="all" selected=move || topic_filter.get() == "all">{"List all"}</option>
                        <option value="visible" selected=move || topic_filter.get() == "visible">{"List visible"}</option>
                        <option value="invisible" selected=move || topic_filter.get() == "invisible">{"List invisible"}</option>
                    </select>
                </div>
                {move || {
                    let topics = all_topics();
                    let cfg = config();
                    let filter = topic_filter.get();
                    if topics.is_empty() {
                        vec![view! { <div class="settings-placeholder">{"No topics available"}</div> }.into_any()]
                    } else {
                        topics.into_iter().filter(|topic_name| {
                            let topic_cfg = cfg.topics.get(topic_name).cloned().unwrap_or_default();
                            match filter.as_str() {
                                "visible" => topic_cfg.visible,
                                "invisible" => !topic_cfg.visible,
                                _ => true,
                            }
                        }).map(|topic_name| {
                            let topic_cfg = cfg.topics.get(&topic_name).cloned().unwrap_or_default();
                            let is_visible = topic_cfg.visible;
                            let show_axes = topic_cfg.show_axes;
                            let show_outlines = topic_cfg.show_outlines;
                            let caching = topic_cfg.caching;
                            let show_phys = topic_cfg.show_physical_lanes;
                            let show_log = topic_cfg.show_logical_lanes;
                            let show_ref = topic_cfg.show_reference_lines;
                            let show_bb = topic_cfg.show_bounding_box;
                            let show_3d = topic_cfg.show_3d_models;
                            let model_path = topic_cfg.default_model_path.clone();
                            let topic_color = topic_cfg.color.clone().unwrap_or_default();
                            // Clone names for closures
                            let tn_vis_off = topic_name.clone();
                            let tn_vis_on = topic_name.clone();
                            let tn_color = topic_name.clone();
                            let tn_outlines = topic_name.clone();
                            let tn_caching = topic_name.clone();
                            let tn_axes = topic_name.clone();
                            let tn_phys = topic_name.clone();
                            let tn_log = topic_name.clone();
                            let tn_ref = topic_name.clone();
                            let tn_bb = topic_name.clone();
                            let tn_3d = topic_name.clone();
                            let tn_model = topic_name.clone();
                            view! {
                                <div style="padding: 6px 0; border-top: 1px solid var(--border-color);">
                                    <div class="settings-row">
                                        <label class="settings-label" style="text-align:left; min-width:0; flex:1; overflow:hidden; text-overflow:ellipsis; font-weight:500;" title=topic_name.clone()>
                                            {topic_name.clone()}
                                        </label>
                                        <div class="toggle-group">
                                            <button class="toggle-btn" class:active=!is_visible
                                                on:click={
                                                    let t = tn_vis_off.clone();
                                                    move |_| { let t = t.clone(); layout.update_three_dee_config(node_id, move |c| { c.topics.entry(t).or_insert_with(TopicDisplayConfig::default).visible = false; }); }
                                                }>{"Off"}</button>
                                            <button class="toggle-btn" class:active=is_visible
                                                on:click={
                                                    let t = tn_vis_on.clone();
                                                    move |_| { let t = t.clone(); layout.update_three_dee_config(node_id, move |c| { c.topics.entry(t).or_insert_with(TopicDisplayConfig::default).visible = true; }); }
                                                }>{"On"}</button>
                                        </div>
                                    </div>
                                    <div class="settings-row">
                                        <label class="settings-label">{"Color"}</label>
                                        <input type="color" class="settings-color-input"
                                            value=if topic_color.is_empty() { "#ffffff".to_string() } else { topic_color }
                                            on:input={
                                                let t = tn_color.clone();
                                                move |ev: leptos::ev::Event| {
                                                    if let Some(target) = ev.target() {
                                                        if let Ok(inp) = target.dyn_into::<web_sys::HtmlInputElement>() {
                                                            let val = inp.value();
                                                            let t = t.clone();
                                                            layout.update_three_dee_config(node_id, move |c| {
                                                                c.topics.entry(t).or_insert_with(TopicDisplayConfig::default).color = Some(val);
                                                            });
                                                        }
                                                    }
                                                }
                                            }
                                        />
                                    </div>
                                    <div class="settings-row">
                                        <label class="settings-label">{"Outlines"}</label>
                                        <div class="toggle-group">
                                            <button class="toggle-btn" class:active=!show_outlines on:click={
                                                let t = tn_outlines.clone();
                                                move |_| { let t = t.clone(); layout.update_three_dee_config(node_id, move |c| { c.topics.entry(t).or_insert_with(TopicDisplayConfig::default).show_outlines = false; }); }
                                            }>{"Off"}</button>
                                            <button class="toggle-btn" class:active=show_outlines on:click={
                                                let t = tn_outlines.clone();
                                                move |_| { let t = t.clone(); layout.update_three_dee_config(node_id, move |c| { c.topics.entry(t).or_insert_with(TopicDisplayConfig::default).show_outlines = true; }); }
                                            }>{"On"}</button>
                                        </div>
                                    </div>
                                    <div class="settings-row">
                                        <label class="settings-label">{"Caching"}</label>
                                        <div class="toggle-group">
                                            <button class="toggle-btn" class:active=!caching on:click={
                                                let t = tn_caching.clone();
                                                move |_| { let t = t.clone(); layout.update_three_dee_config(node_id, move |c| { c.topics.entry(t).or_insert_with(TopicDisplayConfig::default).caching = false; }); }
                                            }>{"Off"}</button>
                                            <button class="toggle-btn" class:active=caching on:click={
                                                let t = tn_caching.clone();
                                                move |_| { let t = t.clone(); layout.update_three_dee_config(node_id, move |c| { c.topics.entry(t).or_insert_with(TopicDisplayConfig::default).caching = true; }); }
                                            }>{"On"}</button>
                                        </div>
                                    </div>
                                    <div class="settings-row">
                                        <label class="settings-label">{"Axes"}</label>
                                        <div class="toggle-group">
                                            <button class="toggle-btn" class:active=!show_axes on:click={
                                                let t = tn_axes.clone();
                                                move |_| { let t = t.clone(); layout.update_three_dee_config(node_id, move |c| { c.topics.entry(t).or_insert_with(TopicDisplayConfig::default).show_axes = false; }); }
                                            }>{"Off"}</button>
                                            <button class="toggle-btn" class:active=show_axes on:click={
                                                let t = tn_axes.clone();
                                                move |_| { let t = t.clone(); layout.update_three_dee_config(node_id, move |c| { c.topics.entry(t).or_insert_with(TopicDisplayConfig::default).show_axes = true; }); }
                                            }>{"On"}</button>
                                        </div>
                                    </div>
                                    <div class="settings-row">
                                        <label class="settings-label">{"Phys. lanes"}</label>
                                        <div class="toggle-group">
                                            <button class="toggle-btn" class:active=!show_phys on:click={
                                                let t = tn_phys.clone();
                                                move |_| { let t = t.clone(); layout.update_three_dee_config(node_id, move |c| { c.topics.entry(t).or_insert_with(TopicDisplayConfig::default).show_physical_lanes = false; }); }
                                            }>{"Off"}</button>
                                            <button class="toggle-btn" class:active=show_phys on:click={
                                                let t = tn_phys.clone();
                                                move |_| { let t = t.clone(); layout.update_three_dee_config(node_id, move |c| { c.topics.entry(t).or_insert_with(TopicDisplayConfig::default).show_physical_lanes = true; }); }
                                            }>{"On"}</button>
                                        </div>
                                    </div>
                                    <div class="settings-row">
                                        <label class="settings-label">{"Log. lanes"}</label>
                                        <div class="toggle-group">
                                            <button class="toggle-btn" class:active=!show_log on:click={
                                                let t = tn_log.clone();
                                                move |_| { let t = t.clone(); layout.update_three_dee_config(node_id, move |c| { c.topics.entry(t).or_insert_with(TopicDisplayConfig::default).show_logical_lanes = false; }); }
                                            }>{"Off"}</button>
                                            <button class="toggle-btn" class:active=show_log on:click={
                                                let t = tn_log.clone();
                                                move |_| { let t = t.clone(); layout.update_three_dee_config(node_id, move |c| { c.topics.entry(t).or_insert_with(TopicDisplayConfig::default).show_logical_lanes = true; }); }
                                            }>{"On"}</button>
                                        </div>
                                    </div>
                                    <div class="settings-row">
                                        <label class="settings-label">{"Ref. lines"}</label>
                                        <div class="toggle-group">
                                            <button class="toggle-btn" class:active=!show_ref on:click={
                                                let t = tn_ref.clone();
                                                move |_| { let t = t.clone(); layout.update_three_dee_config(node_id, move |c| { c.topics.entry(t).or_insert_with(TopicDisplayConfig::default).show_reference_lines = false; }); }
                                            }>{"Off"}</button>
                                            <button class="toggle-btn" class:active=show_ref on:click={
                                                let t = tn_ref.clone();
                                                move |_| { let t = t.clone(); layout.update_three_dee_config(node_id, move |c| { c.topics.entry(t).or_insert_with(TopicDisplayConfig::default).show_reference_lines = true; }); }
                                            }>{"On"}</button>
                                        </div>
                                    </div>
                                    <div class="settings-row">
                                        <label class="settings-label">{"Bounding box"}</label>
                                        <div class="toggle-group">
                                            <button class="toggle-btn" class:active=!show_bb on:click={
                                                let t = tn_bb.clone();
                                                move |_| { let t = t.clone(); layout.update_three_dee_config(node_id, move |c| { c.topics.entry(t).or_insert_with(TopicDisplayConfig::default).show_bounding_box = false; }); }
                                            }>{"Off"}</button>
                                            <button class="toggle-btn" class:active=show_bb on:click={
                                                let t = tn_bb.clone();
                                                move |_| { let t = t.clone(); layout.update_three_dee_config(node_id, move |c| { c.topics.entry(t).or_insert_with(TopicDisplayConfig::default).show_bounding_box = true; }); }
                                            }>{"On"}</button>
                                        </div>
                                    </div>
                                    <div class="settings-row">
                                        <label class="settings-label">{"3D models"}</label>
                                        <div class="toggle-group">
                                            <button class="toggle-btn" class:active=!show_3d on:click={
                                                let t = tn_3d.clone();
                                                move |_| { let t = t.clone(); layout.update_three_dee_config(node_id, move |c| { c.topics.entry(t).or_insert_with(TopicDisplayConfig::default).show_3d_models = false; }); }
                                            }>{"Off"}</button>
                                            <button class="toggle-btn" class:active=show_3d on:click={
                                                let t = tn_3d.clone();
                                                move |_| { let t = t.clone(); layout.update_three_dee_config(node_id, move |c| { c.topics.entry(t).or_insert_with(TopicDisplayConfig::default).show_3d_models = true; }); }
                                            }>{"On"}</button>
                                        </div>
                                    </div>
                                    <div class="settings-row">
                                        <label class="settings-label">{"Model path"}</label>
                                        <input type="text" class="settings-select" style="font-size:11px;"
                                            value=model_path
                                            on:change={
                                                let t = tn_model.clone();
                                                move |ev: leptos::ev::Event| {
                                                    if let Some(target) = ev.target() {
                                                        if let Ok(inp) = target.dyn_into::<web_sys::HtmlInputElement>() {
                                                            let val = inp.value();
                                                            let t = t.clone();
                                                            layout.update_three_dee_config(node_id, move |c| {
                                                                c.topics.entry(t).or_insert_with(TopicDisplayConfig::default).default_model_path = val;
                                                            });
                                                        }
                                                    }
                                                }
                                            }
                                        />
                                    </div>
                                </div>
                            }.into_any()
                        }).collect::<Vec<_>>()
                    }
                }}
            </div>
        </div>

        // === CUSTOM LAYERS ===
        <div class="settings-section" style=custom_layers_style>
            <div class="settings-section-header" style="display:flex; align-items:center; justify-content:space-between;">
                <h4 class="settings-section-title" on:click=move |_| custom_layers_open.update(|v| *v = !*v)>
                    {move || if custom_layers_open.get() { "▼ Custom Layers" } else { "▶ Custom Layers" }}
                </h4>
                <div style="display:flex; gap:4px;">
                    <button class="toggle-btn" style="font-size:10px; padding:2px 6px;"
                        on:click=move |_| {
                            layout.update_three_dee_config(node_id, |c| {
                                c.custom_layers.grids.push(lichtblick_panels::three_dee::GridLayer::default());
                            });
                        }
                    >{"+ Grid"}</button>
                    <button class="toggle-btn" style="font-size:10px; padding:2px 6px;"
                        on:click=move |_| {
                            layout.update_three_dee_config(node_id, |c| {
                                c.custom_layers.urdfs.push(lichtblick_panels::three_dee::UrdfLayer {
                                    visible: true,
                                    url: String::new(),
                                    frame_id: "Global".into(),
                                });
                            });
                        }
                    >{"+ URDF"}</button>
                </div>
            </div>
            <div class="settings-section-body" class:collapsed=move || !custom_layers_open.get()>
                {move || {
                    let cfg = config();
                    let available_frames = frames();
                    let mut views: Vec<leptos::prelude::AnyView> = Vec::new();

                    // Render each grid layer
                    for (idx, grid) in cfg.custom_layers.grids.iter().enumerate() {
                        let grid_visible = grid.visible;
                        let grid_size = grid.size;
                        let grid_divs = grid.divisions;
                        let grid_color = grid.color.clone();
                        let grid_frame = grid.frame_id.clone();
                        let af = available_frames.clone();
                        views.push(view! {
                            <div style="padding:4px 0; border-top:1px solid var(--border-color);">
                                <div style="display:flex; align-items:center; justify-content:space-between; margin-bottom:4px;">
                                    <span style="font-size:11px; font-weight:500; color:var(--text-primary);">{format!("Grid #{}", idx)}</span>
                                    <button class="toggle-btn" style="font-size:10px; padding:1px 5px; color:#e55;"
                                        on:click=move |_| {
                                            layout.update_three_dee_config(node_id, move |c| {
                                                if idx < c.custom_layers.grids.len() {
                                                    c.custom_layers.grids.remove(idx);
                                                }
                                            });
                                        }
                                    >{"✕"}</button>
                                </div>
                                <div class="settings-row">
                                    <label class="settings-label">{"Visible"}</label>
                                    <div class="toggle-group">
                                        <button class="toggle-btn" class:active=!grid_visible
                                            on:click=move |_| {
                                                layout.update_three_dee_config(node_id, move |c| {
                                                    if let Some(g) = c.custom_layers.grids.get_mut(idx) { g.visible = false; }
                                                });
                                            }>{"Off"}</button>
                                        <button class="toggle-btn" class:active=grid_visible
                                            on:click=move |_| {
                                                layout.update_three_dee_config(node_id, move |c| {
                                                    if let Some(g) = c.custom_layers.grids.get_mut(idx) { g.visible = true; }
                                                });
                                            }>{"On"}</button>
                                    </div>
                                </div>
                                <div class="settings-row">
                                    <label class="settings-label">{"Frame"}</label>
                                    <select class="settings-select"
                                        on:change=move |ev: leptos::ev::Event| {
                                            if let Some(target) = ev.target() {
                                                if let Ok(select) = target.dyn_into::<web_sys::HtmlSelectElement>() {
                                                    let val = select.value();
                                                    layout.update_three_dee_config(node_id, move |c| {
                                                        if let Some(g) = c.custom_layers.grids.get_mut(idx) { g.frame_id = val; }
                                                    });
                                                }
                                            }
                                        }
                                    >
                                        {af.into_iter().map(|f| {
                                            let selected = f == grid_frame;
                                            let fv = f.clone();
                                            view! { <option value=fv selected=selected>{f}</option> }
                                        }).collect::<Vec<_>>()}
                                    </select>
                                </div>
                                <div class="settings-row">
                                    <label class="settings-label">{"Size"}</label>
                                    <input type="number" class="settings-number-input" step="1" min="1"
                                        value=format!("{:.0}", grid_size)
                                        on:change=move |ev: leptos::ev::Event| {
                                            if let Some(target) = ev.target() {
                                                if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                                                    if let Ok(v) = input.value().parse::<f64>() {
                                                        layout.update_three_dee_config(node_id, move |c| {
                                                            if let Some(g) = c.custom_layers.grids.get_mut(idx) { g.size = v; }
                                                        });
                                                    }
                                                }
                                            }
                                        }
                                    />
                                </div>
                                <div class="settings-row">
                                    <label class="settings-label">{"Divisions"}</label>
                                    <input type="number" class="settings-number-input" step="1" min="1" max="100"
                                        value=format!("{}", grid_divs)
                                        on:change=move |ev: leptos::ev::Event| {
                                            if let Some(target) = ev.target() {
                                                if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                                                    if let Ok(v) = input.value().parse::<u32>() {
                                                        layout.update_three_dee_config(node_id, move |c| {
                                                            if let Some(g) = c.custom_layers.grids.get_mut(idx) { g.divisions = v; }
                                                        });
                                                    }
                                                }
                                            }
                                        }
                                    />
                                </div>
                                <div class="settings-row">
                                    <label class="settings-label">{"Color"}</label>
                                    <input type="color" class="settings-color-input"
                                        value={
                                            let c = &grid_color;
                                            if c.len() > 7 { c[..7].to_string() } else { c.clone() }
                                        }
                                        on:input=move |ev: leptos::ev::Event| {
                                            if let Some(target) = ev.target() {
                                                if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                                                    let val = input.value();
                                                    layout.update_three_dee_config(node_id, move |c| {
                                                        if let Some(g) = c.custom_layers.grids.get_mut(idx) { g.color = val; }
                                                    });
                                                }
                                            }
                                        }
                                    />
                                </div>
                            </div>
                        }.into_any());
                    }

                    // Render each URDF layer
                    for (idx, urdf) in cfg.custom_layers.urdfs.iter().enumerate() {
                        let urdf_visible = urdf.visible;
                        let urdf_url = urdf.url.clone();
                        let urdf_frame = urdf.frame_id.clone();
                        let af = available_frames.clone();
                        views.push(view! {
                            <div style="padding:4px 0; border-top:1px solid var(--border-color);">
                                <div style="display:flex; align-items:center; justify-content:space-between; margin-bottom:4px;">
                                    <span style="font-size:11px; font-weight:500; color:var(--text-primary);">{format!("URDF #{}", idx)}</span>
                                    <button class="toggle-btn" style="font-size:10px; padding:1px 5px; color:#e55;"
                                        on:click=move |_| {
                                            layout.update_three_dee_config(node_id, move |c| {
                                                if idx < c.custom_layers.urdfs.len() {
                                                    c.custom_layers.urdfs.remove(idx);
                                                }
                                            });
                                        }
                                    >{"✕"}</button>
                                </div>
                                <div class="settings-row">
                                    <label class="settings-label">{"Visible"}</label>
                                    <div class="toggle-group">
                                        <button class="toggle-btn" class:active=!urdf_visible
                                            on:click=move |_| {
                                                layout.update_three_dee_config(node_id, move |c| {
                                                    if let Some(u) = c.custom_layers.urdfs.get_mut(idx) { u.visible = false; }
                                                });
                                            }>{"Off"}</button>
                                        <button class="toggle-btn" class:active=urdf_visible
                                            on:click=move |_| {
                                                layout.update_three_dee_config(node_id, move |c| {
                                                    if let Some(u) = c.custom_layers.urdfs.get_mut(idx) { u.visible = true; }
                                                });
                                            }>{"On"}</button>
                                    </div>
                                </div>
                                <div class="settings-row">
                                    <label class="settings-label">{"Frame"}</label>
                                    <select class="settings-select"
                                        on:change=move |ev: leptos::ev::Event| {
                                            if let Some(target) = ev.target() {
                                                if let Ok(select) = target.dyn_into::<web_sys::HtmlSelectElement>() {
                                                    let val = select.value();
                                                    layout.update_three_dee_config(node_id, move |c| {
                                                        if let Some(u) = c.custom_layers.urdfs.get_mut(idx) { u.frame_id = val; }
                                                    });
                                                }
                                            }
                                        }
                                    >
                                        {af.into_iter().map(|f| {
                                            let selected = f == urdf_frame;
                                            let fv = f.clone();
                                            view! { <option value=fv selected=selected>{f}</option> }
                                        }).collect::<Vec<_>>()}
                                    </select>
                                </div>
                                <div class="settings-row">
                                    <label class="settings-label">{"URL"}</label>
                                    <input type="text" class="settings-select" style="font-size:11px;"
                                        value=urdf_url
                                        on:change=move |ev: leptos::ev::Event| {
                                            if let Some(target) = ev.target() {
                                                if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                                                    let val = input.value();
                                                    layout.update_three_dee_config(node_id, move |c| {
                                                        if let Some(u) = c.custom_layers.urdfs.get_mut(idx) { u.url = val; }
                                                    });
                                                }
                                            }
                                        }
                                    />
                                </div>
                            </div>
                        }.into_any());
                    }

                    if views.is_empty() {
                        vec![view! { <div class="settings-placeholder">{"No layers. Use + Grid or + URDF to add."}</div> }.into_any()]
                    } else {
                        views
                    }
                }}
            </div>
        </div>

        // === PUBLISH ===
        <div class="settings-section" style=publish_style>
            <div class="settings-section-header">
                <h4 class="settings-section-title" on:click=move |_| publish_open.update(|v| *v = !*v)>
                    {move || if publish_open.get() { "▼ Publish" } else { "▶ Publish" }}
                </h4>
            </div>
            <div class="settings-section-body" class:collapsed=move || !publish_open.get()>
                <div class="settings-row">
                    <label class="settings-label">{"Type"}</label>
                    <select class="settings-select" on:change=on_publish_type_change>
                        <option value="point" selected=move || config().publish.publish_type == "point">{"Point"}</option>
                        <option value="pose" selected=move || config().publish.publish_type == "pose">{"Pose"}</option>
                        <option value="pose_estimate" selected=move || config().publish.publish_type == "pose_estimate">{"Pose Estimate"}</option>
                    </select>
                </div>
                <div class="settings-row">
                    <label class="settings-label">{"Topic"}</label>
                    <input type="text" class="settings-select"
                        value=move || config().publish.topic
                        on:change=on_publish_topic_change
                    />
                </div>
            </div>
        </div>
    }
}

/// Raw Messages panel settings - Topic + Font Size.
#[component]
fn RawMessagesSettings(node_id: NodeId, topic: Option<String>) -> impl IntoView {
    let layout = use_layout_state();
    let current_topic = topic.unwrap_or_default();

    let all_topics = move || -> Vec<String> {
        get_player().map(|p| {
            p.topics().iter()
                .map(|t| t.name.clone())
                .collect()
        }).unwrap_or_default()
    };

    let current_topic_for_view = current_topic.clone();

    let on_topic_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(select) = target.dyn_into::<web_sys::HtmlSelectElement>() {
                let val = select.value();
                let new_topic = if val.is_empty() { None } else { Some(val) };
                layout.set_panel_topic(node_id, new_topic);
            }
        }
    };

    let on_font_size_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(select) = target.dyn_into::<web_sys::HtmlSelectElement>() {
                let val = select.value();
                layout.set_panel_font_size(node_id, val);
            }
        }
    };

    let current_font_size = move || {
        layout.panel_font_sizes.with(|sizes| {
            sizes.get(&node_id).cloned().unwrap_or_else(|| "Auto".to_string())
        })
    };

    view! {
        <div class="settings-section">
            <h4 class="settings-section-title">{"▼ General"}</h4>
            <div class="settings-row">
                <label class="settings-label">{"Topic"}</label>
                <select class="settings-select" on:change=on_topic_change>
                    <option value="" selected=current_topic_for_view.is_empty()>{"— Select topic —"}</option>
                    {move || all_topics().into_iter().map(|t| {
                        let selected = t == current_topic;
                        let t_val = t.clone();
                        view! { <option value=t_val selected=selected>{t}</option> }
                    }).collect::<Vec<_>>()}
                </select>
            </div>
            <div class="settings-row">
                <label class="settings-label">{"Font Size"}</label>
                <select class="settings-select" on:change=on_font_size_change>
                    <option value="Auto" selected=move || current_font_size() == "Auto">{"Auto"}</option>
                    <option value="8px" selected=move || current_font_size() == "8px">{"8px"}</option>
                    <option value="9px" selected=move || current_font_size() == "9px">{"9px"}</option>
                    <option value="10px" selected=move || current_font_size() == "10px">{"10px"}</option>
                    <option value="11px" selected=move || current_font_size() == "11px">{"11px"}</option>
                    <option value="12px" selected=move || current_font_size() == "12px">{"12px"}</option>
                    <option value="14px" selected=move || current_font_size() == "14px">{"14px"}</option>
                    <option value="16px" selected=move || current_font_size() == "16px">{"16px"}</option>
                    <option value="18px" selected=move || current_font_size() == "18px">{"18px"}</option>
                    <option value="20px" selected=move || current_font_size() == "20px">{"20px"}</option>
                    <option value="24px" selected=move || current_font_size() == "24px">{"24px"}</option>
                    <option value="28px" selected=move || current_font_size() == "28px">{"28px"}</option>
                    <option value="32px" selected=move || current_font_size() == "32px">{"32px"}</option>
                    <option value="36px" selected=move || current_font_size() == "36px">{"36px"}</option>
                    <option value="48px" selected=move || current_font_size() == "48px">{"48px"}</option>
                    <option value="56px" selected=move || current_font_size() == "56px">{"56px"}</option>
                    <option value="64px" selected=move || current_font_size() == "64px">{"64px"}</option>
                    <option value="72px" selected=move || current_font_size() == "72px">{"72px"}</option>
                </select>
            </div>
        </div>
    }
}

/// Generic topic selector settings (for Log, Plot, etc).
#[component]
fn TopicSelectSettings(node_id: NodeId, topic: Option<String>, #[prop(into)] label: String) -> impl IntoView {
    let layout = use_layout_state();
    let current_topic = topic.unwrap_or_default();

    // Get all topics from the player
    let all_topics = move || -> Vec<String> {
        get_player().map(|p| {
            p.topics().iter()
                .map(|t| t.name.clone())
                .collect()
        }).unwrap_or_default()
    };

    let current_topic_for_view = current_topic.clone();

    let on_topic_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(select) = target.dyn_into::<web_sys::HtmlSelectElement>() {
                let val = select.value();
                let new_topic = if val.is_empty() { None } else { Some(val) };
                layout.set_panel_topic(node_id, new_topic);
            }
        }
    };

    view! {
        <div class="settings-section">
            <h4 class="settings-section-title">{"▼ General"}</h4>
            <div class="settings-row">
                <label class="settings-label">{label}</label>
                <select class="settings-select" on:change=on_topic_change>
                    <option value="" selected=current_topic_for_view.is_empty()>{"— Select topic —"}</option>
                    {move || all_topics().into_iter().map(|t| {
                        let selected = t == current_topic;
                        let t_val = t.clone();
                        view! { <option value=t_val selected=selected>{t}</option> }
                    }).collect::<Vec<_>>()}
                </select>
            </div>
        </div>
    }
}

// ============================================================================
// Variables Panel (right sidebar)
// ============================================================================

/// Variables panel for the right sidebar.
#[component]
fn VariablesPanel() -> impl IntoView {
    let state = use_app_state();
    let editing_idx = RwSignal::new(Option::<usize>::None);
    let edit_name = RwSignal::new(String::new());
    let edit_value = RwSignal::new(String::new());

    let on_add = move |_: leptos::ev::MouseEvent| {
        state.global_variables.update(|vars| {
            // Generate a unique name
            let mut i = 1u32;
            let name = loop {
                let candidate = format!("var{}", i);
                if !vars.iter().any(|(n, _)| *n == candidate) {
                    break candidate;
                }
                i += 1;
            };
            vars.push((name, "\"\"".to_string()));
        });
        // Start editing the new variable
        let len = state.global_variables.get_untracked().len();
        editing_idx.set(Some(len - 1));
        let vars = state.global_variables.get_untracked();
        if let Some((name, value)) = vars.last() {
            edit_name.set(name.clone());
            edit_value.set(value.clone());
        }
    };

    let on_edit_start = move |idx: usize| {
        let vars = state.global_variables.get_untracked();
        if let Some((name, value)) = vars.get(idx) {
            edit_name.set(name.clone());
            edit_value.set(value.clone());
            editing_idx.set(Some(idx));
        }
    };

    let on_edit_confirm = move |_: leptos::ev::MouseEvent| {
        if let Some(idx) = editing_idx.get_untracked() {
            let name = edit_name.get_untracked().trim().to_string();
            let value = edit_value.get_untracked();
            if !name.is_empty() {
                state.global_variables.update(|vars| {
                    if idx < vars.len() {
                        vars[idx] = (name, value);
                    }
                });
            }
        }
        editing_idx.set(None);
    };

    let on_edit_cancel = move |_: leptos::ev::MouseEvent| {
        editing_idx.set(None);
    };

    let on_delete = move |idx: usize| {
        state.global_variables.update(|vars| {
            if idx < vars.len() {
                vars.remove(idx);
            }
        });
        editing_idx.set(None);
    };

    view! {
        <div class="variables-panel">
            <div class="variables-header">
                <h4 class="variables-title">{"Variables"}</h4>
                <button class="variables-add-btn" on:click=on_add title="Add variable">
                    {"+ Add variable"}
                </button>
            </div>
            <div class="variables-list">
                {move || {
                    let vars = state.global_variables.get();
                    let current_editing = editing_idx.get();
                    if vars.is_empty() {
                        return view! {
                            <div class="variables-empty">
                                <p>{"No variables defined."}</p>
                                <p class="text-muted variables-hint">
                                    {"Variables are key/value pairs accessible to all panels via $variable_name in message paths."}
                                </p>
                            </div>
                        }.into_any();
                    }
                    view! {
                        <div class="variables-items">
                            {vars.into_iter().enumerate().map(|(idx, (name, value))| {
                                let is_editing = current_editing == Some(idx);
                                if is_editing {
                                    let on_name_input = move |ev: leptos::ev::Event| {
                                        edit_name.set(leptos::prelude::event_target_value(&ev));
                                    };
                                    let on_value_input = move |ev: leptos::ev::Event| {
                                        edit_value.set(leptos::prelude::event_target_value(&ev));
                                    };
                                    view! {
                                        <div class="variable-item variable-item-editing">
                                            <div class="variable-edit-row">
                                                <span class="variable-dollar">{"$"}</span>
                                                <input
                                                    type="text"
                                                    class="variable-name-input"
                                                    placeholder="name"
                                                    prop:value=move || edit_name.get()
                                                    on:input=on_name_input
                                                />
                                            </div>
                                            <div class="variable-edit-row">
                                                <input
                                                    type="text"
                                                    class="variable-value-input"
                                                    placeholder="value (JSON)"
                                                    prop:value=move || edit_value.get()
                                                    on:input=on_value_input
                                                />
                                            </div>
                                            <div class="variable-edit-actions">
                                                <button class="variable-btn variable-btn-confirm" on:click=on_edit_confirm>{"✓"}</button>
                                                <button class="variable-btn variable-btn-cancel" on:click=on_edit_cancel>{"✕"}</button>
                                            </div>
                                        </div>
                                    }.into_any()
                                } else {
                                    let idx_for_edit = idx;
                                    let idx_for_delete = idx;
                                    let name_display = name.clone();
                                    let value_display = value.clone();
                                    view! {
                                        <div class="variable-item"
                                            on:dblclick=move |_| on_edit_start(idx_for_edit)
                                        >
                                            <div class="variable-info">
                                                <span class="variable-name">{format!("${}", name_display)}</span>
                                                <span class="variable-value">{value_display}</span>
                                            </div>
                                            <div class="variable-actions">
                                                <button
                                                    class="variable-btn"
                                                    title="Edit"
                                                    on:click=move |_| on_edit_start(idx_for_edit)
                                                >{"✎"}</button>
                                                <button
                                                    class="variable-btn variable-btn-danger"
                                                    title="Delete"
                                                    on:click=move |_| on_delete(idx_for_delete)
                                                >{"🗑"}</button>
                                            </div>
                                        </div>
                                    }.into_any()
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    }.into_any()
                }}
            </div>
        </div>
    }
}

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
};

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
                            <div class="sidebar-section">
                                <h4>{"Performance"}</h4>
                                <p class="text-muted">{"Playback stats will appear here"}</p>
                            </div>
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
    // 0=Panel, 1=Topics, 2=Alerts, 3=Layouts
    // Default to Topics (1) when a data source is active
    let active_tab = RwSignal::new(1u8);

    // When a panel's settings gear is clicked, switch to Panel tab
    Effect::new(move |_| {
        if layout.active_settings_panel.get().is_some() {
            active_tab.set(0);
        }
    });

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
                    class="sidebar-tab-btn"
                    class:active=move || active_tab.get() == 2
                    on:click=move |_| active_tab.set(2)
                >{"Alerts"}</button>
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

/// Layouts tab: manage saved layouts.
#[component]
fn LayoutsTabContent() -> impl IntoView {
    let layout = use_layout_state();
    let state = use_app_state();

    let on_create_new = move |_: leptos::ev::MouseEvent| {
        // Create a default layout (current state is already the layout)
        // In a full implementation this would prompt for a name and save to localStorage
        let window = web_sys::window().unwrap();
        if let Ok(Some(storage)) = window.local_storage() {
            let layout_json = export_layout_json(&layout);
            let name = format!("Layout {}", js_sys::Date::new_0().to_locale_time_string("en-US"));
            let key = format!("layout:{}", name);
            storage.set_item(&key, &layout_json).ok();
        }
    };

    let on_import = move |_: leptos::ev::MouseEvent| {
        // Trigger file input for JSON import
        let document = web_sys::window().unwrap().document().unwrap();
        let input = document.create_element("input").unwrap();
        let input: web_sys::HtmlInputElement = input.dyn_into().unwrap();
        input.set_type("file");
        input.set_attribute("accept", ".json").ok();

        let layout_clone = layout;
        let state_clone = state;
        let input_clone = input.clone();
        let onchange = wasm_bindgen::closure::Closure::once(move |_: web_sys::Event| {
            let files = input_clone.files().unwrap();
            if let Some(file) = files.get(0) {
                let reader = web_sys::FileReader::new().unwrap();
                let reader_clone = reader.clone();
                let layout_for_load = layout_clone;
                let state_for_load = state_clone;
                let onload = wasm_bindgen::closure::Closure::once(move |_: web_sys::Event| {
                    if let Ok(result) = reader_clone.result() {
                        if let Some(text) = result.as_string() {
                            import_layout_json(&text, &layout_for_load, state_for_load);
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

    let on_export = move |_: leptos::ev::MouseEvent| {
        let json = export_layout_json(&layout);
        // Create and download the file
        let blob_parts = js_sys::Array::new();
        blob_parts.push(&wasm_bindgen::JsValue::from_str(&json));
        let opts = web_sys::BlobPropertyBag::new();
        opts.set_type("application/json");
        let blob = web_sys::Blob::new_with_str_sequence_and_options(&blob_parts, &opts).unwrap();
        let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let a = document.create_element("a").unwrap();
        a.set_attribute("href", &url).ok();
        a.set_attribute("download", "layout.json").ok();
        let a: web_sys::HtmlElement = a.dyn_into().unwrap();
        a.click();
        web_sys::Url::revoke_object_url(&url).ok();
    };

    // Get saved layouts from localStorage
    let saved_layouts = move || -> Vec<String> {
        let window = web_sys::window().unwrap();
        if let Ok(Some(storage)) = window.local_storage() {
            let len = storage.length().unwrap_or(0);
            let mut layouts = Vec::new();
            for i in 0..len {
                if let Ok(Some(key)) = storage.key(i) {
                    if key.starts_with("layout:") {
                        layouts.push(key[7..].to_string());
                    }
                }
            }
            layouts
        } else {
            Vec::new()
        }
    };

    view! {
        <div class="layouts-list">
            <div class="layouts-actions">
                <button class="layout-action-btn" on:click=on_create_new>
                    <span class="layout-action-icon">{"+"}</span>
                    {"Create new layout"}
                </button>
                <button class="layout-action-btn" on:click=on_import>
                    <span class="layout-action-icon">{"📁"}</span>
                    {"Import from file"}
                </button>
                <button class="layout-action-btn" on:click=on_export>
                    <span class="layout-action-icon">{"💾"}</span>
                    {"Export current layout"}
                </button>
            </div>
            <hr class="layouts-divider" />
            <div class="layouts-saved">
                <div class="layout-item layout-item-active">
                    <span class="layout-item-icon">{"📐"}</span>
                    <span class="layout-item-name">{"Default"}</span>
                </div>
                {move || saved_layouts().into_iter().map(|name| {
                    let layout_c = layout;
                    let state_c = state;
                    let name_clone = name.clone();
                    let on_load = move |_: leptos::ev::MouseEvent| {
                        let window = web_sys::window().unwrap();
                        if let Ok(Some(storage)) = window.local_storage() {
                            let key = format!("layout:{}", name_clone);
                            if let Ok(Some(json)) = storage.get_item(&key) {
                                import_layout_json(&json, &layout_c, state_c);
                            }
                        }
                    };
                    view! {
                        <div class="layout-item" on:click=on_load>
                            <span class="layout-item-icon">{"📐"}</span>
                            <span class="layout-item-name">{name}</span>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

/// Export current layout to JSON (matching Lichtblick format).
fn export_layout_json(layout: &LayoutState) -> String {
    let tree = layout.tree.get();
    let layout_json = layout_node_to_json(&tree);
    let configs_json = "{}"; // Simplified - could serialize panel configs
    format!(
        r#"{{"configById":{configs},"globalVariables":{{}},"userNodes":{{}},"playbackConfig":{{"speed":1}},"layout":{layout}}}"#,
        configs = configs_json,
        layout = layout_json
    )
}

/// Convert layout tree to JSON string.
fn layout_node_to_json(node: &LayoutNode) -> String {
    match node {
        LayoutNode::Panel { panel_type, .. } => {
            let type_name = match panel_type {
                PanelType::Image => "Image",
                PanelType::ThreeDee => "3D",
                PanelType::RawMessages => "RawMessages",
                PanelType::Log => "RosOut",
                PanelType::Plot => "Plot",
                PanelType::DataSourceInfo => "DataSourceInfo",
                PanelType::Diagnostics => "DiagnosticStatusPanel",
                PanelType::StateTransitions => "StateTransitions",
                PanelType::Teleop => "Teleop",
                _ => "Unknown",
            };
            // Panel ID format: "Type!random"
            format!(r#""{}!panel""#, type_name)
        }
        LayoutNode::Split { direction, ratio, first, second, .. } => {
            let dir = match direction {
                SplitDirection::Horizontal => "row",
                SplitDirection::Vertical => "column",
            };
            format!(
                r#"{{"first":{},"second":{},"direction":"{}","splitPercentage":{:.1}}}"#,
                layout_node_to_json(first),
                layout_node_to_json(second),
                dir,
                ratio
            )
        }
    }
}

/// Import a layout from JSON string.
fn import_layout_json(json: &str, layout: &LayoutState, _state: AppState) {
    // Parse the JSON and build a layout tree
    // Simplified parser - handles the Lichtblick format
    if let Some(layout_value) = extract_json_field(json, "layout") {
        if let Some(tree) = parse_layout_node(layout_value, &mut 1) {
            let next_id = count_nodes(&tree) as u32 + 1;
            layout.tree.set(tree);
            layout.next_id.set(next_id);
        }
    }
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

    let on_reset = move |_: leptos::ev::MouseEvent| {
        layout.reset_image_config(node_id);
        panel_menu_open.set(false);
    };

    view! {
        <div class="panel-settings">
            <div class="panel-settings-header">
                <span class="panel-settings-title">{format!("{} panel", panel_name)}</span>
                <div class="panel-settings-header-actions">
                    <div class="panel-settings-menu-wrapper">
                        <button class="panel-settings-menu-btn" on:click=on_panel_menu title="Options">{"⋮"}</button>
                        <div class="panel-settings-dropdown" class:open=move || panel_menu_open.get()>
                            <div class="panel-menu-item" on:mousedown=on_reset>
                                <span>{"Reset"}</span>
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
                    PanelType::RawMessages => view! { <TopicSelectSettings node_id=node_id topic=current_topic label="Topic" /> }.into_any(),
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
#[allow(unused_variables)]
#[component]
fn ThreeDeeSettings(node_id: NodeId) -> impl IntoView {
    view! {
        <div class="settings-section">
            <h4 class="settings-section-title">{"▼ Frame"}</h4>
            <div class="settings-row">
                <label class="settings-label">{"Display frame"}</label>
                <select class="settings-select">
                    <option selected=true>{"global"}</option>
                </select>
            </div>
            <div class="settings-row">
                <label class="settings-label">{"Follow mode"}</label>
                <select class="settings-select">
                    <option selected=true>{"Pose"}</option>
                </select>
            </div>
        </div>
        <div class="settings-section">
            <h4 class="settings-section-title">{"▶ Scene"}</h4>
        </div>
        <div class="settings-section">
            <h4 class="settings-section-title">{"▶ View"}</h4>
        </div>
        <div class="settings-section">
            <h4 class="settings-section-title">{"▶ Transforms"}</h4>
        </div>
        <div class="settings-section">
            <h4 class="settings-section-title">{"▶ Topics"}</h4>
        </div>
    }
}

/// Generic topic selector settings (for RawMessages, Log, Plot, etc).
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

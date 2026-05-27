// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

//! Raw Messages panel - displays decoded message as JSON tree.
//! Matches Lichtblick Node.js output: topic in toolbar, schema@timestamp header, JSON fields.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use leptos::prelude::*;

use crate::cdr_decoder::{self, SchemaMap};
use crate::state::app_state::{get_player, use_app_state, use_layout_state, NodeId};

thread_local! {
    static SCHEMA_CACHE: RefCell<HashMap<String, SchemaMap>> = RefCell::new(HashMap::new());
}

/// Raw Messages panel with topic selector in toolbar and JSON output.
#[component]
pub fn RawMessagesPanel(#[prop(into)] topic: String, node_id: NodeId) -> impl IntoView {
    let state = use_app_state();
    let layout = use_layout_state();
    let frame_tick = state.frame_tick;

    // Reactive font size from panel settings
    let font_size_style = move || {
        let size = layout.panel_font_sizes.with(|sizes| {
            sizes.get(&node_id).cloned().unwrap_or_else(|| "Auto".to_string())
        });
        if size == "Auto" {
            String::new()
        } else {
            format!("font-size: {}", size)
        }
    };

    let initial_topic = if topic.is_empty() { String::new() } else { topic };
    let selected_topic = RwSignal::new(initial_topic);
    let topic_list = RwSignal::new(Vec::<(String, String, String)>::new()); // (name, schema_name, encoding)
    let show_topic_picker = RwSignal::new(false);

    // Decoded message state
    let decoded_json = RwSignal::new(serde_json::Value::Null);
    let schema_name_display = RwSignal::new(String::new());
    let timestamp_display = RwSignal::new(String::new());
    let last_time = RwSignal::new(0u64);

    // Load topics list
    Effect::new(move |_| {
        let _tick = frame_tick.get();
        if let Some(player) = get_player() {
            let topics = player.topics();
            let list: Vec<(String, String, String)> = topics
                .iter()
                .map(|t| (t.name.clone(), t.schema_name.clone(), t.encoding.clone()))
                .collect();
            if !list.is_empty() && topic_list.get_untracked().is_empty() {
                topic_list.set(list);
            }
        }
    });

    // Decode messages
    Effect::new(move |_| {
        let _tick = frame_tick.get();
        let topic_name = selected_topic.get();
        if topic_name.is_empty() {
            return;
        }

        let player = match get_player() {
            Some(p) => p,
            None => return,
        };

        let msg = match player.get_current_message(&topic_name) {
            Some(m) => m,
            None => return,
        };

        if msg.log_time_ns == last_time.get_untracked() {
            return;
        }
        last_time.set(msg.log_time_ns);

        let time_secs = msg.log_time_ns as f64 / 1_000_000_000.0;
        timestamp_display.set(format!("{} sec", time_secs));
        schema_name_display.set(msg.schema_name.clone());

        // Decode the message to JSON
        let json_value = decode_to_json(&msg.data, &msg.schema_name, &msg.encoding, &player);
        decoded_json.set(json_value);
    });

    // Topic picker selection
    let on_select_topic = move |name: String| {
        selected_topic.set(name);
        show_topic_picker.set(false);
        last_time.set(0);
        decoded_json.set(serde_json::Value::Null);
    };

    // Copy JSON to clipboard
    let on_copy = move |_| {
        let json = decoded_json.get_untracked();
        if !json.is_null() {
            let text = serde_json::to_string_pretty(&json).unwrap_or_default();
            copy_to_clipboard(&text);
        }
    };

    view! {
        <div class="raw-messages-panel">
            <div class="raw-messages-toolbar">
                <div
                    class="raw-messages-topic-path"
                    on:click=move |_| show_topic_picker.update(|v| *v = !*v)
                >
                    {move || {
                        let t = selected_topic.get();
                        if t.is_empty() { "/Click to select topic".to_string() } else { t }
                    }}
                </div>
            </div>

            // Topic picker dropdown
            {move || {
                if show_topic_picker.get() {
                    let topics = topic_list.get();
                    let on_select = on_select_topic.clone();
                    view! {
                        <div class="raw-messages-topic-picker">
                            {topics.into_iter().map(move |(name, schema, _enc)| {
                                let n = name.clone();
                                let on_select = on_select.clone();
                                view! {
                                    <div
                                        class="raw-messages-topic-item"
                                        on:click=move |_| {
                                            on_select(n.clone());
                                        }
                                    >
                                        <span class="topic-name">{name.clone()}</span>
                                        <span class="topic-schema">{schema.clone()}</span>
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}

            // Message content
            <div class="raw-messages-content" style=move || font_size_style()>
                {move || {
                    let json = decoded_json.get();
                    if json.is_null() {
                        view! {
                            <div class="raw-messages-empty">"Waiting for messages…"</div>
                        }.into_any()
                    } else {
                        let schema_name = schema_name_display.get();
                        let timestamp = timestamp_display.get();
                        view! {
                            <div class="raw-messages-header">
                                <span class="raw-messages-schema-time">
                                    {format!("{} @ {}", schema_name, timestamp)}
                                </span>
                                <button
                                    class="raw-messages-copy-btn"
                                    on:click=on_copy
                                    title="Copy message JSON"
                                >
                                    "📋"
                                </button>
                            </div>
                            <div class="raw-messages-json-tree">
                                <JsonTree value=json.clone() indent=0 />
                            </div>
                        }.into_any()
                    }
                }}
            </div>
        </div>
    }
}

/// JSON tree display component — virtualized list for performance.
/// Only renders visible rows (based on scroll position), avoiding DOM explosion
/// with large OSI messages that can have 20,000+ nested fields.
#[derive(Clone, Debug, PartialEq)]
struct FlatRow {
    path: String,
    indent: usize,
    label: String,
    value_type: &'static str, // "object", "array", "primitive"
    summary: String,
    is_collapsible: bool,
    is_expanded: bool,
}

fn flatten_json(
    value: &serde_json::Value,
    label: &str,
    path: &str,
    indent: usize,
    expanded_paths: &HashSet<String>,
    rows: &mut Vec<FlatRow>,
) {
    let is_collapsible = value.is_object() || value.is_array();
    let is_expanded = expanded_paths.contains(path);

    let summary = match value {
        serde_json::Value::Object(map) => format!("{{}} {} keys", map.len()),
        serde_json::Value::Array(items) => format!("[] {} items", items.len()),
        _ => format_json_value(value),
    };

    let value_type = match value {
        serde_json::Value::Object(_) => "object",
        serde_json::Value::Array(_) => "array",
        _ => "primitive",
    };

    rows.push(FlatRow {
        path: path.to_string(),
        indent,
        label: label.to_string(),
        value_type,
        summary,
        is_collapsible,
        is_expanded,
    });

    if is_collapsible && is_expanded {
        match value {
            serde_json::Value::Object(map) => {
                let mut keys: Vec<&String> = map.keys().collect();
                keys.sort();
                for key in keys {
                    let child_val = map.get(key).unwrap();
                    let child_path = if path.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", path, key)
                    };
                    flatten_json(child_val, key, &child_path, indent + 1, expanded_paths, rows);
                }
            }
            serde_json::Value::Array(items) => {
                for (i, val) in items.iter().enumerate() {
                    let child_label = format!("[{}]", i);
                    let child_path = format!("{}[{}]", path, i);
                    flatten_json(val, &child_label, &child_path, indent + 1, expanded_paths, rows);
                }
            }
            _ => {}
        }
    }
}

fn get_visible_rows(root: &serde_json::Value, expanded_paths: &HashSet<String>) -> Vec<FlatRow> {
    let mut rows = Vec::new();
    match root {
        serde_json::Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            for key in keys {
                let val = map.get(key).unwrap();
                flatten_json(val, key, key, 0, expanded_paths, &mut rows);
            }
        }
        serde_json::Value::Array(items) => {
            for (i, val) in items.iter().enumerate() {
                let label = format!("[{}]", i);
                let path = format!("[{}]", i);
                flatten_json(val, &label, &path, 0, expanded_paths, &mut rows);
            }
        }
        _ => {
            flatten_json(root, "value", "value", 0, expanded_paths, &mut rows);
        }
    }
    rows
}

#[component]
fn JsonTree(value: serde_json::Value, #[prop(optional)] indent: usize) -> impl IntoView {
    let expanded_paths = RwSignal::new(HashSet::<String>::new());

    let value_clone = value.clone();
    let flat_rows = Memo::new(move |_| {
        expanded_paths.with(|paths| get_visible_rows(&value_clone, paths))
    });

    let scroll_container_ref = NodeRef::<leptos::html::Div>::new();
    let scroll_top = RwSignal::new(0i32);
    let container_height = RwSignal::new(400i32);

    // Measure container on mount
    Effect::new(move |_| {
        if let Some(el) = scroll_container_ref.get() {
            let h = el.client_height();
            if h > 0 {
                container_height.set(h);
            }
        }
    });

    let on_scroll = move |_| {
        if let Some(el) = scroll_container_ref.get() {
            scroll_top.set(el.scroll_top());
            let h = el.client_height();
            if h > 0 {
                container_height.set(h);
            }
        }
    };

    let row_height: i32 = 22;

    view! {
        <div
            node_ref=scroll_container_ref
            on:scroll=on_scroll
            style="position: relative; overflow-y: auto; height: 100%; width: 100%;"
        >
            // Spacer for total scrollable height
            <div style=move || format!(
                "height: {}px; width: 100%; pointer-events: none;",
                flat_rows.with(|r| r.len()) as i32 * row_height
            )></div>

            // Absolutely positioned visible rows
            <div style="position: absolute; top: 0; left: 0; width: 100%;">
                {move || {
                    let rows = flat_rows.get();
                    let st = scroll_top.get();
                    let ch = container_height.get();
                    let rh = row_height;

                    let start_idx = (st / rh).max(0) as usize;
                    let visible_count = (ch / rh) as usize + 4;
                    let end_idx = (start_idx + visible_count).min(rows.len());

                    (start_idx..end_idx).map(|idx| {
                        let row = rows[idx].clone();
                        let path = row.path.clone();
                        let is_expanded = row.is_expanded;
                        let is_collapsible = row.is_collapsible;

                        let toggle = move |ev: leptos::ev::MouseEvent| {
                            ev.stop_propagation();
                            let p = path.clone();
                            expanded_paths.update(|set| {
                                if is_expanded {
                                    set.remove(&p);
                                } else {
                                    set.insert(p);
                                }
                            });
                        };

                        let top_px = idx as i32 * rh;
                        let pad_left = row.indent * 16 + 8;

                        view! {
                            <div
                                style=format!(
                                    "position: absolute; top: {}px; left: 0; width: 100%; height: {}px; display: flex; align-items: center; white-space: nowrap; font-family: monospace; font-size: 12px; padding-left: {}px;",
                                    top_px, rh, pad_left
                                )
                            >
                                {if is_collapsible {
                                    view! {
                                        <span
                                            on:click=toggle
                                            style="cursor: pointer; margin-right: 6px; color: #888; user-select: none;"
                                        >
                                            {if is_expanded { "▼" } else { "▶" }}
                                        </span>
                                    }.into_any()
                                } else {
                                    view! { <span style="width: 16px; display: inline-block;"></span> }.into_any()
                                }}
                                <span style="color: #9cdcfe; margin-right: 4px;">{row.label}</span>
                                <span style="color: #ccc; margin-right: 6px;">":"</span>
                                {if is_collapsible && !is_expanded {
                                    view! {
                                        <span style="color: #6a9955; font-style: italic;">{row.summary}</span>
                                    }.into_any()
                                } else if is_collapsible && is_expanded {
                                    view! {
                                        <span style="color: #ffd700;">
                                            {if row.value_type == "object" { "{" } else { "[" }}
                                        </span>
                                    }.into_any()
                                } else {
                                    view! {
                                        <span style="color: #ce9178;">{row.summary}</span>
                                    }.into_any()
                                }}
                            </div>
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>
        </div>
    }
}

fn format_json_value(val: &serde_json::Value) -> String {
    match val {
        serde_json::Value::Null => "null".to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::String(s) => format!("\"{}\"", s),
        serde_json::Value::Array(items) => {
            let parts: Vec<String> = items.iter().map(|v| format_json_value(v)).collect();
            format!("[{}]", parts.join(", "))
        }
        serde_json::Value::Object(_) => "{…}".to_string(),
    }
}

/// Decode a message to JSON using the correct decoder for the encoding.
fn decode_to_json(
    data: &[u8],
    schema_name: &str,
    encoding: &str,
    player: &crate::player::McapPlayer,
) -> serde_json::Value {
    // Get or parse schema from thread-local cache
    let schema_map = SCHEMA_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some(existing) = cache.get(schema_name) {
            return existing.clone();
        }

        if let Some(schema_data) = player.get_schema_data(schema_name) {
            let parsed = match encoding {
                "protobuf" => cdr_decoder::parse_protobuf_schema(schema_name, &schema_data),
                _ => cdr_decoder::parse_ros_msg_schema(schema_name, &schema_data),
            };
            cache.insert(schema_name.to_string(), parsed.clone());
            return parsed;
        }

        SchemaMap::new()
    });

    if schema_map.is_empty() {
        return serde_json::json!({
            "__raw": format!("{} bytes, encoding: {}", data.len(), encoding)
        });
    }

    cdr_decoder::decode_message_to_json(data, schema_name, encoding, &schema_map)
}

/// Copy text to clipboard using the Web Clipboard API.
fn copy_to_clipboard(text: &str) {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = navigator, js_name = "clipboard")]
        static CLIPBOARD: web_sys::Clipboard;
    }

    let window = web_sys::window().unwrap();
    let nav = window.navigator();
    let clipboard = nav.clipboard();
    let _ = clipboard.write_text(text);
}

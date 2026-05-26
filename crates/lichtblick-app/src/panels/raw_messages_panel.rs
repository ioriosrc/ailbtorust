// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

//! Raw Messages panel - displays decoded message as JSON tree.
//! Matches Lichtblick Node.js output: topic in toolbar, schema@timestamp header, JSON fields.

use std::cell::RefCell;
use std::collections::HashMap;

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

/// JSON tree display component — collapsed by default, lazy render on expand.
#[component]
fn JsonTree(value: serde_json::Value, indent: usize) -> impl IntoView {
    let padding = format!("padding-left: {}px", indent * 16);

    match value {
        serde_json::Value::Object(map) => {
            let entries: Vec<_> = map.into_iter().collect();
            view! {
                <div class="json-object" style=padding>
                    {entries.into_iter().map(|(key, val)| {
                        let is_nested = matches!(&val, serde_json::Value::Object(_) | serde_json::Value::Array(_));
                        if is_nested {
                            let summary = summarize_value(&val);
                            view! {
                                <JsonCollapsible key=key summary=summary value=val indent=indent+1 />
                            }.into_any()
                        } else {
                            let display = format_json_value(&val);
                            view! {
                                <div class="json-field-inline">
                                    <span class="json-key">{format!("{}: ", key)}</span>
                                    <span class="json-value">{display}</span>
                                </div>
                            }.into_any()
                        }
                    }).collect::<Vec<_>>()}
                </div>
            }.into_any()
        }
        serde_json::Value::Array(items) => {
            if items.is_empty() {
                view! {
                    <span class="json-value" style=padding>"[]  0 items"</span>
                }.into_any()
            } else if items.len() <= 8 && items.iter().all(|v| !v.is_object() && !v.is_array()) {
                // Short primitive array - display inline
                let display = format_json_value(&serde_json::Value::Array(items));
                view! {
                    <span class="json-value" style=padding>{display}</span>
                }.into_any()
            } else {
                let count = items.len();
                view! {
                    <div class="json-array" style=padding>
                        {items.into_iter().enumerate().map(move |(i, val)| {
                            let is_nested = matches!(&val, serde_json::Value::Object(_) | serde_json::Value::Array(_));
                            if is_nested {
                                let summary = summarize_value(&val);
                                let key = format!("[{}]", i);
                                view! {
                                    <JsonCollapsible key=key summary=summary value=val indent=0 />
                                }.into_any()
                            } else {
                                let display = format_json_value(&val);
                                view! {
                                    <div class="json-field-inline">
                                        <span class="json-index">{format!("[{}]: ", i)}</span>
                                        <span class="json-value">{display}</span>
                                    </div>
                                }.into_any()
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                }.into_any()
            }
        }
        _ => {
            let display = format_json_value(&value);
            view! {
                <span class="json-value" style=padding>{display}</span>
            }.into_any()
        }
    }
}

/// Collapsible node — collapsed by default, renders children only when expanded.
#[component]
fn JsonCollapsible(
    key: String,
    summary: String,
    value: serde_json::Value,
    indent: usize,
) -> impl IntoView {
    let expanded = RwSignal::new(false);

    view! {
        <div class="json-collapsible">
            <div
                class="json-collapsible-header"
                on:click=move |_| expanded.update(|v| *v = !*v)
                style="cursor: pointer; user-select: none;"
            >
                <span class="json-toggle">{move || if expanded.get() { "▼ " } else { "▶ " }}</span>
                <span class="json-key">{format!("{}  ", key)}</span>
                <span class="json-summary">{summary.clone()}</span>
            </div>
            {move || {
                if expanded.get() {
                    view! {
                        <div class="json-collapsible-body">
                            <JsonTree value=value.clone() indent=indent />
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}
        </div>
    }
}

/// Generate a short summary for collapsed nested values.
fn summarize_value(val: &serde_json::Value) -> String {
    match val {
        serde_json::Value::Object(map) => format!("{{}}  {} keys", map.len()),
        serde_json::Value::Array(items) => {
            if items.is_empty() {
                "[]  0 items".to_string()
            } else {
                format!("[]  {} items", items.len())
            }
        }
        _ => String::new(),
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

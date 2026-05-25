// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

//! Data Source Info panel - shows info about the loaded MCAP.

use leptos::prelude::*;

use crate::state::app_state::{get_player, use_app_state};

/// Data source info panel.
#[component]
pub fn DataSourceInfoPanel() -> impl IntoView {
    let state = use_app_state();
    let topics_expanded = RwSignal::new(false);
    let search_query = RwSignal::new(String::new());
    let copied_topic = RwSignal::new(Option::<String>::None);

    let topics_list = move || {
        if let Some(player) = get_player() {
            let mut topics = player.topics();
            topics.sort_by(|a, b| a.name.cmp(&b.name));
            topics
        } else {
            vec![]
        }
    };

    let filtered_topics = move || {
        let query = search_query.get().to_lowercase();
        let all = topics_list();
        if query.is_empty() {
            all
        } else {
            all.into_iter()
                .filter(|t| t.name.to_lowercase().contains(&query) || t.schema_name.to_lowercase().contains(&query))
                .collect()
        }
    };

    view! {
        <div class="datasource-info-content">
            <div class="info-grid">
                // Topics row - clickable to expand
                <div
                    class="info-row info-row-clickable"
                    on:click=move |_| topics_expanded.update(|v| *v = !*v)
                >
                    <span class="info-label">
                        <span class="info-collapse-arrow" class:info-collapse-arrow-open=move || topics_expanded.get()>
                            {"▶"}
                        </span>
                        {"Topics"}
                    </span>
                    <span class="info-value">{move || state.topic_count.get().to_string()}</span>
                </div>

                // Collapsible topics list
                {move || {
                    if topics_expanded.get() {
                        let on_search = move |ev: leptos::ev::Event| {
                            let val = leptos::prelude::event_target_value(&ev);
                            search_query.set(val);
                        };
                        let topics = filtered_topics();
                        let total = topics_list().len();
                        let filtered_count = topics.len();
                        view! {
                            <div class="topics-collapse-section">
                                <div class="topics-search-row">
                                    <input
                                        type="text"
                                        class="topics-search-input"
                                        placeholder="Search topics..."
                                        prop:value=move || search_query.get()
                                        on:input=on_search
                                    />
                                    {move || {
                                        let q = search_query.get();
                                        if !q.is_empty() {
                                            let total = total;
                                            let count = filtered_count;
                                            Some(view! {
                                                <span class="topics-search-count">{format!("{}/{}", count, total)}</span>
                                            })
                                        } else {
                                            None
                                        }
                                    }}
                                </div>
                                <div class="topics-collapse-list">
                                    {topics.into_iter().map(|t| {
                                        let topic_name = t.name.clone();
                                        let topic_for_copy = topic_name.clone();
                                        let topic_for_check = topic_name.clone();
                                        let on_copy = move |ev: leptos::ev::MouseEvent| {
                                            ev.stop_propagation();
                                            let name = topic_for_copy.clone();
                                            // Copy to clipboard via JS eval
                                            let js_code = format!("navigator.clipboard.writeText('{}')", name.replace('\\', "\\\\").replace('\'', "\\'"));
                                            let _ = js_sys::eval(&js_code);
                                            copied_topic.set(Some(name));
                                            // Reset after 1.5s
                                            let handle = leptos::prelude::set_timeout(move || {
                                                copied_topic.set(None);
                                            }, std::time::Duration::from_millis(1500));
                                            std::mem::forget(handle);
                                        };
                                        view! {
                                            <div class="topic-collapse-item">
                                                <div class="topic-collapse-name">
                                                    <span class="topic-collapse-topic">{topic_name.clone()}</span>
                                                    <span class="topic-collapse-schema">{t.schema_name.clone()}</span>
                                                </div>
                                                <button
                                                    class="topic-copy-btn"
                                                    title="Copy topic name"
                                                    on:click=on_copy
                                                >
                                                    {move || {
                                                        if copied_topic.get().as_deref() == Some(&topic_for_check) {
                                                            "✓"
                                                        } else {
                                                            "⧉"
                                                        }
                                                    }}
                                                </button>
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }
                }}

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

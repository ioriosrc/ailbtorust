// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::state::app_state::{get_player, use_app_state};

/// Topic list sidebar component - shows topics with frequency and message count.
#[component]
pub fn TopicList() -> impl IntoView {
    let state = use_app_state();
    let filter = RwSignal::new(String::new());

    let on_filter = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                filter.set(input.value().to_lowercase());
            }
        }
    };

    view! {
        <div class="topic-list">
            <div class="topic-list-header">
                <div class="topic-search-wrapper">
                    <span class="topic-search-icon">{"🔍"}</span>
                    <input
                        type="text"
                        class="topic-search"
                        placeholder="Filter by topic or schema name..."
                        on:input=on_filter
                    />
                </div>
            </div>
            <div class="topic-list-content">
                {move || {
                    let _tick = state.frame_tick.get();
                    let player = match get_player() {
                        Some(p) => p,
                        None => return view! {
                            <p class="text-muted">{"No topics available. Open a data source."}</p>
                        }.into_any(),
                    };

                    let topics = player.topics();
                    let stats = player.topic_stats();
                    let filter_val = filter.get();

                    let filtered: Vec<_> = topics.iter()
                        .filter(|t| {
                            if filter_val.is_empty() {
                                return true;
                            }
                            t.name.to_lowercase().contains(&filter_val)
                                || t.schema_name.to_lowercase().contains(&filter_val)
                        })
                        .collect();

                    if filtered.is_empty() {
                        return view! {
                            <p class="text-muted">{"No matching topics."}</p>
                        }.into_any();
                    }

                    view! {
                        <ul class="topic-items">
                            {filtered.into_iter().map(|t| {
                                let (count, hz) = stats.get(&t.name).copied().unwrap_or((0, 0.0));
                                let hz_str = if hz > 0.0 { format!("{:.2} Hz", hz) } else { "\u{2013}".to_string() };
                                let count_str = count.to_string();
                                let schema_short = t.schema_name.rsplit('/').next().unwrap_or(&t.schema_name).to_string();
                                let topic_name = t.name.clone();
                                view! {
                                    <li class="topic-item">
                                        <div class="topic-item-left">
                                            <span class="topic-name">{topic_name}</span>
                                            <span class="topic-schema">{schema_short}</span>
                                        </div>
                                        <div class="topic-item-right">
                                            <span class="topic-hz">{hz_str}</span>
                                            <span class="topic-count">{count_str}</span>
                                        </div>
                                    </li>
                                }
                            }).collect_view()}
                        </ul>
                    }.into_any()
                }}
            </div>
        </div>
    }
}

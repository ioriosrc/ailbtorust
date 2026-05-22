// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use leptos::prelude::*;

/// Topic list sidebar component.
#[component]
pub fn TopicList() -> impl IntoView {
    // In the full implementation, this reads from the player state
    let topics: Vec<(&str, &str, u64)> = vec![];

    view! {
        <div class="topic-list">
            <div class="topic-list-header">
                <input
                    type="text"
                    class="topic-search"
                    placeholder="Filter topics..."
                />
            </div>
            <div class="topic-list-content">
                {if topics.is_empty() {
                    view! {
                        <p class="text-muted">{"No topics available. Open a data source."}</p>
                    }.into_any()
                } else {
                    view! {
                        <ul class="topic-items">
                            {topics.into_iter().map(|(name, schema, count)| {
                                view! {
                                    <li class="topic-item">
                                        <span class="topic-name">{name}</span>
                                        <span class="topic-schema">{schema}</span>
                                        <span class="topic-count">{count.to_string()}</span>
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

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

//! Topic list panel - shows all available topics with schema and message count.

use leptos::prelude::*;

use crate::decoder::{is_compressed_image_schema, is_3d_schema};
use crate::state::app_state::get_player;

/// Topic list component for the sidebar.
#[component]
pub fn TopicList() -> impl IntoView {
    let topics = RwSignal::new(Vec::<TopicEntry>::new());

    // Load topics when player is available
    Effect::new(move |_| {
        if let Some(player) = get_player() {
            let infos = player.topics();
            let entries: Vec<TopicEntry> = infos
                .into_iter()
                .map(|t| {
                    let icon = if is_compressed_image_schema(&t.schema_name) {
                        "📷"
                    } else if is_3d_schema(&t.schema_name) {
                        "🔲"
                    } else if t.schema_name.contains("Log") || t.name.contains("rosout") {
                        "📝"
                    } else {
                        "📨"
                    };
                    TopicEntry {
                        name: t.name,
                        schema: t.schema_name,
                        count: t.message_count,
                        icon: icon.to_string(),
                    }
                })
                .collect();
            topics.set(entries);
        }
    });

    view! {
        <div class="topic-list">
            <div class="topic-list-header">
                <span class="topic-list-title">{"Topics"}</span>
                <span class="topic-list-count">{move || topics.get().len().to_string()}</span>
            </div>
            <div class="topic-list-items">
                {move || topics.get().into_iter().map(|entry| {
                    view! {
                        <div class="topic-list-item" title=entry.schema.clone()>
                            <span class="topic-icon">{entry.icon}</span>
                            <span class="topic-name">{entry.name}</span>
                            <span class="topic-count">{entry.count.to_string()}</span>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

#[derive(Clone)]
struct TopicEntry {
    name: String,
    schema: String,
    count: usize,
    icon: String,
}

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use leptos::prelude::*;

/// Panel toolbar shown at the top of each panel.
#[component]
pub fn PanelToolbar(
    #[prop(into)] title: String,
    #[prop(optional)] topic: Option<String>,
) -> impl IntoView {
    view! {
        <div class="panel-toolbar">
            <span class="panel-title">{title}</span>
            {topic.map(|t| view! {
                <span class="panel-topic">{t}</span>
            })}
            <div class="panel-toolbar-actions">
                <button class="panel-toolbar-btn" title="Split">{"⬜"}</button>
                <button class="panel-toolbar-btn" title="Settings">{"⚙️"}</button>
                <button class="panel-toolbar-btn" title="Close">{"✕"}</button>
            </div>
        </div>
    }
}

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use leptos::prelude::*;

use crate::panels::topic_list::TopicList;
use crate::state::app_state::use_app_state;

/// Sidebar component (left or right).
#[component]
pub fn Sidebar(
    #[prop(into)] side: String,
    open: RwSignal<bool>,
) -> impl IntoView {
    let side_clone = side.clone();
    let is_left = side == "left";
    let state = use_app_state();

    let class = move || {
        let base = format!("sidebar sidebar-{}", side_clone);
        if open.get() {
            format!("{} open", base)
        } else {
            base
        }
    };

    view! {
        <aside class=class>
            <div class="sidebar-content">
                {if is_left {
                    view! {
                        <div class="sidebar-panel">
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
        </aside>
    }
}

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::decoder::is_compressed_image_schema;
use crate::panels::topic_list::TopicList;
use crate::state::app_state::{
    get_player, use_app_state, use_layout_state,
    LayoutNode, NodeId, PanelType,
};

/// Sidebar component (left or right).
#[component]
pub fn Sidebar(
    #[prop(into)] side: String,
    open: RwSignal<bool>,
) -> impl IntoView {
    let side_clone = side.clone();
    let is_left = side == "left";
    let _state = use_app_state();

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
        </aside>
    }
}

/// Left sidebar content: shows panel settings when a panel is selected, otherwise topic list.
#[component]
fn LeftSidebarContent() -> impl IntoView {
    let state = use_app_state();
    let layout = use_layout_state();

    view! {
        {move || {
            let settings_panel = layout.active_settings_panel.get();
            if let Some(panel_id) = settings_panel {
                // Show settings for the selected panel
                let tree = layout.tree.get();
                if let Some(node) = find_panel_in_tree(&tree, panel_id) {
                    view! { <PanelSettingsView node_id=panel_id node=node /> }.into_any()
                } else {
                    // Panel was removed
                    layout.active_settings_panel.set(None);
                    view! { <FallbackContent /> }.into_any()
                }
            } else if state.has_active_layout.get() {
                view! { <FallbackContent /> }.into_any()
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

/// Default sidebar content when no panel settings are open.
#[component]
fn FallbackContent() -> impl IntoView {
    view! { <TopicList /> }
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

    let on_close = move |_: leptos::ev::MouseEvent| {
        layout.active_settings_panel.set(None);
    };

    view! {
        <div class="panel-settings">
            <div class="panel-settings-header">
                <span class="panel-settings-title">{format!("{} panel", panel_name)}</span>
                <button class="panel-settings-close" on:click=on_close title="Close settings">{"✕"}</button>
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

/// Image panel settings - topic selector + image options.
#[component]
fn ImageSettings(node_id: NodeId, topic: Option<String>) -> impl IntoView {
    let layout = use_layout_state();
    let current_topic = topic.unwrap_or_default();

    // Get available image topics from the player
    let image_topics = move || -> Vec<String> {
        get_player().map(|p| {
            p.topics().iter()
                .filter(|t| is_compressed_image_schema(&t.schema_name))
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
                <label class="settings-label">{"Topic"}</label>
                <select class="settings-select" on:change=on_topic_change>
                    <option value="" selected=current_topic_for_view.is_empty()>{"— Select topic —"}</option>
                    {move || image_topics().into_iter().map(|t| {
                        let selected = t == current_topic;
                        let t_val = t.clone();
                        view! { <option value=t_val selected=selected>{t}</option> }
                    }).collect::<Vec<_>>()}
                </select>
            </div>
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

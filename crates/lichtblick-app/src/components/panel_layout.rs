// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::decoder::is_compressed_image_schema;
use crate::panels::data_source_info::DataSourceInfoPanel;
use crate::panels::diagnostics_panel::DiagnosticsPanel;
use crate::panels::image_panel::ImagePanel;
use crate::panels::log_panel::LogPanel;
use crate::panels::plot_panel::PlotPanel;
use crate::panels::raw_messages_panel::RawMessagesPanel;
use crate::panels::state_transitions_panel::StateTransitionsPanel;
use crate::panels::teleop_panel::TeleopPanel;
use crate::panels::three_dee_panel::{is_point_cloud_schema, ThreeDeePanel};
use crate::panels::topic_list::TopicList;
use crate::state::app_state::{
    get_player, use_app_state, use_layout_state,
    LayoutNode, LayoutState, NodeId, PanelType, SplitDirection,
};

/// Panel layout manager.
#[component]
pub fn PanelLayout() -> impl IntoView {
    let state = use_app_state();
    let layout = use_layout_state();
    let has_layout = move || state.has_active_layout.get();
    let layout_initialized = RwSignal::new(false);

    // Set up default layout when data loads (only if no saved layout was restored)
    Effect::new(move |_| {
        if !state.has_active_layout.get() {
            return;
        }
        if layout_initialized.get_untracked() {
            return;
        }
        // Skip if a layout was already restored from localStorage
        if !layout.saved_tree_json.get_untracked().is_empty() {
            layout_initialized.set(true);
            return;
        }
        if let Some(player) = get_player() {
            let topics = player.topics();
            let image_topic = topics.iter()
                .find(|t| is_compressed_image_schema(&t.schema_name))
                .map(|t| t.name.clone());
            let has_pc = topics.iter().any(|t| is_point_cloud_schema(&t.schema_name));
            let first_other = topics.iter()
                .find(|t| !is_compressed_image_schema(&t.schema_name) && !is_point_cloud_schema(&t.schema_name))
                .map(|t| t.name.clone());
            layout.set_default_layout(image_topic, has_pc, first_other);
            layout_initialized.set(true);
        }
    });

    // Handle Escape key for fullscreen exit
    Effect::new(move |_| {
        let _ = layout.fullscreen_panel.get();
        // Register keydown listener
        let document = web_sys::window().unwrap().document().unwrap();
        let closure = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(move |ev: web_sys::KeyboardEvent| {
            if ev.key() == "Escape" {
                layout.fullscreen_panel.set(None);
            }
        });
        document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).ok();
        closure.forget();
    });

    view! {
        <div class="panel-layout">
            {move || if has_layout() {
                let tree = layout.tree.get();
                let fullscreen = layout.fullscreen_panel.get();
                if let Some(fs_id) = fullscreen {
                    // Show fullscreen panel on top
                    view! {
                        <div class="panel-fullscreen-backdrop">
                            <FullscreenPanel node_id=fs_id tree=tree />
                        </div>
                    }.into_any()
                } else {
                    view! { <LayoutNodeView node=tree /> }.into_any()
                }
            } else {
                view! { <EmptyLayout /> }.into_any()
            }}
        </div>
    }
}

/// Render a fullscreen panel by finding it in the tree.
#[component]
fn FullscreenPanel(node_id: NodeId, tree: LayoutNode) -> impl IntoView {
    let panel_node = find_panel_in_tree(&tree, node_id);
    if let Some(node) = panel_node {
        view! { <PanelContainer node=node /> }.into_any()
    } else {
        view! { <div></div> }.into_any()
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

/// Recursive layout node renderer.
#[component]
fn LayoutNodeView(node: LayoutNode) -> impl IntoView {
    match node {
        LayoutNode::Panel { .. } => {
            view! { <PanelContainer node=node /> }.into_any()
        }
        LayoutNode::Split { id, direction, ratio, first, second } => {
            let ratio_signal = RwSignal::new(ratio);
            let is_horizontal = direction == SplitDirection::Horizontal;
            let split_id = id;

            view! {
                <div class="mosaic-split"
                    class:mosaic-split-h=is_horizontal
                    class:mosaic-split-v=!is_horizontal
                >
                    <div class="mosaic-pane mosaic-first"
                        style=move || format!("flex: 0 0 calc({}% - 2px)", ratio_signal.get())
                    >
                        <LayoutNodeView node=*first />
                    </div>
                    <SplitHandle is_horizontal=is_horizontal ratio=ratio_signal split_id=split_id />
                    <div class="mosaic-pane mosaic-second"
                        style=move || format!("flex: 0 0 calc({}% - 2px)", 100.0 - ratio_signal.get())
                    >
                        <LayoutNodeView node=*second />
                    </div>
                </div>
            }.into_any()
        }
    }
}

/// Draggable split handle.
#[component]
fn SplitHandle(is_horizontal: bool, ratio: RwSignal<f64>, split_id: NodeId) -> impl IntoView {
    let layout = use_layout_state();
    let is_dragging = RwSignal::new(false);

    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        is_dragging.set(true);

        let document = web_sys::window().unwrap().document().unwrap();
        let window = web_sys::window().unwrap();

        document.body().unwrap().class_list().add_1("splitter-dragging").ok();

        let move_cb: std::rc::Rc<std::cell::RefCell<Option<Closure<dyn FnMut(web_sys::MouseEvent)>>>> =
            std::rc::Rc::new(std::cell::RefCell::new(None));
        let up_cb: std::rc::Rc<std::cell::RefCell<Option<Closure<dyn FnMut(web_sys::MouseEvent)>>>> =
            std::rc::Rc::new(std::cell::RefCell::new(None));

        let move_cb_clone = move_cb.clone();
        let up_cb_clone = up_cb.clone();

        let target = ev.current_target().unwrap();
        let handle_el: web_sys::HtmlElement = target.dyn_into().unwrap();
        let parent = handle_el.parent_element().unwrap();
        let parent_rect = parent.get_bounding_client_rect();
        let parent_left = parent_rect.left();
        let parent_top = parent_rect.top();
        let parent_width = parent_rect.width();
        let parent_height = parent_rect.height();

        let mousemove = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |ev: web_sys::MouseEvent| {
            let pct = if is_horizontal {
                let x = ev.client_x() as f64 - parent_left;
                (x / parent_width * 100.0).clamp(15.0, 85.0)
            } else {
                let y = ev.client_y() as f64 - parent_top;
                (y / parent_height * 100.0).clamp(15.0, 85.0)
            };
            ratio.set(pct);
        });

        let window_clone = window.clone();
        let doc_clone = document.clone();
        let mouseup = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
            is_dragging.set(false);
            doc_clone.body().unwrap().class_list().remove_1("splitter-dragging").ok();
            if let Some(cb) = move_cb_clone.borrow().as_ref() {
                window_clone.remove_event_listener_with_callback("mousemove", cb.as_ref().unchecked_ref()).ok();
            }
            if let Some(cb) = up_cb_clone.borrow().as_ref() {
                window_clone.remove_event_listener_with_callback("mouseup", cb.as_ref().unchecked_ref()).ok();
            }
            // Sync the ratio back to the tree and mark dirty
            let new_ratio = ratio.get_untracked();
            layout.tree.update(|tree| {
                update_ratio_in_tree(tree, split_id, new_ratio);
            });
            layout.mark_dirty();
        });

        window.add_event_listener_with_callback("mousemove", mousemove.as_ref().unchecked_ref()).unwrap();
        window.add_event_listener_with_callback("mouseup", mouseup.as_ref().unchecked_ref()).unwrap();

        *move_cb.borrow_mut() = Some(mousemove);
        *up_cb.borrow_mut() = Some(mouseup);
    };

    view! {
        <div
            class="splitter"
            class:splitter-horizontal=is_horizontal
            class:splitter-vertical=!is_horizontal
            class:active=move || is_dragging.get()
            on:mousedown=on_mousedown
        ></div>
    }
}

/// A single panel container with toolbar and context menu.
#[component]
fn PanelContainer(node: LayoutNode) -> impl IntoView {
    let layout = use_layout_state();
    let app_state = use_app_state();

    let (node_id, panel_type, topic) = match &node {
        LayoutNode::Panel { id, panel_type, topic } => (*id, panel_type.clone(), topic.clone()),
        _ => return view! { <div></div> }.into_any(),
    };

    let menu_open = RwSignal::new(false);
    let submenu_open = RwSignal::new(false);

    let title = panel_type.display_name().to_string();

    let on_settings = move |_: leptos::ev::MouseEvent| {
        layout.toggle_settings(node_id);
        // Open left sidebar when settings open
        if layout.active_settings_panel.get_untracked() == Some(node_id) {
            app_state.left_sidebar_open.set(true);
        }
    };

    let toggle_menu = move |_: leptos::ev::MouseEvent| {
        let is_open = menu_open.get_untracked();
        menu_open.set(!is_open);
        if is_open {
            submenu_open.set(false);
        }
    };

    // Close menu on outside click
    let close_menu = move || {
        let close = Closure::once(move || {
            menu_open.set(false);
            submenu_open.set(false);
        });
        web_sys::window().unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                close.as_ref().unchecked_ref(), 150
            ).ok();
        close.forget();
    };

    let on_blur = move |_: leptos::ev::FocusEvent| {
        close_menu();
    };

    let on_split_right = move |_: leptos::ev::MouseEvent| {
        layout.split_panel(node_id, SplitDirection::Horizontal);
        menu_open.set(false);
    };

    let on_split_down = move |_: leptos::ev::MouseEvent| {
        layout.split_panel(node_id, SplitDirection::Vertical);
        menu_open.set(false);
    };

    let on_fullscreen = move |_: leptos::ev::MouseEvent| {
        let current = layout.fullscreen_panel.get_untracked();
        if current == Some(node_id) {
            layout.fullscreen_panel.set(None);
        } else {
            layout.fullscreen_panel.set(Some(node_id));
        }
        menu_open.set(false);
    };

    let on_remove = move |_: leptos::ev::MouseEvent| {
        layout.remove_panel(node_id);
        menu_open.set(false);
    };

    let on_change_hover = move |_: leptos::ev::MouseEvent| {
        submenu_open.set(true);
    };

    let on_panel_click = move |_: leptos::ev::MouseEvent| {
        // Select this panel for settings sidebar when clicked
        layout.active_settings_panel.set(Some(node_id));
    };

    view! {
        <div class="panel-container" on:click=on_panel_click>
            <div class="panel-toolbar">
                <span class="panel-title">{title}</span>
                {topic.clone().map(|t| view! {
                    <span class="panel-topic">{t}</span>
                })}
                <div class="panel-toolbar-actions">
                    <button class="panel-toolbar-btn" title="Settings" on:click=on_settings>{"⚙"}</button>
                    <button
                        class="panel-toolbar-btn panel-menu-trigger"
                        title="Panel menu"
                        on:click=toggle_menu
                        on:blur=on_blur
                    >{"⋮"}</button>
                </div>
                // Context menu dropdown
                <div class="panel-context-menu" class:panel-context-menu-open=move || menu_open.get()>
                    <div class="panel-menu-item panel-menu-item-submenu"
                        on:mouseenter=on_change_hover
                    >
                        <span>{"Change panel"}</span>
                        <span class="menu-arrow">{"›"}</span>
                        <div class="panel-submenu" class:panel-submenu-open=move || submenu_open.get()>
                            {PanelType::all().iter().map(|pt| {
                                let pt_clone = pt.clone();
                                let name = pt.display_name().to_string();
                                view! {
                                    <div class="panel-menu-item"
                                        on:mousedown=move |_: leptos::ev::MouseEvent| {
                                            layout.change_panel(node_id, pt_clone.clone());
                                            menu_open.set(false);
                                            submenu_open.set(false);
                                        }
                                    >
                                        <span>{name}</span>
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                    <div class="panel-menu-separator"></div>
                    <div class="panel-menu-item" on:mousedown=on_split_right>
                        <span>{"Split right"}</span>
                    </div>
                    <div class="panel-menu-item" on:mousedown=on_split_down>
                        <span>{"Split down"}</span>
                    </div>
                    <div class="panel-menu-separator"></div>
                    <div class="panel-menu-item" on:mousedown=on_fullscreen>
                        <span>{"Fullscreen"}</span>
                    </div>
                    <div class="panel-menu-separator"></div>
                    <div class="panel-menu-item panel-menu-item-danger" on:mousedown=on_remove>
                        <span>{"Remove panel"}</span>
                    </div>
                </div>
            </div>
            <div class="panel-content">
                <PanelContent panel_type=panel_type topic=topic node_id=node_id />
            </div>
        </div>
    }.into_any()
}

/// Render the actual panel content based on type.
#[component]
fn PanelContent(panel_type: PanelType, topic: Option<String>, node_id: NodeId) -> impl IntoView {
    match panel_type {
        PanelType::Image => {
            let t = topic.unwrap_or_default();
            if t.is_empty() {
                view! { <div class="panel-empty">{"Select an image topic"}</div> }.into_any()
            } else {
                view! { <ImagePanel topic=t node_id=node_id /> }.into_any()
            }
        }
        PanelType::ThreeDee => view! { <ThreeDeePanel /> }.into_any(),
        PanelType::RawMessages => {
            let t = topic.unwrap_or_default();
            if t.is_empty() {
                view! { <div class="panel-empty">{"Select a topic"}</div> }.into_any()
            } else {
                view! { <RawMessagesPanel topic=t /> }.into_any()
            }
        }
        PanelType::DataSourceInfo => view! { <DataSourceInfoPanel /> }.into_any(),
        PanelType::Log => {
            let t = topic.unwrap_or_default();
            view! { <LogPanel topic=t /> }.into_any()
        }
        PanelType::Plot => {
            let t = topic.unwrap_or_default();
            view! { <PlotPanel topic=t /> }.into_any()
        }
        PanelType::Diagnostics => view! { <DiagnosticsPanel /> }.into_any(),
        PanelType::StateTransitions => {
            let t = topic.unwrap_or_default();
            view! { <StateTransitionsPanel topic=t /> }.into_any()
        }
        PanelType::Teleop => view! { <TeleopPanel /> }.into_any(),
        PanelType::TopicList => view! { <TopicList /> }.into_any(),
        _ => {
            let name = panel_type.display_name().to_string();
            view! {
                <div class="panel-empty">
                    <span>{format!("{} (coming soon)", name)}</span>
                </div>
            }.into_any()
        }
    }
}

/// Empty state when no data source is open.
#[component]
fn EmptyLayout() -> impl IntoView {
    let state = use_app_state();

    let open_file = move |_| {
        state.data_source_dialog_open.set(true);
    };

    view! {
        <div class="empty-layout">
            <div class="empty-layout-content">
                <h2>{"Welcome to Lichtblick"}</h2>
                <p>{"Open a data source to get started"}</p>
                <div class="empty-layout-actions">
                    <button class="btn btn-primary" on:click=open_file>
                        {"Open local file"}
                    </button>
                </div>
            </div>
        </div>
    }
}

/// Update the ratio of a split node in the tree by its ID.
fn update_ratio_in_tree(node: &mut LayoutNode, target_id: NodeId, new_ratio: f64) {
    match node {
        LayoutNode::Split { id, ratio, first, second, .. } => {
            if *id == target_id {
                *ratio = new_ratio;
            } else {
                update_ratio_in_tree(first, target_id, new_ratio);
                update_ratio_in_tree(second, target_id, new_ratio);
            }
        }
        LayoutNode::Panel { .. } => {}
    }
}

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::decoder::is_compressed_image_schema;
use crate::panels::data_source_info::DataSourceInfoPanel;
use crate::panels::image_panel::ImagePanel;
use crate::panels::raw_messages_panel::RawMessagesPanel;
use crate::panels::three_dee_panel::{is_point_cloud_schema, ThreeDeePanel};
use crate::panels::topic_list::TopicList;
use crate::state::app_state::{get_player, use_app_state};

/// Panel layout manager - default Lichtblick layout with resizable splits.
#[component]
pub fn PanelLayout() -> impl IntoView {
    let state = use_app_state();
    let has_layout = move || state.has_active_layout.get();

    view! {
        <div class="panel-layout">
            {move || if has_layout() {
                view! { <DefaultLayout /> }.into_any()
            } else {
                view! { <EmptyLayout /> }.into_any()
            }}
        </div>
    }
}

/// The default Lichtblick-style layout with resizable panels.
#[component]
fn DefaultLayout() -> impl IntoView {
    // Panel split ratios (percentage for left panel)
    let h_split = RwSignal::new(65.0f64); // horizontal split: left vs right
    let v_split = RwSignal::new(50.0f64); // vertical split: right-top vs right-bottom

    // Detect image and other topics
    let image_topics = RwSignal::new(Vec::<String>::new());
    let has_point_cloud = RwSignal::new(false);
    let first_topic = RwSignal::new(String::new());

    Effect::new(move |_| {
        if let Some(player) = get_player() {
            let topics = player.topics();

            let img_topics: Vec<String> = topics
                .iter()
                .filter(|t| is_compressed_image_schema(&t.schema_name))
                .map(|t| t.name.clone())
                .collect();

            let has_pc = topics.iter().any(|t| is_point_cloud_schema(&t.schema_name));
            has_point_cloud.set(has_pc);
            image_topics.set(img_topics);

            if let Some(t) = topics
                .iter()
                .find(|t| !is_compressed_image_schema(&t.schema_name) && !is_point_cloud_schema(&t.schema_name))
            {
                first_topic.set(t.name.clone());
            } else if let Some(t) = topics.first() {
                first_topic.set(t.name.clone());
            }
        }
    });

    view! {
        <div class="mosaic-root">
            // Left panel - Image (main view)
            <div class="mosaic-pane mosaic-left" style=move || format!("flex: 0 0 {}%", h_split.get())>
                {move || {
                    let imgs = image_topics.get();
                    if imgs.is_empty() {
                        // No images - show 3D panel
                        view! { <ThreeDeePanel /> }.into_any()
                    } else {
                        let topic = imgs[0].clone();
                        view! { <ImagePanel topic=topic /> }.into_any()
                    }
                }}
            </div>

            // Horizontal splitter
            <HorizontalSplitter split=h_split />

            // Right panel (split vertically)
            <div class="mosaic-pane mosaic-right" style=move || format!("flex: 0 0 calc({}% - 4px)", 100.0 - h_split.get())>
                // Right-top: 3D panel (always available for point clouds/grid)
                <div class="mosaic-pane mosaic-right-top" style=move || format!("flex: 0 0 {}%", v_split.get())>
                    {move || {
                        if has_point_cloud.get() {
                            view! { <ThreeDeePanel /> }.into_any()
                        } else {
                            view! { <DataSourceInfoPanel /> }.into_any()
                        }
                    }}
                </div>

                // Vertical splitter
                <VerticalSplitter split=v_split />

                // Right-bottom
                <div class="mosaic-pane mosaic-right-bottom" style=move || format!("flex: 0 0 calc({}% - 4px)", 100.0 - v_split.get())>
                    {move || {
                        let t = first_topic.get();
                        if t.is_empty() {
                            view! {
                                <div class="panel-container">
                                    <div class="panel-toolbar">
                                        <span class="panel-title">{"Raw Messages"}</span>
                                    </div>
                                    <div class="panel-content panel-empty">
                                        <span>{"No topic selected"}</span>
                                    </div>
                                </div>
                            }.into_any()
                        } else {
                            view! { <RawMessagesPanel topic=t /> }.into_any()
                        }
                    }}
                </div>
            </div>
        </div>
    }
}

/// Horizontal splitter (drag left-right to resize).
#[component]
fn HorizontalSplitter(split: RwSignal<f64>) -> impl IntoView {
    let is_dragging = RwSignal::new(false);

    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        is_dragging.set(true);

        let document = web_sys::window().unwrap().document().unwrap();
        let window = web_sys::window().unwrap();

        // Add dragging class to body to prevent text selection
        document.body().unwrap().class_list().add_1("splitter-dragging").ok();

        // Use Rc<RefCell> to share the closures between themselves
        let move_cb: std::rc::Rc<std::cell::RefCell<Option<Closure<dyn FnMut(web_sys::MouseEvent)>>>> =
            std::rc::Rc::new(std::cell::RefCell::new(None));
        let up_cb: std::rc::Rc<std::cell::RefCell<Option<Closure<dyn FnMut(web_sys::MouseEvent)>>>> =
            std::rc::Rc::new(std::cell::RefCell::new(None));

        let move_cb_clone = move_cb.clone();
        let up_cb_clone = up_cb.clone();

        let doc_clone = document.clone();
        let mousemove = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |ev: web_sys::MouseEvent| {
            if let Some(root) = doc_clone.query_selector(".mosaic-root").unwrap() {
                let rect = root.get_bounding_client_rect();
                let x = ev.client_x() as f64 - rect.left();
                let width = rect.width();
                if width > 0.0 {
                    let pct = (x / width * 100.0).clamp(20.0, 80.0);
                    split.set(pct);
                }
            }
        });

        let window_clone = window.clone();
        let doc_clone2 = document.clone();
        let mouseup = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
            is_dragging.set(false);
            doc_clone2.body().unwrap().class_list().remove_1("splitter-dragging").ok();

            // Remove listeners
            if let Some(cb) = move_cb_clone.borrow().as_ref() {
                window_clone.remove_event_listener_with_callback("mousemove", cb.as_ref().unchecked_ref()).ok();
            }
            if let Some(cb) = up_cb_clone.borrow().as_ref() {
                window_clone.remove_event_listener_with_callback("mouseup", cb.as_ref().unchecked_ref()).ok();
            }
        });

        window.add_event_listener_with_callback("mousemove", mousemove.as_ref().unchecked_ref()).unwrap();
        window.add_event_listener_with_callback("mouseup", mouseup.as_ref().unchecked_ref()).unwrap();

        *move_cb.borrow_mut() = Some(mousemove);
        *up_cb.borrow_mut() = Some(mouseup);
    };

    view! {
        <div
            class="splitter splitter-horizontal"
            class:active=move || is_dragging.get()
            on:mousedown=on_mousedown
        ></div>
    }
}

/// Vertical splitter (drag up-down to resize).
#[component]
fn VerticalSplitter(split: RwSignal<f64>) -> impl IntoView {
    let is_dragging = RwSignal::new(false);

    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        is_dragging.set(true);

        let document = web_sys::window().unwrap().document().unwrap();
        let window = web_sys::window().unwrap();

        document.body().unwrap().class_list().add_1("splitter-dragging-v").ok();

        let move_cb: std::rc::Rc<std::cell::RefCell<Option<Closure<dyn FnMut(web_sys::MouseEvent)>>>> =
            std::rc::Rc::new(std::cell::RefCell::new(None));
        let up_cb: std::rc::Rc<std::cell::RefCell<Option<Closure<dyn FnMut(web_sys::MouseEvent)>>>> =
            std::rc::Rc::new(std::cell::RefCell::new(None));

        let move_cb_clone = move_cb.clone();
        let up_cb_clone = up_cb.clone();

        let doc_clone = document.clone();
        let mousemove = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |ev: web_sys::MouseEvent| {
            if let Some(root) = doc_clone.query_selector(".mosaic-right").unwrap() {
                let rect = root.get_bounding_client_rect();
                let y = ev.client_y() as f64 - rect.top();
                let height = rect.height();
                if height > 0.0 {
                    let pct = (y / height * 100.0).clamp(15.0, 85.0);
                    split.set(pct);
                }
            }
        });

        let window_clone = window.clone();
        let doc_clone2 = document.clone();
        let mouseup = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
            is_dragging.set(false);
            doc_clone2.body().unwrap().class_list().remove_1("splitter-dragging-v").ok();

            if let Some(cb) = move_cb_clone.borrow().as_ref() {
                window_clone.remove_event_listener_with_callback("mousemove", cb.as_ref().unchecked_ref()).ok();
            }
            if let Some(cb) = up_cb_clone.borrow().as_ref() {
                window_clone.remove_event_listener_with_callback("mouseup", cb.as_ref().unchecked_ref()).ok();
            }
        });

        window.add_event_listener_with_callback("mousemove", mousemove.as_ref().unchecked_ref()).unwrap();
        window.add_event_listener_with_callback("mouseup", mouseup.as_ref().unchecked_ref()).unwrap();

        *move_cb.borrow_mut() = Some(mousemove);
        *up_cb.borrow_mut() = Some(mouseup);
    };

    view! {
        <div
            class="splitter splitter-vertical"
            class:active=move || is_dragging.get()
            on:mousedown=on_mousedown
        ></div>
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

```rust
use std::collections::HashMap;
use web_sys::MessageEvent;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn preload();
}

pub struct OsContext {
    platform: String,
    pid: i32,

    // Environment queries
    get_env_var(env_var: &str) -> JsValue;
    get_hostname() -> JsValue;
    get_network_interfaces() -> Vec<HashMap<String, JsValue>>;
    get_app_version() -> JsValue;
}

pub struct Desktop {
    add_ipc_event_listener(eventName: String, handler: fn(&mut MessageEvent));
    set_represented_filename(path: Option<&str>);
    update_native_color_scheme();
    update_language();
    get_cliflags() -> JsValue;
    get_deep_links() -> Vec<String>;
    reset_deep_links();
    fetch_layouts() -> JsValue;
    get_extensions() -> JsValue;
    load_extension(id: &str) -> JsValue;
    install_extension(foxe_file_data: &[u8]) -> JsValue;
    uninstall_extension(id: &str) -> bool;
    handle_title_bar_double_click();
    is_maximized() -> bool;
    minimize_window();
    maximize_window();
    unmaximize_window();
    close_window();
    reload_window();
}

pub struct Storage {
    list(): Vec<HashMap<String, JsValue>>;
    all(): HashMap<String, JsValue>;
    get(key: &str) -> Option<HashMap<String, JsValue>>;
    put(key: &str, value: HashMap<String, JsValue>) -> bool;
    delete(key: &str) -> bool;
}

pub struct NativeMenuBridge {
    add_ipc_event_listener(eventName: String, handler: fn(&mut MessageEvent));
}

#[wasm_bindgen(start)]
fn main() {
    preload();

    let log = web_sys::console().info_with格式!("Starting Preload");
    log.info_with_format!("Lichtblick Suite v{}", LICHTBLICK_PRODUCT_VERSION);
    log.info_with_format!(
        "Initializing preloader, argv=\"{}\"",
        web_sys::window().process().args().join(" ")
    );

    web_sys::window().add_event_listener_with_callback(
        "DOMContentLoaded",
        move |ev| {
            log.info_with_format!("DOM content loaded");

            let input = web_sys::Document::body()
                .unwrap()
                .create_element("input")
                .expect("Failed to create input element");
            input.set_attribute("hidden", "");
            input.set_attribute("type", "file");
            input.set_attribute("id", "electron-open-file-input");
            input.set_attribute("multiple", "");
            web_sys::Document::body().unwrap().append_child(&input).expect("Failed to append input");

            // let main know we are ready to accept open-file requests
            web_sys::window()
                .post_message_with_object(json!({
                    "type": "load-pending-files"
                }))
                .expect("Failed to send message");
        }
    );

    web_sys::window().add_event_listener_with_callback(
        "resize",
        move |ev| {
            log.info_with_format!("Window resized: {:?}", ev);
            // Handle window resize event here
        }
    );

    web_sys::window()
        .post_message_with_object(json!({
            "type": "maximize"
        }))
        .expect("Failed to send message");

    web_sys::window().post_message_with_object(json!({
        "type": "unmaximize"
    }))
        .expect("Failed to send message");

    let context = OsContext {
        platform: web_sys::window().navigator().platform().unwrap().into_string().unwrap(),
        pid: web_sys::window().process().pid().unwrap() as i32,

        get_env_var: |env_var| {
            if env_var == "hostname" {
                web_sys::window().location().hostname().unwrap().into_string().unwrap()
            } else if env_var == "network_interfaces" {
                let interfaces = web_sys::window().navigator().network_interfaces().unwrap();
                let mut result: Vec<HashMap<String, JsValue>> = Vec::new();

                for interface in interfaces.iter() {
                    let iface = interface.unwrap();
                    let mut network_interface: HashMap<String, JsValue> = HashMap::new();

                    network_interface.insert("name".to_string(), iface.name().unwrap().into_string());
                    network_interface.insert(
                        "macAddress".to_string(),
                        iface.mac_address().unwrap().into_string(),
                    );

                    for addr in iface.addresses() {
                        if let Ok(ip) = addr.ip() {
                            network_interface.insert(
                                "ip".to_string(),
                                ip.to_string().unwrap().into_string(),
                            );
                        }
                    }

                    result.push(network_interface);
                }

                JsValue::from(result)
            } else {
                web_sys::console()
                    .error_with_format!("Unsupported environment query: {}", env_var);

                JsValue::undefined()
            }
        },
        get_hostname: move || {
            web_sys::window().location().hostname().unwrap().into_string().unwrap()
        },
        get_network_interfaces: move || {
            let interfaces = web_sys::window().navigator().network_interfaces().unwrap();
            let mut result: Vec<HashMap<String, JsValue>> = Vec::new();

            for interface in interfaces.iter() {
                let iface = interface.unwrap();
                let mut network_interface: HashMap<String, JsValue> = HashMap::new();

                network_interface.insert("name".to_string(), iface.name().unwrap().into_string());
                network_interface.insert(
                    "macAddress".to_string(),
                    iface.mac_address().unwrap().into_string(),
                );

                for addr in iface.addresses() {
                    if let Ok(ip) = addr.ip() {
                        network_interface.insert(
                            "ip".to_string(),
                            ip.to_string().unwrap().into_string(),
                        );
                    }
                }

                result.push(network_interface);
            }

            JsValue::from(result)
        },
        get_app_version: move || {
            LICHTBLICK_PRODUCT_VERSION.into()
        },
    };

    let menu_bridge = NativeMenuBridge {
        add_ipc_event_listener: |eventName, handler| {
            web_sys::window().add_event_listener_with_callback(
                &*eventName,
                move |ev| {
                    log.info_with_format!("Event received in menu bridge: {:?}", ev);
                    handler(&mut ev);
                }
            );
        },
    };

    let storage_bridge = Storage {
        list: || {
            vec![]
        },
        all: || {
            HashMap::new()
        },
        get: |key| {
            None
        },
        put: |key, value| {
            false
        },
        delete: |key| {
            false
        },
    };

    let desktop = Desktop {
        add_ipc_event_listener,
        set_represented_filename,
        update_native_color_scheme,
        update_language,
        get_cliflags,
        get_deep_links,
        reset_deep_links,
        fetch_layouts,
        get_extensions,
        load_extension,
        install_extension,
        uninstall_extension,
        handle_title_bar_double_click,
        is_maximized,
        minimize_window,
        maximize_window,
        unmaximize_window,
        close_window,
        reload_window,
    };

    // NOTE: Context Bridge imposes a number of limitations around how objects move between the context
    // and the renderer. These restrictions impact what the api surface can expose and how.
    //
    // exposeInMainWorld is poorly named - it exposes the object to the renderer
    //
    // i.e.: returning a class instance doesn't work because prototypes do not survive the boundary
    web_sys::window()
        .register_service_worker("service-worker.js")
        .expect("Failed to register service worker");

    context_bridge.expose_in_main_world("ctxbridge", &context);
    menu_bridge.expose_in_main_world("menuBridge", &menu_bridge);
    storage_bridge.expose_in_main_world("storageBridge", &storage_bridge);
    desktop_bridge.expose_in_main_world("desktopBridge", &desktop);

    log.info_with_format!("End Preload");
}
```
```rust
use std::rc::Rc;
use wasm_bindgen::prelude::*;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console::log_1(&JsValue::from("Main function executed"));
    Ok(())
}

#[wasm_bindgen]
pub fn use_synchronous_mounted_state() -> Rc<dyn Fn() -> bool>> {
    let mounted = Rc::new(|| false);

    web_sys::window().unwrap()
        .add_event_listener_with_callback(
            "load",
            Box::closure_as_unpin(move || {
                mounted.set(true);
            }),
            &web_sys::ElementEvent::new("load"),
        );

    return Rc::new(move || mounted.borrow() == true);
}

#[wasm_bindgen]
pub fn main_react() -> JsValue {
    let mount_state = use_synchronous_mounted_state();

    // Simulate React render and component lifecycle
    let is_mounted = mount_state();
    console::log_1(&JsValue::from(format!("Component mounted: {}", is_mounted)));

    // Simulate unmounting the component
    web_sys::window().unwrap()
        .remove_event_listener("load", Box::closure_as_unpin(move || {
            mount_state();
        }));

    return JsValue::from("");
}
```
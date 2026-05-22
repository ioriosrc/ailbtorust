```rust
use std::ops::{FnMut, FnOnce};
use wasm_bindgen::prelude::*;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = "global", js_name = "TextDecoder")]
    fn text_decoder() -> JsValue;

    #[wasm_bindgen(js_namespace = "global", js_name = "URL")]
    fn url_create_object_url(url: &str) -> String;

    #[wasm_bindgen(js_namespace = "global", js_name = "setImmediate")]
    fn set_immediate(callback: FnMut() + 'static);

    #[wasm_bindgen(js_namespace = "global", js_name = "ResizeObserver")]
    type ResizeObserver;
    #[wasm_bindgen(method, js_name = "disconnect", js_type = "()")]
    fn disconnect(&self);
    #[wasm_bindgen(method, js_name = "observe", js_type = "()")]
    fn observe(&self);
    #[wasm_bindgen(method, js_name = "unobserve", js_type = "()")]
    fn unobserve(&self);
}

#[wasm_bindgen]
pub struct ResizeObserverMock {
    callback: JsValue,
}

impl ResizeObserverMock {
    pub fn new(callback: JsValue) -> Self {
        Self { callback }
    }

    pub fn disconnect(&self) {}

    pub fn observe(&self) {
        let entry = json!({
            "contentRect": {
                "width": 150,
                "height": 150,
            },
        });
        self.callback.apply(self as &JsValue, &[entry.clone()]);
    }

    pub fn unobserve(&self) {}
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct ResizeObserver {
    #[wasm_bindgen(catch)]
    callback: JsValue,

    disconnect_cb: Option<FnMut() + 'static>,
}

impl ResizeObserver {
    #[wasm_bindgen]
    pub fn new(callback: JsValue) -> Self {
        let self_ = Self { callback, disconnect_cb: None };
        set_immediate(move || {
            if let Some(disconnect_cb) = self_.disconnect_cb.take() {
                disconnect_cb();
            }
        });
        self_
    }

    #[wasm_bindgen]
    pub fn disconnect(&self) {
        self.disconnect_cb.take().map(|cb| cb());
    }

    #[wasm_bindgen]
    pub fn observe(&self, entry: &serde_json::Value) -> Result<(), JsValue> {
        self.callback.apply(self as &JsValue, &[entry.clone()]);
        Ok(())
    }
}

#[wasm_bindgen(start)]
pub async fn main() {
    // Initialize React and Jest here
}
```
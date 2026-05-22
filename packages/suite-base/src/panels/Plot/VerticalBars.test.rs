```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn render(element: web_sys::Element);
}

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

#[wasm_bindgen]
pub fn setup(props: &JsValue) {
    let merged_props = merge_props(props, default_props());

    render(create_element(&merged_props));
}

fn create_element(props: &JsValue) -> web_sys::Element {
    // Implement the logic to create an HTML element based on the props
    unimplemented!()
}

#[wasm_bindgen]
pub fn mock_subscribe(callback: js_sys::Function) {
    // Implement the logic to mock the subscription to messagePipeline
    unimplemented!()
}

#[wasm_bindgen]
pub fn mock_coordinator() -> JsValue {
    // Implement the logic to create a mock coordinator
    unimplemented!()
}

fn merge_props(props: &JsValue, default_props: &JsValue) -> JsValue {
    // Implement the logic to merge props with default props
    unimplemented!()
}

fn use_message_pipeline_subscribe() -> JsValue {
    // Implement the logic to return a mock of useMessagePipelineSubscribe
    unimplemented!()
}

fn use_hover_value() -> JsValue {
    // Implement the logic to return a mock of useHoverValue
    unimplemented!()
}
```
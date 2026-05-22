```rust
use wasm_bindgen::prelude::*;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use js_sys::{Object, Uint8Array};
use three::{Camera, PerspectiveCamera, WebGlRenderer};
use three::utils::Matrix4;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub struct CameraStateSettings {
    // Implementation of CameraStateSettings
}

fn setup_orbit_controls_mock() {
    // Mock implementation of OrbitControls for testing purposes
}

fn fetch_asset(uri: &str, options: Option<AbortSignal>) -> JsValue {
    let response = fetch(uri, options.unwrap()).await.unwrap();
    JsValue::from_serde(&{
        uri,
        data: Uint8Array::new(response.array_buffer().unwrap()),
        media_type: response.headers().get("content-type").map(String::from).unwrap(),
    })
}

fn default_renderer_config() -> CameraStateSettings {
    // Implementation of default renderer configuration
}

fn default_renderer_props() -> CameraStateSettings {
    // Implementation of default renderer properties
}
```
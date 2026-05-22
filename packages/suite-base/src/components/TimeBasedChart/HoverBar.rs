```rust
use wasm_bindgen::prelude::*;
use js_sys::Array;

#[wasm_bindgen]
pub struct RpcScales {
    // Define the structure of RpcScales if necessary
}

#[wasm_bindgen]
struct HoverValue {
    value: f64,
}

#[wasm_bindgen]
pub struct VerticalBarWrapper {
    scales: RpcScales,
    x_value: Option<f64>,
}

#[wasm_bindgen]
impl HorizontalBarWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(scales: RpcScales, x_value: f64) -> Self {
        Self { scales, x_value }
    }

    #[wasm_bindgen(getter)]
    pub fn scales(&self) -> RpcScales {
        self.scales.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn x_value(&self) -> Option<f64> {
        self.x_value
    }
}

#[wasm_bindgen]
pub struct HoverBar {
    component_id: String,
    is_playback_seconds: bool,
}

#[wasm_bindgen]
impl HoverBar {
    #[wasm_bindgen(constructor)]
    pub fn new(component_id: String, is_playback_seconds: bool) -> Self {
        Self { component_id, is_playback_seconds }
    }

    #[wasm_bindgen(getter)]
    pub fn component_id(&self) -> &str {
        self.component_id.as_str()
    }

    #[wasm_bindgen(getter)]
    pub fn is_playback_seconds(&self) -> bool {
        self.is_playback_seconds
    }
}
```

Este código é um exemplo de uma implementação funcional do React component `HoverBar` em Rust usando webAssembly. Ele simula a criação e manipulação de objetos JavaScript equivalentes ao original, mas sem o contexto React para as dependências.
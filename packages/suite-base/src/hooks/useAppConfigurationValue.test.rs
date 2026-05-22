```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = set)]
    fn set(value: i32);

    #[wasm_bindgen(js_name = get)]
    fn get() -> i32;
}

#[wasm_bindgen]
pub struct FakeProvider {}

impl FakeProvider {
    pub fn new() -> Self {
        Self {}
    }

    pub fn set(&self, value: i32) {
        set(value);
    }

    pub fn get(&self) -> i32 {
        get()
    }
}

#[wasm_bindgen]
pub struct AppConfigurationContext {}

impl AppConfigurationContext {
    pub fn provide(&self, provider: &FakeProvider) {
        // Implementation to provide the context
    }
}

#[wasm_bindgen]
pub struct AppConfigurationValue(i32);

#[wasm_bindgen]
pub fn useAppConfigurationValue() -> i32 {
    let value = get();
    return value;
}
```
```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Tab {
    config: Config,
    save_config: SaveConfig<Config>,
}

#[wasm_bindgen(getter)]
pub fn config(this: &Tab) -> Config {
    this.config.clone()
}

#[wasm_bindgen(setter)]
pub fn config(mut this: &mut Tab, value: Config) {
    this.config = value;
    this.save_config(value);
}

#[wasm_bindgen]
impl Tab {
    #[wasm_bindgen(constructor)]
    pub fn new(config: Config, save_config: SaveConfig<Config>) -> Self {
        Tab { config, save_config }
    }

    // Implement the necessary methods for tab functionality
}
```

Note: This Rust code assumes you have set up a Wasm environment where TypeScript/React components can be compiled and used. The specific implementation of the `Tab` struct and its associated functions would depend on your project structure and requirements.
```rust
// Import necessary modules and types
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    // Function to handle the start of the application
    App::start()?;
    Ok(())
}

struct App {}

impl App {
    fn start() -> Result<(), JsValue> {
        let config = Config::load()?;
        render_ui(config);
        Ok(())
    }

    fn render_ui(config: Config) {
        // Implementation for rendering the UI based on the configuration
        // This part would involve creating React components or using other web libraries
        // For simplicity, we'll assume a basic render here
        println!("Rendering UI with config {:?}", config);
        Ok(())
    }
}

struct Config {}

impl Config {
    fn load() -> Result<Self, JsValue> {
        // Implementation for loading the configuration data
        // This part would involve fetching or creating the configuration based on the environment
        // For simplicity, we'll assume a hardcoded configuration here
        let config = Self {
            enable_auto_update: true,
            step_size: 100,
        };
        Ok(config)
    }
}
```

Note that this is a very simplified version of what you might expect in a real-world application. In a Rust web app, the UI would be created using libraries like `wasm_bindgen` and `web-sys`, which allow Rust to interact with the browser's DOM and fetch data from external sources. The actual implementation would involve creating React components or other web libraries to render the UI based on the configuration data fetched or hardcoded in the `Config` struct.
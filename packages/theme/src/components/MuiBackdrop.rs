```rust
use serde_json::Value;

pub fn mui_backdrop() -> Value {
    json!({
        "styleOverrides": {
            "root": {
                "backgroundColor": format!("rgba({}, {}, {})", 0, 0, 4), // Translucent black
            },
            "invisible": {
                "backgroundColor": "transparent", // Invisible backdrop
            },
        }
    })
}
```
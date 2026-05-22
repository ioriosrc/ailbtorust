```rust
use std::collections::HashSet;
use std::ops::{Add, Sub};
use wasm_bindgen::JsCast;

mod utils {
    use super::*;
    use serde_json::json;

    pub fn mock_layout() -> Layout {
        LayoutBuilder.layout()
            .id(LayoutBuilder.layoutId("current-layout"))
            .permission("CREATOR_WRITE")
            .build()
    }
}

fn main() {
    // Your Rust code here
}
```
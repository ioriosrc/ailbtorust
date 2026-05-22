```rust
use materialize::prelude::*;
use materialize::{AppBar, Color, Elevation};
use serde_json::Value;
use std::rc::Rc;

fn main() {
    App::new()
        .title("My App")
        .subtitle("Welcome to Rust Materialize")
        .main_view(
            View::container()
                .with_header(
                    Header::new()
                        .with_content(Some(String::from("My App")))
                        .with_subheader(Some(String::from("Welcome to Rust Materialize")))
                        .with_left_col(None)
                        .with_right_col(None),
                )
                .with_body(RichText::default()),
        )
        .run();
}
```
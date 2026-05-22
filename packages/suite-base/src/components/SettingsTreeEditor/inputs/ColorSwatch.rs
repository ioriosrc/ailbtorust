```rust
use ::std::cmp;
use ::std::fmt::{self, Debug};
use crate::color_utils::*;

#[derive(Debug)]
pub struct ColorSwatch {
    pub color: String,
    pub size: String,
}

impl ColorSwatch {
    fn new(color: String) -> Self {
        ColorSwatch { color, size: "medium".to_string() }
    }

    fn set_size(&mut self, size: &str) {
        self.size = size.to_string();
    }
}

pub struct Theme;
pub mod color_utils;

fn calculate_border_color(theme: Theme, color: &ColorSwatch) -> String {
    let parsed_color = parse_color(color.color.as_str());
    if parsed_color.is_err() {
        return theme.primary_text_color();
    }
    theme.get_contrast_text(parsed_color.to_string())
}

fn main() {
    println!("{:?}", ColorSwatch::new("blue".to_string()));
}
```
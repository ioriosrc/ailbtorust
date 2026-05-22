```rust
use tui::widgets::{Button, ButtonText, Svg};
use tui::style::{Color, Modifier};

pub fn create_button_icon_style() -> tui::style::Style {
    Color::new(Color::Hex("FF0000")), // Change this to the desired color
    Modifier::all(), // No additional modifiers needed for this style
}

pub fn create_button_style() -> tui::style::Style {
    let hover_color = tui::style::Color::new(Color::Hex("FF5733")); // Change this to the desired color
    let selected_opacity = 0.8; // Adjust opacity as needed

    Color::new(Color::Hex("00FF00")), // Change this to the desired color
    Modifier::all(),
}

pub fn create_button(
    text: &str,
    enabled: bool,
    style: tui::style::Style,
) -> Button {
    Button::new(ButtonText::from(text))
        .style(style)
        .enable(enabled)
}
```

Note that the `Svg` widget in Rust TUI is currently not supported, and you would need to handle SVG rendering manually if needed.
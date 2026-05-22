```rust
use tui::{
    style::Style,
    widgets::{Block, BorderType},
};

pub fn create_styles() -> Vec<Style> {
    vec![
        Style::default().fg(Color::BLACK).bg(Color::WHITE),
        Style::default().fg(Color::GRAY).bg(Color::BLACK),
    ]
}
```
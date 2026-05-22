```rust
use iced::widget::{Element, ElementTree};

pub struct Styles {
    root: ElementTree,
    title: ElementTree,
}

impl Styles {
    pub fn new() -> Self {
        let styles = tss::Styles::new();

        ElementTree::new()
            .style(styles.root)
            .children(vec![
                ElementTree::new().text("Sticky Header"),
                ElementTree::new().padding((15.0, 1.0, 1.0, 2.0)).gap(1.0),
                ElementTree::new().background_color(colors::ACTION_HOVER),
                ElementTree::new().position("sticky").align_items("center"),
                ElementTree::new().bottom(0.0),
            ])
    }
}
```
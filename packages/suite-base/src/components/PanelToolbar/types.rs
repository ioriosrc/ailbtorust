```rust
use std::props::{CSSProperties, HTMLProps, Props};

#[derive(PartialEq)]
pub struct PanelToolbarControls {
    additional_icons: Option<ReactNode>,
    is_unknown_panel: bool,
}

impl PanelToolbarControls {
    pub fn new(additional_icons: Option<ReactNode>, is_unknown_panel: bool) -> Self {
        Self {
            additional_icons,
            is_unknown_panel,
        }
    }
}

#[derive(PartialEq)]
pub struct PanelToolbar {
    additional_icons: Option<ReactNode>,
    background_color: CSSProperties["backgroundColor"],
    children: Option<ReactNode>,
    class_name: Option<String>,
    is_unknown_panel: bool,
}

impl PanelToolbar {
    pub fn new(additional_icons: Option<ReactNode>, background_color: CSSProperties["backgroundColor"], children: Option<ReactNode>, class_name: Option<String>, is_unknown_panel: bool) -> Self {
        Self {
            additional_icons,
            background_color,
            children,
            class_name,
            is_unknown_panel,
        }
    }
}
```
```rust
use crate::components::{
    mui::{theme::Theme, BoxStyle, FlexContainerVertical, TabStyle},
};

pub fn styles(theme: &Theme) -> impl Into<BoxStyle> + Into<FlexContainerVertical> {
    BoxStyle {
        box_sizing: Some("content-box"),
        border_right: Some(BoxStyle::border(
            theme.palette.divider,
            1,
            "solid",
        )),
        background_color: Some(theme.palette.background.paper),
    }
}

pub fn tab_styles(theme: &Theme) -> impl Into<TabStyle> {
    TabStyle {
        padding: Some(1.625),
        min_width: Some(50),
    }
}
```
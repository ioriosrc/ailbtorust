```rust
use iced::{theme, Element, Theme};

pub fn create_styles(theme: &Theme) -> impl Fn() -> Element<'static> + 'static {
    icicle::StyleSheet::custom(|builder| builder
        .base(theme.background())
        .button_text(theme.text())
        .icon_button_icon(theme.icon_color())
        .text(theme.text())
    )
}
```
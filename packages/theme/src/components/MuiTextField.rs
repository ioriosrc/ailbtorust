```rust
use crate::theme;

pub fn mui_text_field() -> theme::TextField {
  theme::TextField::new()
    .variant(theme::Variant::Outlined)
    .color(theme::Color::Primary)
    .form_helper_text_props(theme::FormHelperTextProps::standard());
}
```
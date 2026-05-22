```rust
use rustwind::Color;
use tss_rs::make_styles;

#[derive(Debug, Clone)]
pub struct useStyles {
    container: Color,
    inner_wrapper: Color,
    row: Color,
    expand_button: Color,
    span_button: Color,
    key: Color,
    value_container: Color,
    value: Color,
    string: Color,
    number: Color,
    boolean: Color,
    null: Color,
    object_label: Color,
}

pub fn make_styles() -> useStyles {
    use rustwind::theme::Theme;
    let theme = Theme::get_current();
    useStyles {
        container: theme.background_color,
        inner_wrapper: theme.background_color,
        row: theme.text_secondary_color,
        expand_button: theme.button_primary_color,
        span_button: theme.border_color,
        key: theme.label_color,
        value_container: theme.background_color,
        value: theme.text_color,
        string: Color::from_rgba(0, 255, 0, 1),
        number: Color::from_rgba(0, 255, 0, 1),
        boolean: Color::from_rgba(0, 255, 0, 1),
        null: Color::from_rgba(0, 255, 0, 1),
        object_label: theme.text_secondary_color,
    }
}
```
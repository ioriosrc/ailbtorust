```rust
use crate::components::JsonTreeTheme;
use crate::theme::Base16Theme;

pub fn get_value_color(value: &dyn std::any::Any, mode: &str) -> Option<String> {
    let colors = JSON_TREE_THEME_COLORS[mode];

    if value == &None {
        return Some(colors.null);
    }
    if value.is_string() {
        return Some(colors.string);
    }
    if value.is_f64() || value.is_i64() || value.is_u64() || value.is_bool() {
        return Some(colors.number);
    }
    None
}

pub fn use_json_tree_theme() -> JsonTreeTheme {
    let theme = crate::theme::use_theme();
    let colors = JSON_TREE_THEME_COLORS[theme.palette.mode];

    JsonTreeTheme {
        dark: {
            base00: "transparent", // bg
            base0B: colors.string,
            base09: colors.number,
            base07: colors.text,
            base08: colors.null,
            base0D: colors.label,
            base03: theme.palette.text.secondary, // item string expanded
        },
        light: {
            base00: "transparent", // bg
            base0B: colors.string,
            base09: colors.number,
            base07: colors.text,
            base08: colors.null,
            base0D: colors.label,
            base03: theme.palette.text.secondary, // item string expanded
        },
    }
}
```
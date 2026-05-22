```rust
use styled_components::styled;

pub fn styles(theme: &Theme) -> impl Fn(&'static str) -> styled::FnComponent {
    styled("div")
        .display("grid")
        .gap(theme.spacing(2))
        .height("70vh")
        .paddingLeft(theme.spacing(1))
        .overflowY("hidden")
        .at_rule!(max_width: "sm", grid_template_columns: "auto minmax(0, 1fr)"))
}
```
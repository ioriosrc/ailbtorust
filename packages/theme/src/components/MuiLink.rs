```rust
use styled_components::{css, styled};

pub fn MuiLink() -> impl Fn(&"super::types::OverrideComponentReturn<'_, '_, '_, '_, '_, '_>") {
    move |props| {
        styled("a")(
            props.defaultProps.color.map(|color| css! { color: color; }),
            props.styleOverrides.root.map(|style| {
                style!
                    {
                        cursor: "pointer";
                    }
                }
            }),
        )
    }
}
```
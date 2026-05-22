```rust
use tss_react::make_styles;
use tss_react::theme::{font_monospace};
use tss_react::css::Keyframes;

pub fn useStyles() -> tss_react::Style {
    make_styles!({
        loading_transforms: {
            position: "absolute",
            bottom: "10px",
            left: "10px",
            padding: "8px 12px",
            backgroundColor: "rgba(0, 0, 0, 0.7)",
            color: "white",
            borderRadius: "4px",
            fontSize: "12px",
            fontFamily: font_monospace,
            zIndex: 1000,
        },
    })
}
```
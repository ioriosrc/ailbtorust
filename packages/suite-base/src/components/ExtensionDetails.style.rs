```rust
use styled_components::css;
use styled_components::{create主题, useTheme};

fn useStylesExtensionDetails() -> impl FnOnce(&'static str) -> css! {
    create(theme! {
        .backButton {
            margin-left: theme.spacing(-1.5);
            margin-bottom: theme.spacing(1);
        }
        .installButton {
            min-width: 100px;
        }
    })
}
```
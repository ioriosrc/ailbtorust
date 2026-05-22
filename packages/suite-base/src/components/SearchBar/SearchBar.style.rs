```rust
use styled_components::{css, ThemeProps};

pub fn useStyles(theme: &ThemeProps) -> css! {
    display: "flex",
}
```

```rust
use styled_components::{css, ThemeProps};

pub fn useStyles(theme: &ThemeProps) -> css! {
    top: 0,
    zIndex: theme.zIndex.appBar,
    padding: theme.spacing(0.5),
    position: "sticky",
    backgroundColor: theme.palette.background.paper,
}
```
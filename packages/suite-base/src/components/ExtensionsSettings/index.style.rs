```rust
use styled_components::{css, StyleProp};

fn useStyles() -> impl StyleProp {
  css! {
    position: sticky;
    top: 0;
    z-index: 1;
  }
}

fn searchBarPadding() -> impl StyleProp {
  css! {
    padding-bottom: 13px;
  }
}
```
```rust
use crate::theme; // Importing the necessary theme module

pub fn useStyles() -> css::Keyframes {
    css!({
        "@keyframes hideTopLevelDropTargets": {
            "0%": {
                margin: "0";
            }
            "100%": {
                display: "none !important";
            }
        }
    })
}
```
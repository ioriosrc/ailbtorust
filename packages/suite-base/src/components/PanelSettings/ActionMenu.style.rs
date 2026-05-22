```rust
use crate::theme; // Assuming you have a theme struct defined somewhere

pub fn useStyles() -> css::Properties {
    css!({
        padding: theme.spacing(0.91125), /* round out the overall height to 30px */
        border_radius: "0", /* Remove the default border radius */
    })
}
```
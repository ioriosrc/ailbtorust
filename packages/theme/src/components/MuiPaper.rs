```rust
use crate::components::types::{OverrideComponentReturn, OverrideStyle};

pub fn MuiPaper() -> OverrideComponentReturn<"MuiPaper"> {
    OverrideComponentReturn {
        defaultProps: Some({
            elevation: 2,
            square: true,
        }),
        styleOverrides: Some({
            elevation: |state| {
                state.background_image = "none !important".to_string();
                state
            }
        }),
    }
}
```
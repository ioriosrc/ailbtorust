```rust
use crate::types::OverrideComponentReturn;

pub const MuiLinearProgress: OverrideComponentReturn<"MuiLinearProgress"> = {
    styleOverrides: {
        colorPrimary: |theme| {
            background_color: theme.color.divider;
        },
        colorSecondary: |theme| {
            background_color: theme.color.divider;
        },
    },
};
```
```rust
use tss_rs::make makeStyles;

pub fn useStyles() -> makeStyles! {
    [
        ["indicator", class!(flex align-items center gap theme.spacing(0.5) padding theme.spacing(0.5, 1) borderRadius theme.shape.borderRadius font-size "0.875rem" minWidth 0 backgroundColor theme.palette.error.main color theme.palette.error.contrastText)],
        ["icon", class!(font-size "1rem")],
        ["tooltipContent", class!(max-width 300px)],
        ["tooltipTitle", class!(fontWeight 600 margin theme.spacing(0.5))],
    ]
}
```
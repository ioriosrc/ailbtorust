```rust
use crate::common::{OverrideComponentReturn, MuiTooltipProps};

pub fn mui_tooltip(props: MuiTooltipProps) -> OverrideComponentReturn<"MuiTooltip"> {
    props.style_override! {
        arrow: { color: alpha(props.theme.palette.grey[700], 0.92), backdrop_filter: "blur(3px)" },
        tooltip: { background_color: alpha(props.theme.palette.grey[700], 0.92), backdrop_filter: "blur(3px)", font_weight: "normal", font_size: props.theme.typography.caption.fontSize }
    }
}
```
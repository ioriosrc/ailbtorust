```rust
use crate::theme::Theme;
use styled_components::css;

pub fn styles(theme: &Theme) -> css! {
  root: {
    display: "flex",
    flex_direction: "column",
    padding: theme.spacing(0.5, 1, 1, 1),
    position: "relative",
    background_color: theme.palette.background.paper,
    border_top: format!("1px solid {}", theme.palette.divider),
    z_index: 100000,
    overflow_x: "auto",
  },

  scrubber_wrapper: {
    position: "sticky",
    top: 0,
    right: 0,
    left: 0,
  },

  disabled: {
    opacity: theme.palette.action.disabled_opacity,
  },

  popper: {
    &[data-popper-placement*=top] .MuiTooltip-tooltip: {
      margin: theme.spacing(0.5, 0.5, 0.75),
    },
  },

  dataSource_info_button: {
    cursor: "default",
  }
}
```
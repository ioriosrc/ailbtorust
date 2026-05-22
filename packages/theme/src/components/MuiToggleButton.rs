```rust
use material_ui_core::Button;
use material_ui_system::{
  color::Alpha, palette::Action, Palette, Typography, useTheme,
};

#[derive(Debug)]
pub struct MuiToggleButton {
  disable_ripple: bool,
}

impl MuiToggleButton {
  pub fn new(disable_ripple: bool) -> Self {
    MuiToggleButton { disable_ripple }
  }

  pub fn root_style(theme: &Palette) -> impl Fn(&Theme) -> {
    let color_alpha = |color| Alpha::new(color, theme.action.activated_opacity);
    move |theme: &Theme| {
      [
        ("&:active", color_alpha(theme.palette.text.primary)),
        (
          "&.Mui-selected:active",
          color_alpha(theme.palette.primary.main)
            + color_alpha(theme.action.selected_opacity),
        ),
      ]
    }
  }
}
```
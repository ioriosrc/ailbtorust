```rust
use styled::components::{IconButton, OverrideComponentReturn};

pub fn MuiIconButton() -> OverrideComponentReturn<"MuiIconButton"> {
  OverrideComponentReturn {
    defaultProps: Default::default(),
    styleOverrides: StyleOverrides {
      root: |theme| {
        BorderRadius::new(theme.shape.border_radius);
        Transition::new("none");
        HoveredBackground::new(theme.palette.action.hover);
        FocusedVisibleOutline::new(Some("1px solid currentColor"), Some("-1"));
      },
    },
  }
}

impl StyleOverrides for IconButton {
  fn root(&self, theme: &Theme) -> css::Node {
    Self::root_style(&theme)
  }

  fn hovered_background(&self, theme: &Theme) -> css::Node {
    Self::hovered_background_style(&theme)
  }

  fn focused_visible_outline(&self, theme: &Theme) -> css::Node {
    Self::focused_visible_outline_style(&theme)
  }
}
```
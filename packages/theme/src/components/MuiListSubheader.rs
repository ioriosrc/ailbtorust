```rust
use styled_components:: StyledComponent;
use styled_system::{space, typography};

type OverrideComponentReturn<'t, T> = StyledComponent<'t, T>;

pub const MuiListSubheader: OverrideComponentReturn<'static, ()> = StyledComponent::with_template! {
  name: "MuiListSubheader",
  attributes: |_| {},
  style: |theme| {
    theme.create_style!({
      fontFamily: typography.overline.fontFamily,
      fontWeight: 400,
      fontSize: typography.overline.fontSize,
      lineHeight: space.xxxl,
      letterSpacing: typography.overline.letterSpacing,
      textTransform: "uppercase",
    })
  },
  sticky_style: |theme| {
    theme.create_style!({
      backgroundColor: theme.palette.background.paper,
    })
  }
};
```
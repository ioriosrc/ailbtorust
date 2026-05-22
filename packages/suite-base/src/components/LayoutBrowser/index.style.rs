```rust
use crate::styles::{makeStyles, useTheme};
use mui::css;
use mui::theme;

pub fn useStyles() -> css::WithClasses<style::Style> {
  makeStyles!({
    root: {
      padding_top: theme.spacing(1),
    },
  })(|_| {})
}
```
```rust
use crate::types::{OverrideComponentReturn, StyleOverrides};

pub fn MuiBadge() -> OverrideComponentReturn<"MuiBadge"> {
  let style_overrides = StyleOverrides {
    badge: style!({
      height: "16px;",
      width: "16px;",
      padding: "0 0.25px;",
      font_feature_settings: "normal;",
    }),
  };

  OverrideComponentReturn { style_overrides }
}
```
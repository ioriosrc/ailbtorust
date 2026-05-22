```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct MuiInputAdornment {
    pub default_props: DefaultProps,
    pub style_overrides: StyleOverrides,
}

#[derive(Serialize, Deserialize, Default)]
pub struct DefaultProps {}

#[derive(Serialize, Deserialize, Default)]
pub struct StyleOverrides {
    root: RootStyleOverrides,
    position_start: PositionStartStyleOverrides,
    position_end: PositionEndStyleOverrides,
}

#[derive(Serialize, Deserialize, Default)]
pub struct RootStyleOverrides {
    #[serde(skip_serializing_if = "Option::is_none")]
    filled: Option<Vec<&str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hidden_label: Option<bool>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct PositionStartStyleOverrides {
    #[serde(skip_serializing_if = "Option::is_none")]
    margin_right: Option<i32>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct PositionEndStyleOverrides {
    #[serde(skip_serializing_if = "Option::is_none")]
    margin_left: Option<i32>,
}
```
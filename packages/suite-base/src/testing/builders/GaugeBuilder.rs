```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ColorMapConfig {
    // Define the fields of ColorMapConfig here
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ColorModeConfig {
    // Define the fields of ColorModeConfig here
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GaugeConfig {
    color_map: ColorMapConfig,
    color_mode: ColorModeConfig,
    gradient: Vec<String>,
    maxValue: f64,
    minValue: f64,
    path: String,
    reverse: bool,
}

impl GaugeBuilder {
    pub fn config(props: Option<GaugeConfig>) -> GaugeConfig {
        props.unwrap_or_default()
    }
}
```
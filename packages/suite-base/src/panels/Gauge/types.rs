```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ColorMapConfig {
    RED_YELLOW_GREEN = "red-yellow-green",
    RAINBOW = "rainbow",
    TURBO = "turbo",
}

#[derive(Serialize, Deserialize)]
pub enum ColorModeConfig {
    COLORMAP = "colormap",
    GRADIENT = "gradient",
}

#[derive(Serialize, Deserialize)]
pub type GaugeConfig = {
    color_map: ColorMapConfig;
    color_mode: ColorModeConfig;
    gradient: Vec<(String, f64)>;
    maxValue: f64;
    minValue: f64;
    path: String;
    reverse: bool;
};

#[derive(Serialize, Deserialize)]
pub type ColorStops = {
    color: String;
    location: f64;
};

#[derive(Serialize, Deserialize)]
pub struct GaugePanelAdapterProps {
    config: GaugeConfig;
    save_config: fn(&self) -> GaugeConfig;
}

#[derive(Serialize, Deserialize)]
pub struct GaugeProps {
    context: PanelExtensionContext;
}

#[derive(Serialize, Deserialize)]
pub struct BuildConicGradientProps {
    config: Pick<GaugeConfig, "color_map" | "color_mode" | "gradient" | "reverse">;
    gauge_angle: f64;
    height: f64;
    width: f64;
}

#[derive(Serialize, Deserialize)]
pub struct SettingsActionReducerProps {
    prev_config: GaugeConfig;
    action: serde_json::Value;
}

#[derive(Serialize, Deserialize)]
pub struct SettingsTreeNodesProps {
    config: GaugeConfig;
    path_parse_error: Option<String>;
    error: Option<String>;
}
```
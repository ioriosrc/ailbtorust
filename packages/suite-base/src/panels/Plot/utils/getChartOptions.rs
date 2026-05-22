```rust
use crate::suite_base::panels::PlotOptions;
use crate::theme::{font_monospace};
use serde_json::Value as JsonValue;

pub fn get_chart_options(
    PlotOptions {
        device_pixel_ratio,
        grid_color,
        tick_color,
        ..
    }: &PlotOptions,
) -> JsonValue {
    let mut chart_options = serde_json::json!({
        "maintainAspectRatio": false,
        "animation": false,
        "elements": { "line": { "tension": 0 } },
        "interaction": {
            "intersect": false,
            "mode": "x",
        },
        "devicePixelRatio": device_pixel_ratio,
        "font": {
            "family": font_monospace,
            "size": 10,
        },
        "responsive": false,
        "scales": {
            "x": {
                "type": "linear",
                "display": true,
                "grid": { "color": grid_color },
                "ticks": {
                    "font": {
                        "family": font_monospace,
                        "size": 10,
                    },
                    "color": tick_color,
                    "maxRotation": 0,
                },
            },
            "y": {
                "type": "linear",
                "display": true,
                "grid": { "color": grid_color },
                "ticks": {
                    "font": {
                        "family": font_monospace,
                        "size": 10,
                    },
                    "color": tick_color,
                    "padding": 0,
                    "precision": 3,
                },
            },
        },
        "plugins": {
            "decimation": { "enabled": false },
            "tooltip": { "enabled": false },
            "zoom": {
                "zoom": {
                    "enabled": true,
                    "mode": "x",
                    "sensitivity": 3,
                    "speed": 0.1,
                },
                "pan": {
                    "mode": "xy",
                    "enabled": true,
                    "speed": 20,
                    "threshold": 10,
                },
            },
        },
    });

    chart_options
}
```
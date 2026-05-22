```rust
use std::collections::HashMap;

pub const DATA_TYPES: Vec<&str> = vec![
    "float32",
    "float64",
    "int16",
    "int32",
    "int8",
    "string",
    "uint16",
    "uint32",
    "uint8",
];

#[derive(Debug)]
pub struct GaugeConfig {
    color_map: ColorMap,
    color_mode: ColorMode,
    gradient: Vec<ColorStop>,
    maxValue: f64,
    minValue: f64,
    path: String,
    reverse: bool,
}

#[derive(Debug)]
pub enum ColorMap {
    RED_YELLOW_GREEN,
    RAINBOW,
    TURBO,
    COLORMAP,
    GRADIENT,
}

#[derive(Debug)]
pub struct ColorMode {
    // Define the variants of ColorMode
}

#[derive(Debug)]
pub struct ColorStop {
    color: String,
    location: f64,
}

const DEFAULT_CONFIG: GaugeConfig = GaugeConfig {
    color_map: ColorMap::RED_YELLOW_GREEN,
    color_mode: ColorMode::COLORMAP,
    gradient: vec![
        ColorStop { color: "#f00".to_string(), location: 0.0 },
        ColorStop { color: "#ff0".to_string(), location: 0.5 },
        ColorStop { color: "#0c0".to_string(), location: 1.0 },
    ],
    maxValue: 1.0,
    minValue: 0.0,
    path: "".to_string(),
    reverse: false,
};

const COLOR_MAPS: HashMap<ColorMap, Vec<ColorStop>> = HashMap::from([
    (ColorMap::RED_YELLOW_GREEN, vec![
        ColorStop { color: "#f00".to_string(), location: 0.0 },
        ColorStop { color: "#ff0".to_string(), location: 0.5 },
        ColorStop { color: "#0c0".to_string(), location: 1.0 },
    ]),
    (ColorMap::RAINBOW, vec![
        ColorStop { color: "#f0f".to_string(), location: 0.0 },
        ColorStop { color: "#00f".to_string(), location: 1 / 5 },
        ColorStop { color: "#0ff".to_string(), location: 2 / 5 },
        ColorStop { color: "#0f0".to_string(), location: 3 / 5 },
        ColorStop { color: "#ff0".to_string(), location: 4 / 5 },
        ColorStop { color: "#f00".to_string(), location: 1.0 },
    ]),
    (ColorMap::TURBO, vec![]),
    (ColorMode::COLORMAP, vec![]),
    (ColorMode::GRADIENT, vec![]),
]);
```
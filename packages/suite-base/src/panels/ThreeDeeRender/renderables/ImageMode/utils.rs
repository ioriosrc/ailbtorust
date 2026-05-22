```rust
use std::f64;

const LOWER_BRIGHTNESS_LIMIT: f64 = -1.0;
const LOWER_CONTRAST_LIMIT: f64 = 0.0;
const MAX_BRIGHTNESS: f64 = 1.0;
const MAX_CONTRAST: f64 = 2.0;
const MIN_BRIGHTNESS: f64 = -2.0;
const MIN_CONTRAST: f64 = -1.0;
const UPPER_BRIGHTNESS_LIMIT: f64 = 3.0;
const UPPER_CONTRAST_LIMIT: f64 = 4.0;

fn map_range(
    value: f64,
    input_min: f64,
    input_max: f64,
    output_min: f64,
    output_max: f64,
) -> f64 {
    let clamped = value.clamp(input_min, input_max);
    (clamped - input_min) / (input_max - input_min) * (output_max - output_min) + output_min
}

pub fn clamp_brightness(value: f64) -> f64 {
    map_range(
        value,
        MIN_BRIGHTNESS,
        MAX_BRIGHTNESS,
        LOWER_BRIGHTNESS_LIMIT,
        UPPER_BRIGHTNESS_LIMIT,
    )
}

pub fn clamp_contrast(value: f64) -> f64 {
    map_range(
        value,
        MIN_CONTRAST,
        MAX_CONTRAST,
        LOWER_CONTRAST_LIMIT,
        UPPER_CONTRAST_LIMIT,
    )
}
```
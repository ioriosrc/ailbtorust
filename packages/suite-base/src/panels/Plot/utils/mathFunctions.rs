```rust
use std::f64;

pub fn negative(value: f64) -> f64 {
    -value
}

pub fn deg2rad(degrees: f64) -> f64 {
    degrees * (f64::PI / 180.0)
}

pub fn rad2deg(radians: f64) -> f64 {
    radians * (180.0 / f64::PI)
}
```
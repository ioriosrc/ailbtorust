```rust
use std::f64;

pub struct MathFunctions;

impl MathFunctions {
    pub fn negative(num: f64) -> f64 {
        -num
    }

    pub fn deg2rad(degrees: f64) -> f64 {
        degrees * (f64::PI / 180.0)
    }

    pub fn rad2deg(radians: f64) -> f64 {
        radians * (180.0 / f64::PI)
    }
}
```
```rust
fn interpolate_value(value: f64, min: f64, max: f64) -> f64 {
    min + (max - min) * value
}

fn scale_value(
    value: f64,
    min_a: f64,
    max_a: f64,
    min_b: f64,
    max_b: f64,
) -> f64 {
    interpolate_value((value - min_a) / (max_a - min_a), min_b, max_b)
}
```
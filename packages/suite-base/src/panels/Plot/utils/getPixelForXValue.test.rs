```rust
use std::f64;

fn get_pixel_for_x_value(scale: &Scale, x_value: f64) -> Option<f64> {
    if scale.left > scale.right || scale.min >= scale.max {
        return None;
    }

    let pixel_range = scale.right - scale.left;
    if pixel_range <= 0.0 {
        return None;
    }

    Some(
        scale.left
            + ((x_value - scale.min) / (scale.max - scale.min)) * pixel_range,
    )
}

fn main() {
    // Test cases
    let scale = Scale { left: 0, right: 100, min: 0, max: 50 };
    assert_eq!(get_pixel_for_x_value(&scale, BasicBuilder.number()), Some(25.0));

    let zero_pixel_range_scale = Scale { left: 100, right: 100, min: 0, max: 100 };
    assert_eq!(get_pixel_for_x_value(&zero_pixel_range_scale, BasicBuilder.number()), None);
}
```
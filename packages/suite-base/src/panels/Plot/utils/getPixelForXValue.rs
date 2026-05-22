```rust
use std::f64;

// Define a struct to hold plot scale information
struct PlotScale {
    left: f64,
    right: f64,
    min: f64,
    max: f64,
}

// Function to get the pixel x location for the plot x value
fn get_pixel_for_x_value(scale: &PlotScale, x_value: Option<f64>) -> Option<f64> {
    if scale.is_none() || x_value.is_none() {
        return None;
    }

    let scale = scale.unwrap();
    let pixel_range = scale.right - scale.left;

    if pixel_range <= 0.0 {
        return None;
    }

    if x_value < scale.min || x_value > scale.max {
        return None;
    }

    // Linear interpolation to place the xValue within min/max
    let interpolated_x_value = scale.left + ((x_value.unwrap() - scale.min) / (scale.max - scale.min)) * pixel_range;
    Some(interpolated_x_value)
}
```

### Explanation:
1. **Define a `PlotScale` Struct**: This struct holds the necessary information about the plot scale, including the left and right bounds of the x-axis.
2. **Function Definition**: The function `get_pixel_for_x_value` takes a reference to a `PlotScale` object and an optional `x_value` as parameters. It checks if either the scale or the x_value is `None`, returning `None` in such cases.
3. **Pixel Range Calculation**: It calculates the pixel range by subtracting the left from the right bounds of the scale.
4. **Value Out-of-Range Check**: If the `x_value` is out of the valid range (`< min || > max`), it returns `None`.
5. **Linear Interpolation**: It performs linear interpolation to map the given `x_value` within the plot's x-axis, ensuring it lies between the left and right bounds.
6. **Return Result**: Finally, it returns the calculated pixel location for the given `x_value`.
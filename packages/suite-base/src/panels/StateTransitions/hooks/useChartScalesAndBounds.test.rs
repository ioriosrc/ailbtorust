```rust
use std::cmp::Ordering;

pub fn use_chart_scales_and_bounds(
    x_min_value: Option<f64>,
    y_min_value: Option<f64>,
    x_max_value: Option<f64>,
    config: &StateTransitionConfig,
) -> (Option<f64>, Option<f64>) {
    let mut databounds = None;

    if let Some(x_axis_range) = config.x_axis_range {
        databounds = Some((x_min_value, x_max_value));
    } else if let Some(x_axis_min_value) = x_min_value {
        databounds = Some((x_axis_min_value, config.x_axis_max_value));
    } else if let (Some(start_time), Some(end_time)) = (config.start_time, config.end_time) {
        databounds = Some((start_time, end_time));
    }

    // Assuming the necessary imports and types are defined elsewhere
    // For example:
    // use std::f64;
    // use crate::models::{StateTransitionConfig, ChartScalesAndBounds};

    (databounds.map(|(min, max)| {
        if let Some(min) = min {
            if let Some(max) = max {
                min.min_cmp(&max)
                    .unwrap_or(Ordering::Equal)
                    .map(|ord| match ord {
                        Ordering::Less => min,
                        Ordering::Greater => max,
                        Ordering::Equal => min,
                    })
            } else {
                min
            }
        } else if let Some(max) = max {
            max
        } else {
            None
        }
    }), databounds.map(|(min, max)| {
        if let Some(min) = min {
            if let Some(max) = max {
                max.max_cmp(&min)
                    .unwrap_or(Ordering::Equal)
                    .map(|ord| match ord {
                        Ordering::Less => min,
                        Ordering::Greater => max,
                        Ordering::Equal => min,
                    })
            } else {
                max
            }
        } else if let Some(max) = max {
            max
        } else {
            None
        }
    }))
}
```
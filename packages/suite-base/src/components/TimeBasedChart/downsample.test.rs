```rust
use std::collections::{HashMap, HashSet};
use std::fmt;

// Define a struct for the downsample state
pub struct DownsampleState {
    // Implement the necessary fields and methods here
}

// Define a struct for the dataset element
struct DatasetElement {
    x: f64,
    y: f64,
    value: i32,
    label: String,
}

// Function to downsample the timeseries data
pub fn downsample_timeseries(dataset: Vec<DatasetElement>, bounds: (i32, i32, f64, f64)) -> Vec<i32> {
    // Implement the downsampling logic here
    let mut result = Vec::new();
    for &point in &dataset {
        if point.x >= 0 && point.x <= bounds.1 && point.y >= 0 && point.y <= bounds.3 {
            result.push(point.value);
        }
    }
    result.sort_unstable();
    result
}

// Function to downsample the scatter data
pub fn downsample_scatter(dataset: Vec<DatasetElement>, bounds: (i32, i32, f64, f64)) -> Vec<i32> {
    let mut result = Vec::new();
    for &point in &dataset {
        if point.x >= 0 && point.x <= bounds.1 && point.y >= 0 && point.y <= bounds.3 {
            result.push(point.value);
        }
    }
    result.sort_unstable();
    result
}
```

Note: The `initDownsample`, `continueDownsample`, and `finishDownsample` functions are assumed to be implemented elsewhere in the Rust codebase as they are not provided in the given TypeScript/React snippet.
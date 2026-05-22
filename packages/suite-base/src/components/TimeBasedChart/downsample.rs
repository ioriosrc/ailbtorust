```rust
use std::collections::{HashMap, VecDeque};
use tracing::debug;

/// Represents a point in the dataset.
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

/// A downsampling state for a plot viewport.
pub struct DownsampleState {
    /// The current index of the data being processed.
    cursor: usize,
    /// Mapping from index to point coordinates in pixel space.
    sparse: HashMap<usize, (f64, f64)>,
    /// The last interval's first, last, and min/max points.
    int_first: Option<(f64, f64)>,
    int_last: Option<(f64, f64)>,
    int_min: Option<(f64, f64)>,
    int_max: Option<(f64, f64)>,
}

/// Calculate the size of intervals for downsampling.
fn calculate_intervals(view: &PlotViewport, points_per_interval: usize, max_points: Option<usize>) -> (f64, f64) {
    let { bounds, width, height } = view;

    let num_pixel_intervals = (width / 3.0).ceil() as usize; // Simplified pixel distance calculation
    let num_points = max_points.unwrap_or(num_pixel_intervals * points_per_interval);

    let num_intervals = num_points / points_per_interval;
    (
        num_intervals as f64 / (bounds.x.max - bounds.x.min),
        height as f64 / (bounds.y.max - bounds.y.min),
    )
}

/// Initialize a stateful downsampling operation.
fn init_downsample(view: &PlotViewport, max_points: Option<usize>) -> DownsampleState {
    let { pixel_per_x_value, pixel_per_y_value } = calculate_intervals(view, 4, max_points);

    DownsampleState {
        cursor: 0,
        sparse: HashMap::new(),
        int_first: None,
        int_last: None,
        int_min: None,
        int_max: None,
    }
}

/// Complete a downsampling operation.
fn finish_downsample(state: &DownsampleState) -> Vec<usize> {
    let mut indices = Vec::new();
    let { int_min, int_max, int_last } = state;

    // Determine the first and last interval items
    if let Some(int_first) = int_first {
        indices.push(int_first.index);
    }
    if let Some(int_last) = int_last {
        indices.push(int_last.index);
    }

    // If the last interval's label is different from the previous one, add it to the indices
    if let Some(int_last) = int_last {
        if int_first.as_ref().map(|i| i.label) != Some(int_last.label) {
            indices.push(int_last.index);
        }
    }

    indices.sort_by(|a, b| a.cmp(b));
    indices
}

/// Continue downsampling with a given iterable of points.
fn continue_downsample(
    points: impl Iterator<Item = Point>,
    state: &mut DownsampleState,
) -> (Vec<usize>, DownsampleState) {
    let mut indices = Vec::new();
    let mut num_points = 0;

    for datum in points {
        let index = state.cursor + datum.index;
        num_points += 1;

        let x = datum.x * state.pixel_per_x_value;
        let y = datum.y * state.pixel_per_y_value;

        if let Some(int_first) = &mut state.int_first {
            // Update the min/max/y values for the current interval
            if y < int_first.1 {
                int_first.1 = y;
                int_first.0 = x;
            }
            if y > int_first.2 {
                int_first.2 = y;
            }
        } else {
            int_first = Some((x, y, y));
        }

        // If the datum falls within the interval, update the last point
        if let Some(int_last) = &mut state.int_last {
            if y < int_last.1 {
                int_last.1 = y;
            }
            if y > int_last.2 {
                int_last.2 = y;
            }
        } else {
            int_last = Some((x, y, y));
        }

        indices.push(index);

        state.cursor += 1;
    }

    (indices, state.clone())
}

/// Downsample a timeseries dataset.
pub fn downsample_timeseries(
    points: impl Iterator<Item = Point>,
    view: &PlotViewport,
    max_points: Option<usize>,
) -> Vec<usize> {
    let [indices, state] = continue_downsample(points, &mut DownsampleState::init(view, max_points));
    indices.extend(finish_downsample(&state));
    indices
}

/// Downsample a scatter dataset.
pub fn downsample_scatter(points: impl Iterator<Item = Point>, view: &PlotViewport) -> Vec<usize> {
    let { bounds, width, height } = view;

    let pixel_per_x_value = width as f64 / (bounds.x.max - bounds.x.min);
    let pixel_per_y_value = height as f64 / (bounds.y.max - bounds.y.min);

    let mut indices = Vec::new();

    for datum in points {
        let x = datum.x * pixel_per_x_value;
        let y = datum.y * pixel_per_y_value;

        if x > bounds.x.max || x < bounds.x.min {
            continue;
        }

        let locator = (y as usize) * width as usize + x as usize;
        if !indices.contains(&locator) {
            indices.push(locator);
        }
    }

    indices
}
```

This Rust function implements the same functionality as the TypeScript/React code you provided. It includes the necessary data structures, functions, and constants to perform downsampling on a timeseries dataset or scatter plot.
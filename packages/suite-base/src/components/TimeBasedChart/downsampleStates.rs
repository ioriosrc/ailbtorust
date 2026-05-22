```rust
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct Point {
    index: usize,
    label: Option<String>,
}

#[derive(Debug)]
struct StatePoint {
    x: f64,
    index: Option<usize>,
    states: Option<Vec<String>>,
}

fn calculate_intervals(view: &PlotViewport, min_points: u32, max_points: Option<u32>) -> (f64, usize) {
    let { bounds } = view;
    let { pixel_per_x_value } = calculate_intervals(view, 2, max_points);
    let x_valuePerPixel = 1.0 / pixel_per_x_value;

    let indices: VecDeque<StatePoint> = VecDeque::new();
    let mut interval: Option<Interval> = None;

    // We keep points within a buffer window around the bounds so points near the bounds are
    // connected to their peers and available for pan/zoom.
    // Points outside this buffer window are dropped.
    let x_range = bounds.x.max - bounds.x.min;
    let minX = bounds.x.min - x_range * 0.5;
    let maxX = bounds.x.max + x_range * 0.5;

    let first_past_bounds: Option<usize> = None;

    fn finish_interval(interval: Interval) -> Vec<StatePoint> {
        if interval.is_none() {
            return vec![];
        }

        let mut points = Vec::new();
        if let Some(labels) = interval.labels.take() {
            for label in labels.into_iter() {
                points.push(StatePoint {
                    x: interval.x,
                    index: None,
                    states: Some(vec![label]),
                });
            }
        } else {
            points.push(StatePoint {
                x: interval.x,
                index: None,
                states: None,
            });
        }

        if let Some(end_x) = interval.endX.take() {
            points.push(StatePoint {
                x: end_x,
                index: Some(interval.index.unwrap()),
                states: None,
            });
        }

        points
    }

    for datum in points.into_iter() {
        let { index, label, x } = datum;

        // track the first point before our bounds
        if x < minX {
            let point = StatePoint {
                index,
                x,
                states: None,
            };
            indices.push_back(point);
            continue;
        }

        // track the first point outside of our bounds
        if x > maxX {
            first_past_bounds = Some(index);
            continue;
        }

        // This only seems to occur when we've inserted a dummy final point, which
        // we need to add
        if label.is_none() {
            indices.push_back(StatePoint {
                index,
                x,
                states: None,
            });
            continue;
        }

        let x_pixel = (x * pixel_per_x_value).round();
        let isNew = interval.is_none() || x_pixel != interval.x_pixel;
        if let Some(interval) = &mut interval {
            if isNew {
                finish_interval(interval.clone());
            }
        }

        // Start a new interval if this point falls in a new one
        if let Some(interval) = &mut interval {
            if isNew {
                interval = Interval {
                    x,
                    endX: (x * pixel_per_x_value + pixel_per_x_value).round(),
                    x_pixel,
                    index: index,
                    labels: vec![Label {
                        index,
                        value: label.as_ref().unwrap().to_string(),
                    }],
                };
            } else {
                interval.labels.push(Label {
                    index,
                    value: label.as_ref().unwrap().to_string(),
                });
            }
        }
    }

    if let Some(interval) = &mut interval {
        finish_interval(interval.clone());
    }

    if let Some(first_past_bounds) = first_past_bounds {
        indices.push_back(StatePoint {
            x: maxX,
            index: first_past_bounds,
            states: None,
        });
    }

    indices.into_iter().collect()
}

#[derive(Debug)]
struct Interval {
    x: f64,
    endX: f64,
    x_pixel: usize,
    index: Option<usize>,
    labels: Vec<Label>,
}

#[derive(Debug)]
struct Label {
    index: usize,
    value: String,
}
```
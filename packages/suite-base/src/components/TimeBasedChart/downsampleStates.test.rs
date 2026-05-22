```rust
use std::collections::{HashMap, VecDeque};
use serde_json::Value;

fn create_data(refs: Vec<(f64, &str)>) -> Vec<Datum> {
    refs.into_iter().map(|(x, label)| Datum { x, y: 0.0, label }).collect()
}

const A = "a";
const B = "b";

fn downsample_states(
    data: VecDeque<(f64, Value)>,
    bounds: &Bounds,
    num_points: usize,
) -> Vec<Datum> {
    if data.len() <= num_points {
        return data.into_iter().map(|(x, y)| Datum { x, y }).collect();
    }

    let interval_width = bounds.width / (num_points as f64);
    let mut result = Vec::new();

    for i in 0..num_points {
        if i == 0 {
            result.push(Datum {
                x: i as f64 * interval_width,
                y: data[0].1.as_f64().unwrap(),
                label: A,
            });
        } else if i < num_points - 1 {
            let next_point = data[i + 1];
            if let (x, &state) = next_point {
                if state == Value::String(A.into()) {
                    result.push(Datum {
                        x,
                        y: data[i].1.as_f64().unwrap(),
                        label: A,
                    });
                } else if state == Value::String(B.into()) {
                    result.push(Datum {
                        x,
                        y: data[i].1.as_f64().unwrap(),
                        label: B,
                    });
                }
            }
        } else {
            result.push(Datum {
                x: i as f64 * interval_width,
                y: data[data.len() - 1].1.as_f64().unwrap(),
                label: A,
            });
        }
    }

    result
}

fn main() {
    let bounds = Bounds {
        width: 100.0,
        height: 100.0,
        bounds: BoundingBox { x: (0.0, 100.0), y: (0.0, 100.0) },
    };
    let num_points = 6;
    let data = vec![
        (0.0, Value::String(A.into())),
        (50.0, Value::String(B.into())),
        (100.0, Value::String(A.into())),
    ];
    let result = downsample_states(data.into(), &bounds, num_points);
    println!("{:?}", result);
}
```
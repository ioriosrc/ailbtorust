```rust
use std::vec::Vec;

type Datum = (
    // chart.js supported properties to show a label above the datapoint
    // used by the state transition panel to show a label above the transition datum
    Option<&str>,
    Option<String>,
);

type ObjectData = Vec<Option<Datum>>;

#[derive(Clone, Debug)]
pub struct ChartData {
    points: Vec<(f32, f32)>,
    labels: Vec<Option<String>>,
}

impl ChartData {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            labels: Vec::new(),
        }
    }

    pub fn add_point(&mut self, x: f32, y: f32, label: Option<&str>) {
        self.points.push((x, y));
        self.labels.push(label);
    }
}

pub type TypedData = (
    Vec<f32>,
    Vec<f32>,
    Vec<Option<String>>,
);

#[derive(Clone, Debug)]
pub struct TypedChartData {
    points: Vec<(f32, f32)>,
    labels: Vec<Option<&str>>,
}

impl TypedChartData {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            labels: Vec::new(),
        }
    }

    pub fn add_point(&mut self, x: f32, y: f32, label: Option<&str>) {
        self.points.push((x, y));
        self.labels.push(label);
    }
}

pub type RpcScale = (
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
);

#[derive(Clone, Debug)]
pub struct RpcScales {
    x: Option<RpcScale>,
    y: Option<RpcScale>,
}

pub type RpcElement = (
    Option<Datum>,
    usize,
    usize,
    (f32, f32),
);

type EventListenerHandler = fn(&str, Option<fn() -> ()>);
```
```rust
use std::ops::{AddAssign, Div};
use std::f64;

type UpdateParams = (
    Option<Vec<f64>>,
    Option<(usize, usize)>,
    Option<rpc_scales::RpcScales>,
);

// Define the RpcScales type from the given code snippet
struct RpcScales {
    x: Option<RpcScale>,
    y: Option<RpcScale>,
}

// Define the RpcScale type from the given code snippet
struct RpcScale {
    min: f64,
    max: f64,
}

// Define the PlotViewport struct from the given code snippet
#[derive(Clone)]
struct PlotViewport {
    width: usize,
    height: usize,
    bounds: Option<PlotBounds>,
}

// Define the PlotBounds struct from the given code snippet
#[derive(Clone)]
struct PlotBounds {
    x: PlotScale,
    y: PlotScale,
}

// Define the PlotScale struct from the given code snippet
#[derive(Clone, Copy)]
struct PlotScale {
    min: f64,
    max: f64,
}

type ChartDatasets = Vec<ChartDataset>;

// Define the ChartDataset struct from the given code snippet
struct ChartDataset {
    data: Vec<f64>,
    label_color: String,
    label: String,
    states: Vec<f64>,
}

const MAX_POINTS: usize = 100;

impl Downsampler {
    fn new() -> Self {
        Self {
            datasets: vec![],
            dataset_bounds: None,
            scales: None,
        }
    }

    pub fn update(&mut self, opt: UpdateParams) {
        let (datasets, dataset_bounds, scales) = opt;
        self.datasets = datasets.unwrap_or_default();
        self.dataset_bounds = dataset_bounds;
        self.scales = scales;
    }

    pub fn downsample(&self) -> Option<Vec<ChartDataset>> {
        if self.dataset_bounds.is_none() {
            return None;
        }

        let width = self.dataset_bounds?.width as f64;
        let height = self.dataset_bounds?.height as f64;

        let current_scales = self.scales.unwrap_or_default();
        let view: PlotViewport = Some(PlotViewport {
            width,
            height,
            bounds: Some(PlotBounds {
                x: PlotScale {
                    min: current_scales.x.min,
                    max: current_scales.x.max,
                },
                y: PlotScale {
                    min: current_scales.y.min,
                    max: current_scales.y.max,
                },
            }),
        });

        let num_points = MAX_POINTS as f64 / self.datasets.len().max(1.0);
        let downsampled_datasets = self
            .datasets
            .iter()
            .map(|dataset| {
                if !view.is_none() {
                    let downsampled = downsample_states(
                        dataset.data.clone(),
                        view.unwrap(),
                        num_points,
                    );
                    let y_value = dataset.data[0].unwrap_or(0.0);
                    let resolved = downsampled
                        .iter()
                        .map(|item| {
                            if item.x.is_nan() || item.y.is_nan() {
                                return Some({
                                    x: f64::NAN,
                                    y: f64::NAN,
                                    value: f64::NAN,
                                });
                            }
                            Some({
                                x: item.x,
                                y: item.y,
                                label_color: "#000",
                                label: "[...]",
                                states: item.states.clone(),
                            })
                        })
                        .collect::<Vec<Option<f64>>>();
                    return Some({
                        dataset.data: resolved.into_iter().flatten().collect(),
                        label_color: "#000",
                        label: "[...]",
                        states: vec![],
                    });
                }
                None
            })
            .filter_map(|item| item)
            .collect();

        if downsampled_datasets.is_empty() {
            return None;
        }

        Some(downsampled_datasets)
    }
}
```
```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Datum {
    pub x: f64,
    pub y: f64,
    pub value: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct HoverElement {
    config_index: u32,
    data: Datum,
}

impl Default for HoverElement {
    fn default() -> Self {
        Self {
            config_index: 0,
            data: Datum {
                x: 0.0,
                y: 0.0,
                value: None,
            },
        }
    }
}

pub fn hover_elements(count: usize) -> Vec<HoverElement> {
    (1..=count).map(|_| HoverElement::default()).collect()
}

#[derive(Serialize, Deserialize)]
pub struct PlotPath {
    enabled: bool,
    timestamp_method: TimestampMethod,
    value: String,
}

impl Default for PlotPath {
    fn default() -> Self {
        Self {
            enabled: true,
            timestamp_method: TimestampMethod::HeaderStamp,
            value: "".to_string(),
        }
    }
}

pub fn paths(count: usize) -> Vec<PlotPath> {
    (1..=count).map(|_| PlotPath::default()).collect()
}

#[derive(Serialize, Deserialize)]
pub struct BasePlotPath {
    value: String,
    enabled: bool,
}

impl Default for BasePlotPath {
    fn default() -> Self {
        Self {
            value: "".to_string(),
            enabled: true,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PlotConfig {
    following_view_width: f64,
    lichtblick_panel_title: String,
    is_synced: bool,
    legend_display: String,
    maxX_value: f64,
    maxY_value: f64,
    minX_value: f64,
    minY_value: f64,
    paths: Vec<PlotPath>,
    show_legend: bool,
    show_plot_values_in_legend: bool,
    show_sidebar: bool,
    show_x_axis_labels: bool,
    show_y_axis_labels: bool,
    sidebar_dimension: f64,
    sidebar_width: f64,
    xAxis_path: BasePlotPath,
    xAxis_val: PlotXAxisVal,
}

impl Default for PlotConfig {
    fn default() -> Self {
        Self {
            following_view_width: 0.0,
            lichtblick_panel_title: "".to_string(),
            is_synced: false,
            legend_display: "floating".to_string(),
            maxX_value: 0.0,
            maxY_value: 0.0,
            minX_value: 0.0,
            minY_value: 0.0,
            paths: Vec::new(),
            show_legend: true,
            show_plot_values_in_legend: false,
            show_sidebar: true,
            show_x_axis_labels: true,
            show_y_axis_labels: true,
            sidebar_dimension: 0.0,
            sidebar_width: 0.0,
            xAxis_path: BasePlotPath::default(),
            xAxis_val: PlotXAxisVal::Timestamp,
        }
    }
}

pub enum TimestampMethod {
    HeaderStamp,
    ReceiveTime,
}
```
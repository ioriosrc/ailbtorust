```rust
use chrono::{NaiveDateTime, Utc};
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

type Time = NaiveDateTime<Utc>;

#[derive(Debug)]
struct MessageEvent {
    receive_time: Time,
    header_stamp: Option<Time>,
}

#[derive(Debug)]
struct Immutable<T> {
    data: T,
}

#[derive(Debug)]
struct MessagePathDataItem {
    // Implement this struct based on your specific requirements
}

#[derive(Debug)]
enum TimestampMethod {
    // Define the possible timestamp methods here
}

fn is_reference_line_plot_path_type(path: &Immutable<PlotPath>) -> bool {
    path.data.value.parse::<f64>().is_ok()
}

fn presence<T>(value: Option<&T>) -> Option<&T> {
    value.or(Some(&""))
}

fn plot_path_display_name(path: Immutable<PlotPath>, index: usize) -> String {
    let label = path.data.label.as_ref().map(|label| label.clone()).unwrap_or("Series".to_string());
    let value = path.data.value.as_ref().map(|value| value.clone()).unwrap_or("".to_string());

    if label.is_empty() && value.is_empty() {
        format!("Series {}", index + 1)
    } else {
        format!("{}: {}", label, value)
    }
}

type DeprecatedPlotConfig = HashMap<String, bool>;

#[derive(Debug)]
struct PlotLegendDisplay {
    display: String,
}

#[derive(Debug)]
struct PlotConfig {
    paths: Vec<Immutable<PlotPath>>,
    min_x_value: Option<f64>,
    max_x_value: Option<f64>,
    min_y_value: Option<&str | f64>,
    max_y_value: Option<&str | f64>,
    show_legend: bool,
    legend_display: PlotLegendDisplay,
    show_plot_values_in_legend: bool,
    show_x_axis_labels: bool,
    show_y_axis_labels: bool,
    is_synced: bool,
    x_axis_val: PlotXAxisVal,
    x_axis_path: Option<Immutable<BasePlotPath>>,
    following_view_width: Option<f64>,
    sidebar_dimension: f64,
}
```
```rust
use fluentui::{
    components::{icon::Icon, button::IconButton},
    iconset as fui_iconset,
};
use mui::prelude::*;
use std::rc::Rc;

use crate::{
    constants::{DEFAULT_PLOT_PATH, ROW_HEIGHT},
    utils::config::{PlotConfig, PlotPath},
};

const min_legend_width: u16 = 25;
const max_legend_width: u16 = 800;

type Props<'a> = {
    coordinator: Option<&'a PlotCoordinator>,
    legend_display: &'a str,
    onClick_path: &'a dyn Fn(usize),
    paths: Vec<PlotPath>,
    save_config: &'a dyn Fn(&'a mut PlotConfig),
    show_legend: bool,
    sidebar_dimension: f64,
    show_values: bool,
    hovered_values_by_series_index: Option<&'a [String]>,
};

fn main() {
    // Implementation of the main function
}
```
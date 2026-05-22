```rust
use crate::chartjs_plugin_annotation::{AnnotationOptions};
use crate::suite_base::panels::plot::utils::{Config, MathFunction};

pub const MATH_FUNCTIONS: { fn: String, func: Box<dyn Fn(f64) -> f64> } = {
  ("abs", Box::new(MathFunction::Abs)),
  ("acos", Box::new(MathFunction::Acos)),
  ("asin", Box::new(MathFunction::Asin)),
  ("atan", Box::new(MathFunction::Atan)),
  ("ceil", Box::new(MathFunction::Ceil)),
  ("cos", Box::new(MathFunction::Cos)),
  ("log", Box::new(MathFunction::Log)),
  ("log1p", Box::new(MathFunction::Log1p)),
  ("log2", Box::new(MathFunction::Log2)),
  ("log10", Box::new(MathFunction::Log10)),
  ("round", Box::new(MathFunction::Round)),
  ("sign", Box::new(MathFunction::Sign)),
  ("sin", Box::new(MathFunction::Sin)),
  ("sqrt", Box::new(MathFunction::Sqrt)),
  ("tan", Box::new(MathFunction::Tan)),
  ("trunc", Box::new(MathFunction::Trunc)),
};

pub const DEFAULT_SIDEBAR_DIMENSION: i32 = 240;

pub const DEFAULT_ANNOTATION: AnnotationOptions = {
  type: "line",
  display: true,
  drawTime: "beforeDatasetsDraw",
  scaleID: "y",
  borderWidth: 1,
  borderDash: vec![5, 5],
};

pub const DEFAULT_PLOT_CONFIG: Config = {
  paths: Vec::new(),
  minYValue: None,
  maxYValue: None,
  showXAxisLabels: true,
  showYAxisLabels: true,
  showLegend: true,
  legendDisplay: "floating",
  showPlotValuesInLegend: false,
  isSynced: true,
  xAxisVal: "timestamp",
  sidebarDimension: DEFAULT_SIDEBAR_DIMENSION,
};

pub const DEFAULT_PLOT_PATH: PlotPath = PlotPath {
  timestamp_method: "receiveTime",
  value: String::new(),
  enabled: true,
};
```
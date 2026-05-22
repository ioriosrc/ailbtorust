```rust
use std::convert::From;

pub struct PieChartConfig {
    pub path: String,
    pub title: String,
    pub legend1: String,
    pub legend2: String,
    pub legend3: String,
    pub legend4: String,
    pub legend5: String,
    pub legend6: String,
    pub legend7: String,
    pub legend8: String,
    pub legend9: String,
    pub legend10: String,
}

impl From<()> for PieChartConfig {
    fn from(_: ()) -> Self {
        PieChartConfig {
            path: "".to_string(),
            title: "".to_string(),
            legend1: "0".to_string(),
            legend2: "0".to_string(),
            legend3: "0".to_string(),
            legend4: "0".to_string(),
            legend5: "0".to_string(),
            legend6: "0".to_string(),
            legend7: "0".to_string(),
            legend8: "0".to_string(),
            legend9: "0".to_string(),
            legend10: "0".to_string(),
        }
    }
}

pub struct PieChartState {
    pub path: String,
    pub parsed_path: Option<Vec<f32>>,
    pub latest_message: Option<String>,
    pub latest_matching_queried_data: Vec<f32>,
    pub error: Option<String>,
    pub path_parse_error: Option<String>,
}

impl From<()> for PieChartState {
    fn from(_: ()) -> Self {
        PieChartState {
            path: "".to_string(),
            parsed_path: None,
            latest_message: None,
            latest_matching_queried_data: Vec::new(),
            error: None,
            path_parse_error: None,
        }
    }
}
```
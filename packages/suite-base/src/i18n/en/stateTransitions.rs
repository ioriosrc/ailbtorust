```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StateTransitions {
    add_series_button: String,
    labels: Labels,
    max: String,
    max_x_error: String,
    min: String,
    path_error_message: String,
    seconds_range: String,
    x_axis: String,
}

#[derive(Serialize, Deserialize)]
pub struct Labels {
    add_series: String,
    delete_series: String,
    general: String,
    help_general: String,
    label: String,
    message_path: String,
    series: String,
    show_points: String,
    sync: String,
    timestamp: String,
    timestamp_header_stamp: String,
    timestamp_receive_time: String,
}
```
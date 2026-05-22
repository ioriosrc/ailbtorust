```rust
use chart::{Options, plugins::datalabels::LabelDisplayMode, plugins::zoom::Zoom};
use serde_json::Value;

pub const EMPTY_ITEMS_BY_PATH: Value = json!({});

pub const DEFAULT_STATE_TRANSITION_PATH: Value = json!({
  value: "",
  timestamp_method: "receive_time",
});

pub const STATE_TRANSITION_PLUGINS: Options<plugins::datalabels::LabelDisplayMode, plugins::zoom::Zoom> = {
  datalabels: plugins::datalabels::LabelDisplayMode::Auto,
  zoom: Zoom {
    enabled: true,
    mode: "x",
    sensitivity: 3.0,
    speed: 0.1,
  },
  pan: plugins::zoom::Pan {
    mode: "x",
    enabled: true,
    speed: 20.0,
    threshold: 10.0,
  },
};

pub const TRANSITIONABLE_ROS_TYPES: &[&str] = &["bool", "int8", "uint8", "int16", "uint16", "int32", "uint32", "int64", "uint64", "string"];
```
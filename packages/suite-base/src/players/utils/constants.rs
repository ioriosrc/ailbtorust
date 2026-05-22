```rust
const FREQUENCY_LIMIT: u32 = 60;

pub const HIGH_FREQUENCY_ALERT: Alert = Alert {
    id: "high-frequency",
    severity: Severity::Warn,
    message: "High frequency topics detected".to_string(),
    error_message: "The current data source has one or more topics with message frequency higher than 60Hz, which may impact performance and application memory.".to_string(),
};

pub const LOG_SCHEMAS: HashSet<String> = [
    "rosgraph_msgs/Log".to_string(),
    "rosgraph_msgs/msg/Log".to_string(), // ROS 1 (alternative format)
    "rcl_interfaces/msg/Log".to_string(), // ROS 2
    "foxglove.Log".to_string(), // Foxglove schema
].iter().cloned().collect();
```
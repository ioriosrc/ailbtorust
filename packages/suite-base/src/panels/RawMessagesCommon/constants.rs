```rust
pub const COLLAPSE_TEXT_OVER_LENGTH: usize = 512;
pub const CUSTOM_METHOD: &str = "custom";
pub const DATA_ARRAY_PREVIEW_LIMIT: usize = 20;
pub const diffArrow: &str = "->";
pub const FONT_SIZE_OPTIONS: [usize; 13] = [8, 9, 10, 11, 12, 14, 16, 18, 24, 30, 36, 48, 60, 72];
pub const PATH_NAME_AGGREGATOR: &str = "~";
pub const PREV_MSG_METHOD: &str = "previous message";
pub const ROS1_COMMON_MSG_PACKAGES: std::collections::HashSet<String> = std::collections::HashSet::from_iter(
    ros1.keys()
        .map(|key| key.split("/").next().unwrap())
        .collect::<Vec<&str>>()
);
```
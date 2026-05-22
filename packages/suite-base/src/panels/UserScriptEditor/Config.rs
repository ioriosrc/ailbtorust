```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub selected_node_id: Option<String>,
    // Used only for storybook screenshot testing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor_for_storybook: Option<Box<dyn std::fmt::Display>>,
    // Used only for storybook screenshot testing.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub additional_back_stack_items: Vec<Script>,
    // Used only for storybook screenshot testing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_format_on_save: Option<bool>,
}
```
```rust
use serde::{Deserialize, Serialize};

// Define the types for icon and action items based on your requirements

#[derive(Serialize, Deserialize)]
struct SettingsIcon {
    // Icon details
}

#[derive(Serialize, Deserialize)]
pub struct SettingsTreeNodeActionItem {
    id: String,
    label: String,
    type: &'static str,
    display: &'static str,
    icon: Option<SettingsIcon>,
}

#[derive(Serialize, Deserialize)]
pub struct SettingsTreeNodeActionDivider {
    type: &'static str,
}

#[derive(Serialize, Deserialize)]
pub struct SettingsTreeNode {
    actions: Vec<SettingsTreeNodeActionItem>,
    children: Option<serde_json::Value>,
    default_expansion_state: String,
    enable_visibility_filter: bool,
    error: Option<String>,
    fields: Option<serde_json::Value>,
    icon: Option<SettingsIcon>,
    label: String,
    order: Option<i32>,
    renamable: bool,
    visible: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SettingsTreeNodeNoChildren {
    actions: Vec<SettingsTreeNodeActionItem>,
    children: None,
    default_expansion_state: String,
    enable_visibility_filter: bool,
    error: Option<String>,
    fields: Option<serde_json::Value>,
    icon: Option<SettingsIcon>,
    label: String,
    order: Option<i32>,
    renamable: bool,
    visible: bool,
}
```
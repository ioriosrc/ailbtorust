```rust
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct MosaicNode<String> {}

#[derive(Debug, PartialEq)]
pub struct LayoutData {};

pub type TabConfig = String;

pub type TabPanelConfig = HashMap<i32, TabConfig>;

#[derive(Debug, PartialEq)]
pub struct TabLocation {
    pub panel_id: String;
    pub tab_index: Option<i32>;
}

/**
 * Metadata describing a layout.
 */
#[derive(Debug, PartialEq)]
pub struct LayoutInfo {
    pub name: String;
    pub from: String;
    pub data: LayoutData;
}
```
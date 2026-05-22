```rust
use std::any::{Any, TypeId};
use std::collections::HashMap;

// Mosaic Types
#[derive(Debug)]
pub enum MosaicDropTargetPosition {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug)]
pub struct MosaicDropResult {
    path: Option<MosaicPath>,
    position: Option<MosaicDropTargetPosition>,
    tab_id: Option<String>,
}

// PanelConfig
#[derive(Debug)]
pub type PanelConfig = HashMap<TypeId, Box<dyn Any>>;

#[derive(Debug)]
pub enum TimeDisplayMethod {
    SEC,
    TOD,
}

#[derive(Debug)]
pub struct PlaybackConfig {
    speed: f64,
}

#[derive(Debug)]
pub struct UserScript {
    name: String,
    source_code: String,
}

#[derive(Debug)]
pub type UserScripts = HashMap<String, UserScript>;

#[derive(Debug)]
pub type SaveConfig<T> = fn(&mut T, &dyn Any) -> ();

#[derive(Debug)]
pub type SavedProps = HashMap<&str, Box<dyn Any>>;

#[derive(Debug)]
pub struct OpenSiblingPanel {
    panel_type: String,
    sibling_config_creator: fn(&PanelConfig) -> PanelConfig,
    update_if_exists: bool,
}
```
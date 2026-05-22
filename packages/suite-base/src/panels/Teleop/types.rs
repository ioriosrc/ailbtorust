```rust
use crate::{PanelExtensionContext, SaveConfig, PanelExtensionContext as Context};
use crate::TeleopConfig;

// Define the props for the teleop panel
pub type TeleopPanelProps = Context<TeleopConfig>;

// Define the config structure for the teleop panel
#[derive(Debug)]
pub struct TeleopConfig {
    pub topic: Option<String>,
    pub publish_rate: f64,
    pub up_button: ButtonConfig,
    pub down_button: ButtonConfig,
    pub left_button: ButtonConfig,
    pub right_button: ButtonConfig,
}

// Define the button configuration structure
#[derive(Debug)]
pub struct ButtonConfig {
    pub field: String,
    pub value: i32,
}

// Define the directional pad action enum
#[derive(Debug, PartialEq, Eq)]
pub enum DirectionalPadAction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

// Define the props for the directional pad
pub type DirectionalPadProps = ButtonConfig;

// Define the props for the teleop panel adapter
pub type TeleopPanelAdapterProps = SaveConfig<TeleopConfig>;
```
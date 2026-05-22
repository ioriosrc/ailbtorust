```rust
use crate::ExtensionSettings;
use crate::BuildSettingsTreeProps;
use crate::ActionMenuProps;

pub type ExtensionSettings = std::collections::HashMap<String, HashMap<String, PanelSettings<()>>>;

pub type BuildSettingsTreeProps = {
  config: Option<&serde_json::Value>,
  extension_settings: ExtensionSettings,
  message_pipeline_state: fn() -> MessagePipelineContext,
  panel_type: Option<&str>,
  settings_tree: Option<&ImmutableSettingsTree>,
};

pub type ActionMenuProps = {
  allow_share: bool,
  on_reset: fn(),
  on_share: fn(),
  fontSize: Option<svgicons::SvgIconProps<"small">>,
};
```
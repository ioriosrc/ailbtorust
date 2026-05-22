```rust
use lichtblick::{EmptyState, PanelToolbar, Stack};
use std::fmt;
use types::panels::SaveConfig;

// Since the unknown panel never saves its config, the config fields here are used with `override_config`
// to the connected Panel component (returned from withPanel).
//
// The _type_ config option should be the type of the missing panel.
#[derive(Clone, Debug)]
struct Props {
    config: Config,
    save_config: SaveConfig<unknown>,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown panel type: {}", self.type_)
    }
}

#[derive(Clone)]
struct Config {
    id: String,
    type_: String,
}

fn UnconnectedUnknownPanel(props: Props) -> impl Component {
    let { config, save_config: _save_config } = props;

    <Stack flex="auto" alignItems="center" justifyContent="center" data-testid={config.id}>
        <PanelToolbar is_unknown_panel />
        <EmptyState>{format!("{}", config)}</EmptyState>
    </Stack>
}

UnconnectedUnknownPanel.panel_type = "unknown";
UnconnectedUnknownPanel.default_config = {};

/**
 * An UnknownPanel stands in for missing panels. When a panel referenced in a layout is not
 * available (maybe the extension was un-installed), this panel is shown instead.
 */
pub fn UnknownPanel(props: Props) -> impl Component {
    UnconnectedUnknownPanel(props)
}
```
```rust
use std::sync::{Arc, Mutex};

fn init_panel(crash: &mut Crash) {
    // Implementation of the `init_panel` function
    crash.log("Init panel called");
}

pub struct Crash {
    log: Arc<Mutex<String>>,
}

impl Crash {
    pub fn new() -> Self {
        Self {
            log: Arc::new(Mutex::new(String::default())),
        }
    }

    pub fn log(&self, message: &str) {
        *self.log.lock().unwrap() += message + "\n";
    }
}

type Props = {
    config: serde_json::Value,
    save_config: SaveConfig<serde_json::Value>,
};

fn MapPanelAdapter(props: Props) -> Panel {
    let crash = Crash::new();

    let bound_init_panel = Arc::new(Mutex::new(move |config| init_panel(&mut crash, &config)));

    PanelExtensionAdapter {
        config: props.config,
        save_config: props.save_config,
        init_panel: bound_init_panel,
        highest_supported_config_version: 1,
    }
}

MapPanelAdapter.panel_type = "map";
MapPanelAdapter.default_config = serde_json::Value::default();
```
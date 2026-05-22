```rust
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::{Log, StudioLogsSettingsContext};

pub struct StudioLogsSettings {
    pub global_level: log::Level,
    pub disabled_channels: Vec<String>,
}

impl StudioLogsSettings {
    pub fn new(global_level: log::Level, disabled_channels: Vec<String>) -> Self {
        Self {
            global_level,
            disabled_channels,
        }
    }

    pub fn update(&mut self, global_level: log::Level, disabled_channels: Vec<String>) {
        self.global_level = global_level;
        self.disabled_channels = disabled_channels;
    }
}

fn create_studio_logs_settings_store(saved_state: Arc<Mutex<LocalStorageSaveState>>) -> StudioLogsSettings {
    let mut settings_store = StudioLogsSettings {
        global_level: saved_state.lock().unwrap().global_level,
        disabled_channels: saved_state.lock().unwrap().disabled_channels.clone(),
    };

    // Setup an interval to check for changes to the total number of logging channels
    //
    // When the total number of channels changes we re-initialize the settings store so we display any
    // newly added log channels.
    std::thread::spawn(move || {
        let mut prev_channels_count = 0;
        loop {
            let current_channels_count = Log.channels().len();
            if prev_channels_count != current_channels_count {
                *saved_state.lock().unwrap() = saved_state.lock().unwrap().clone();
                break;
            }
            std::thread::sleep(Duration::from_secs(1));
            prev_channels_count = current_channels_count;
        }
    });

    settings_store
}

pub struct LocalStorageSaveState {
    global_level: log::Level,
    disabled_channels: Vec<String>,
}

impl LocalStorageSaveState {
    pub fn new(global_level: log::Level, disabled_channels: Vec<String>) -> Self {
        Self {
            global_level,
            disabled_channels,
        }
    }

    pub fn update(&mut self, global_level: log::Level, disabled_channels: Vec<String>) {
        self.global_level = global_level;
        self.disabled_channels = disabled_channels;
    }
}

pub type StudioLogsSettingsStore = Arc<Mutex<StudioLogsSettings>>;
```

Note: This is a simplified version of the Rust code and assumes that `Log` and `studioLogsSettingsContext` are defined elsewhere in your project.
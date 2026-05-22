```rust
use std::sync::{Arc, RwLock};

pub struct StudioLogsSettings {
    global_level: LogLevel,
    channels: Vec<(String, bool)>,
}

impl StudioLogsSettings {
    pub fn new(global_level: LogLevel) -> Self {
        StudioLogsSettings {
            global_level,
            channels: Vec::new(),
        }
    }

    pub fn enable_channel(&mut self, name: String) {
        if !self.channels.iter().any(|(n, _)| n == &name) {
            self.channels.push((name, true));
        }
    }

    pub fn disable_channel(&mut self, name: String) {
        self.channels.retain(|(_, enabled)| enabled && name != &name);
    }

    pub fn enable_prefix(&mut self, prefix: String) {
        let mut enabled_channels = Vec::new();
        for (name, enabled) in &self.channels {
            if name.starts_with(&prefix) || enabled {
                enabled_channels.push((*name, *enabled));
            }
        }
        self.channels = enabled_channels;
    }

    pub fn disable_prefix(&mut self, prefix: String) {
        let mut disabled_channels = Vec::new();
        for (name, enabled) in &self.channels {
            if !name.starts_with(&prefix) || !enabled {
                disabled_channels.push((*name, *enabled));
            }
        }
        self.channels = disabled_channels;
    }

    pub fn set_global_level(&mut self, level: LogLevel) {
        self.global_level = level;
    }
}

pub struct StudioLogsSettingsContext(Arc<RwLock<StudioLogsSettings>>);

impl StudioLogsSettingsContext {
    pub fn new(global_level: LogLevel) -> Self {
        StudioLogsSettingsContext(Arc::new(RwLock::new(StudioLogsSettings::new(global_level))))
    }

    pub fn use_studio_logs_settings(&self) -> StudioLogsSettings {
        let studio_logs_settings = self.0.read().unwrap();
        StudioLogsSettings {
            global_level: studio_logs_settings.global_level,
            channels: studio_logs_settings.channels.clone(),
        }
    }
}

pub type IStudioLogsSettings = StudioLogsSettings;
```

### Explanation:
1. **Structs and Enums**:
   - `StudioLogsSettings`: Holds the global log level and a list of channel settings.
   - `LogLevel`: An enum representing different log levels.
   - `StudioLogsSettingsContext`: Manages the shared state of `StudioLogsSettings`.

2. **Initialization**:
   - The `StudioLogsSettings` struct is initialized with a global log level.

3. **Methods**:
   - `enable_channel`, `disable_channel`, `enable_prefix`, `disable_prefix`, and `set_global_level` are methods to manage the channels and the global log level.

4. **Context Management**:
   - The `StudioLogsSettingsContext` struct provides a context for managing the shared state of `StudioLogsSettings`.

5. **Usage**:
   - The `use_studio_logs_settings` method allows consuming components to access the current state of `StudioLogsSettings`.
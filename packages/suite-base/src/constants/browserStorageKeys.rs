```rust
use std::env;

const APP_CONFIG: &str = "your_app_config_here"; // Replace with actual AppConfig value

const KEY_WORKSPACE_PREFIX: &str = if env::var("APP_CONFIG_DEVWORKSPACE").is_ok() {
    format!("{},", APP_CONFIG.dev_workspace)
} else {
    ""
};

// Local storage keys
pub const LOCAL_STORAGE_STUDIO_LAYOUT_KEY: &str = format!("{}.studio.layout", KEY_WORKSPACE_PREFIX);
pub const LOCAL_STORAGE_PROFILE_DATA: &str = format!("{}.studio.profile-data", KEY_WORKSPACE_PREFIX);
pub const LOCAL_STORAGE_APP_CONFIGURATION: &str = format!("{}.studio.app-configuration.", KEY_WORKSPACE_PREFIX);
pub const LOCAL_STORAGE_PANEL_LOGS_HEIGHT: &str = format!("{}.studio.panel-logs-height", KEY_WORKSPACE_PREFIX);

// Session storage keys
pub const SESSION_STORAGE_LOGS_SETTINGS: &str = format!("{}.blick.logs-settings", KEY_WORKSPACE_PREFIX);
pub const SESSION_STORAGE_LICHTBLICK_WORKSPACE: &str = format!("{}.fox.workspace", KEY_WORKSPACE_PREFIX);
pub const SESSION_STORAGE_I18N_LANGUAGE: &str = format!("{}.i18nextLng", KEY_WORKSPACE_PREFIX);
```
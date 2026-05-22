```rust
pub enum AppSetting {
    COLOR_SCHEME = "color_scheme",
    TIMEZONE = "timezone",
    TIME_FORMAT = "time.format",
    MESSAGE_RATE = "message_rate",
    UPDATES_ENABLED = "updates.enabled",
    LANGUAGE = "language",
    DEFAULT_STEP_SIZE = "step_size",

    ROS_PACKAGE_PATH = "ros.ros_package_path",
    ENABLE_NEW_TOPNAV = "enable_new_topnav",

    TELEMETRY_ENABLED = "telemetry.telemetry_enabled",
    CRASH_REPORTING_ENABLED = "telemetry.crash_reporting_enabled",

    SHOW_DEBUG_PANELS = "show_debug_panels",

    HIDE_SIGN_IN_PROMPT = "hide_sign_in_prompt",
    LAUNCH_PREFERENCE = "launch_preference",
    SHOW_OPEN_DIALOG_ON_STARTUP = "ui.open-dialog-startup",
    ENABLE_UNIFIED_NAVIGATION = "ui.new-app-menu",

    // Dev only
    ENABLE_LAYOUT_DEBUGGING = "enable_layout_debugging",
    ENABLE_MEMORY_USE_INDICATOR = "dev.memory-use-indicator",
}
```
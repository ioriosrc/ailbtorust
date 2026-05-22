```rust
use crate::settings::{get_app_setting, AppSetting};

fn get_telemetry_settings() -> {
    let crash_reporting_enabled: bool = get_app_setting(AppSetting::CRASH_REPORTING_ENABLED).unwrap_or(true);
    let telemetry_enabled: bool = get_app_setting(AppSetting::TELEMETRY_ENABLED).unwrap_or(true);

    ({ crash_reporting_enabled, telemetry_enabled })
}
```
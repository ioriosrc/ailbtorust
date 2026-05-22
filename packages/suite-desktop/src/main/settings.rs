```rust
use std::fs;
use std::path::{self};

type AppSetting = &'static str;

fn get_app_setting<T>(key: &AppSetting, override_user_data_dir: Option<&str>) -> Result<T, Box<dyn std::error::Error>> {
    let datastore_dir = if let Some(dir) = override_user_data_dir {
        dir
    } else {
        app::get_path("userData")?
    };
    let settings_dir = path::join(datastore_dir, "datastores", "settings");
    let settings_file = path::join(settings_dir, "settings.json");

    fs::create_dir_all(&settings_dir)?;

    if !fs::file_exists(&settings_file) {
        return Ok(None);
    }

    let contents = fs::read_to_string(&settings_file)?;
    let parsed_settings: serde_json::Value = serde_json::from_str(&contents)?;

    match parsed_settings.get(key) {
        Some(v) => Ok(v.clone()),
        None => Ok(None),
    }
}

fn set_app_setting(key: &AppSetting, value: &str, override_user_data_dir: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let datastore_dir = if let Some(dir) = override_user_data_dir {
        dir
    } else {
        app::get_path("userData")?
    };
    let settings_dir = path::join(datastore_dir, "datastores", "settings");
    let settings_file = path::join(settings_dir, "settings.json");

    fs::create_dir_all(&settings_dir)?;

    let mut existing_settings: serde_json::Value = serde_json::from_str("{}")?;
    if let Some(settings) = existing_settings.get_mut(key) {
        *settings = serde_json::Value::String(value.to_string());
    } else {
        existing_settings.insert(key.to_string(), serde_json::Value::String(value.to_string()));
    }

    fs::write(&settings_file, serde_json::to_string_pretty(&existing_settings)?)?;
    Ok(())
}
```
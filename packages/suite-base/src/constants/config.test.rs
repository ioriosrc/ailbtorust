```rust
use std::process::{self, env};

#[tokio::test]
async fn test_app_config() {
    let original_env = std::env::vars();

    // Clear environment variables to restore later
    std::env::set_var("API_URL", "");
    std::env::set_var("LICHTBLICK_SUITE_VERSION", "");
    std::env::set_var("DEV_WORKSPACE", "");

    // Clear module cache to ensure fresh imports
    let mut modules = std::collections::HashMap::new();
    for (key, value) in original_env {
        modules.insert(key.clone(), value.clone());
    }
    env::remove_all_vars();

    let config = super::config::APP_CONFIG;

    assert_eq!(config.api_url(), "");
    assert_eq!(config.version(), "unknown");
    assert_eq!(config.dev_workspace(), "");

    // Restore environment variables
    for (key, value) in modules {
        std::env::set_var(key, value);
    }
}
```
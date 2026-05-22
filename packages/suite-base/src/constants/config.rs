```rust
const API_URL: &str = env::var("API_URL").unwrap_or_default();
const LICHTBLICK_SUITE_VERSION: &str = env::var("LICHTBLICK_SUITE_VERSION").unwrap_or_default();
const DEV_WORKSPACE: &str = env::var("DEV_WORKSPACE").unwrap_or_default();

pub const APP_CONFIG: &[(&str, &str)] = &[
  ("apiUrl", API_URL),
  ("version", LICHTBLICK_SUITE_VERSION.unwrap_or("unknown")),
  ("devWorkspace", DEV_WORKSPACE.unwrap_or("")),
];
```
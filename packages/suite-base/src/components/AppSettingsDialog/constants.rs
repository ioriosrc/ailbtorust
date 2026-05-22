```rust
use std::collections::HashMap;

fn main() {
    let app_settings_about_items: HashMap<AppSettingsSectionKey, AppSettingsAboutItem> = HashMap::new();

    app_settings_about_items.insert(
        AppSettingsSectionKey::Documentation,
        AppSettingsAboutItem {
            subheader: "Documentation",
            links: vec![AppSettingsLink {
                title: "Check out our documentation",
                url: LICHTBLICK_DOCUMENTATION_LINK.to_string(),
            }],
        },
    );

    app_settings_about_items.insert(
        AppSettingsSectionKey::Legal,
        AppSettingsAboutItem {
            subheader: "Legal",
            links: vec![AppSettingsLink {
                title: "License terms",
                url: "https://github.com/lichtblick-suite/lichtblick/blob/main/LICENSE".to_string(),
            }],
        },
    );
}

enum AppSettingsSectionKey {
    Documentation,
    Legal,
}

struct AppSettingsAboutItem {
    subheader: String,
    links: Vec<AppSettingsLink>,
}

struct AppSettingsLink {
    title: String,
    url: String,
}
```
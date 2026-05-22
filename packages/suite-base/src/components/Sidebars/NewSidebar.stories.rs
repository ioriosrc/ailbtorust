```rust
use leptos::prelude::*;
use leptos_dom::IntoViewRef;
use leptos_useful::{use_app_configuration_value, use_leptos_devtools};

#[derive(Debug)]
struct AppSetting {
    ENABLE_NEW_TOPNAV: bool,
}

#[component]
pub fn AppSettings() -> impl IntoViewRef {
    let enable_new_topnav = use_app_configuration_value(AppSetting::ENABLE_NEW_TOPNAV);

    // Your application logic here
}
```

This Rust code represents the TypeScript/React code for a sidebar with left and right content. It uses `leptos` and `leptos_dom` for reactive programming and DOM interaction respectively. The `AppSettings` component is used to manage app settings, including enabling new topnav functionality.

Please note that this is a simplified example and does not include the full implementation of the original TypeScript/React code, such as the `Sidebars`, `Item`, and related functions.
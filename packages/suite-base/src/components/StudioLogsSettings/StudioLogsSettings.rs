```rust
use leptos::*;
use log::Level;

#[component]
fn StudioLogsSettings() -> impl IntoView {
    let log_settings = use_studio_logs_settings_tree();
    
    view! {
        <SettingsTreeEditor variant="log" settings={log_settings} />
    }
}

#[component]
fn StudioLogsSettingsSidebar() -> impl IntoView {
    view! {
        <SidebarContent overflow="auto" title="Studio Logs Settings" disable_padding>
            <StudioLogsSettings />
        </SidebarContent>
    }
}
```
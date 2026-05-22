```rust
use crate::components::launch_preference_screen::LaunchPreferenceScreen;
use crate::components::launching_in_desktop_screen::LaunchingInDesktopScreen;
use crate::settings::AppSetting;
use crate::{hooks::use_app_configuration_value, hooks::use_session_storage_value};

#[derive(Clone, Debug)]
pub struct LaunchPreference {
    global_launch_preference: String,
    session_launch_preference: Option<String>,
}

impl LaunchPreference {
    pub fn new(global_launch_preference: String) -> Self {
        Self {
            global_launch_preference,
            session_launch_preference: None,
        }
    }

    pub fn set_session_launch_preference(&mut self, value: &str) {
        self.session_launch_preference = Some(value.to_string());
    }

    pub fn get_active_preference(&self) -> Option<String> {
        self.session_launch_preference.or_else(|| {
            self.global_launch_preference.as_str().cloned()
        })
    }
}

pub fn launch_preference(props: props_with_children::PropsWithChildren) -> JSXElement {
    let global_launch_preference = use_app_configuration_value::<String>(AppSetting::LAUNCH_PREFERENCE).value();
    let session_launch_preference = use_session_storage_value::<String>(AppSetting::LAUNCH_PREFERENCE);
    let url = new_url(&window.location.href);

    // Session preferences take priority over URL and global preferences. This allows the button in
    // LaunchPreferenceScreen to override the url when clicked.
    let mut active_preference =
        session_launch_preference.unwrap_or_else(|| url.query().get("openIn").map(String::from));
    if active_preference.is_none() {
        active_preference = Some(global_launch_preference.to_string());
    }

    match active_preference.as_str() {
        LaunchPreferenceValue::WEB | LaunchPreferenceValue::DESKTOP | LaunchPreferenceValue::ASK => (),
        _ => panic!("Invalid launch preference value: {}", active_preference),
    }

    if let Some(has_params) = url.query().get("openIn").map(|v| v == "true") {
        if has_params && active_preference.as_str() != LaunchPreferenceValue::DESKTOP.to_string() {
            return <LaunchingInDesktopScreen />;
        } else if !has_params && active_preference.as_str() != LaunchPreferenceValue::WEB.to_string() {
            return <LaunchPreferenceScreen />;
        }
    }

    props.children()
}
```
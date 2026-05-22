```rust
use std::sync::{Arc, RwLock};

use lightning::prelude::*;

use crate::{
    AppSetting,
    LaunchPreferenceValue,
    use_message_pipeline,
    use_session_storage_value,
};

#[derive(Debug)]
struct Context {
    player_state: Arc<RwLock<serde_json::Value>>,
}

fn select_has_url_state(ctx: &Context) -> bool {
    ctx.player_state.read().unwrap()["urlState"].is_some()
}

pub fn use_default_web_launch_preference() {
    let has_url_state = use_message_pipeline(select_has_url_state);
    let launch_preference = use_session_storage_value(
        AppSetting::LAUNCH_PREFERENCE,
    );

    if is_desktop_app() {
        return;
    }

    if has_url_state && launch_preference.is_none() {
        set_launch_preference(LaunchPreferenceValue::WEB);
    }
}
```
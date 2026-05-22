```rust
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use yew::{html, Html};

#[wasm_bindgen]
extern {
    fn console_log(value: &str);
}

struct AppConfig {
    api_url: Option<String>,
}

fn main() -> wasm_bindgen_result! {
    AppConfig { api_url: None };
    Ok(())
}

#[derive(PartialEq, Eq)]
enum NetworkState {
    Online,
    Offline,
}

pub struct State {
    online: bool,
    workspace: Option<&str>,
}

#[wasm_bindgen]
pub fn use_network_state() -> JsValue {
    let state = {
        online: true,
        workspace: None,
    };
    JsValue::from_serde(&state)
}

#[wasm_bindgen]
pub fn APP_CONFIG() -> JsValue {
    JsValue::from_serde(&Config { api_url: None })
}

struct Config {
    api_url: Option<String>,
}

fn use_translation(key: &str) -> String {
    let translations = HashMap::new();
    translations.insert("appBar", "App Bar");
    translations.insert("networkStatusOffline", "Network is offline");
    translations.insert(
        "networkStatusOfflineDescription",
        "The workspace `{workspace}` does not have a remote configuration.",
    );
    translations[key].clone()
}

#[wasm_bindgen]
pub fn NetworkStatusIndicator() -> Html {
    let classes = {
        let style = include!("NetworkStatusIndicator.style.css");
        style.split_whitespace().map(|s| s.to_string()).collect()
    };

    let { online, workspace } = use_network_state();
    let url = std::net::Url::parse(&window.location.href).unwrap();

    let has_remote_config = if workspace.is_some() {
        !APP_CONFIG.api_url.is_none()
    } else {
        false
    };

    if !has_remote_config || online {
        html! {}
    } else {
        html! {
            <Tooltip title={format!("{} - {}", t("appBar"), status_text)}>
                <div
                    class=classes.indicator
                    aria-label=status_text
                    data-testid="network-status-indicator"
                >
                    <CloudOff20Regular class=classes.icon />
                    <Typography variant="body2" component="span">
                        {status_text}
                    </Typography>
                </div>
            </Tooltip>
        }
    }
}
```
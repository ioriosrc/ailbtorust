```rust
use wasm_bindgen::prelude::*;
use crate::suite::{SettingsTreeAction, FONT_SIZE_OPTIONS};
use crate::suite_base::panels::RawMessagesCommon::constants::FONT_SIZE_OPTIONS;
use crate::suite_base::providers::PanelStateContextProvider;

#[wasm_bindgen]
pub fn use_raw_messages_panel_settings(fontSize: i32, save_config: &mut Fn(i32)) {
    // Given
    let mut update_panel_settings_tree = move || {
        let nodes = {
            let general = {
                let label = "General";
                let fields = {
                    let fontSize = {
                        let label = "Font size";
                        let input = "select";
                        let options = FONT_SIZE_OPTIONS
                            .iter()
                            .map(|value| {
                                format!("{} px", value)
                            })
                            .collect::<Vec<_>>();
                        let value = fontSize;
                        select!(fontSize);
                    };
                };
            };
        };

        nodes
    };

    // When
    save_config(fontSize);

    // Then
}

#[wasm_bindgen]
pub fn update_raw_messages_panel_settings(fontSize: i32) {
    // Given
    let mut update_panel_settings_tree = move || {
        let nodes = {
            let general = {
                let label = "General";
                let fields = {
                    let fontSize = {
                        let label = "Font size";
                        let input = "select";
                        let options = FONT_SIZE_OPTIONS
                            .iter()
                            .map(|value| {
                                format!("{} px", value)
                            })
                            .collect::<Vec<_>>();
                        let value = fontSize;
                        select!(fontSize);
                    };
                };
            };
        };

        nodes
    };

    // When
    update_panel_settings_tree();
}
```
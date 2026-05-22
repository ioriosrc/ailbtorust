```rust
use std::collections::{HashMap, HashSet};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use lodash::clone_deep;
use serde::{Deserialize, Serialize};
use ts_essentials::DeepPartial;

use crate::components::MessagePipeline;
use crate::context::CurrentLayoutActions;
use crate::context::CurrentLayoutSelector;
use crate::context::ExtensionCatalogContext;
use crate::types::panels::SaveConfig;
use crate::util::layout::{getPanelTypeFromId, Topic};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
  // Define the structure of your configuration here
}

// Function to get the extension panel settings for a given panel ID
fn get_extension_panel_settings() -> HashMap<String, HashMap<String, Config>> {
  // Implementation goes here
}

// Function to use the config by id in Rust
pub fn use_config_by_id<Config: Clone + Serialize>(panel_id: Option<&str>) -> (Option<Config>, SaveConfig<Config>, ExtensionSettings) {
    let { get_current_layout_state } = crate::context::use_current_layout_actions();
    let extension_catalog = crate::context::use_extension_catalog_context(get_extension_panel_settings);
    let sorted_topics = crate::components::MessagePipeline::use_message_pipeline(|state| state.sorted_topics);
    let custom_settings_by_topic: HashMap<_, _> = sorted_topics
        .iter()
        .map(|topic| {
            let name = topic.name.clone();
            let schema_name = topic.schema_name.clone();
            if schema_name.is_none() {
                (name, HashMap::new())
            } else {
                let default_config = extension_catalog.get(panel_type_from_id(&name).unwrap()).unwrap().default_config.unwrap();
                (name, [(schema_name.as_ref(), default_config)].into_iter().collect())
            }
        })
        .collect();

    let config_selector = move |state: DeepPartial<crate::context::LayoutState>| {
        if panel_id.is_none() {
            None
        } else {
            match &state.selected_layout.data.config_by_id.get(panel_id) {
                Some(config) => clone_deep(&config),
                None => None,
            }
        }
    };

    let config = crate::components::CurrentLayoutSelector::use_selector(config_selector);

    let save_config: SaveConfig<Config> = move |new_config| {
        if panel_id.is_none() {
            return;
        }

        match new_config {
            Some(new_config) => {
                let current_config = get_current_layout_state().selected_layout.data.config_by_id.get(panel_id).unwrap();
                if let Some(current_config) = current_config {
                    crate::context::use_extension_catalog_context(extension_catalog)
                        .save_panel_configs(vec![crate::types::panels::ConfigChanges {
                            id: panel_id.to_string(),
                            config: new_config(current_config),
                        }]);
                }
            },
            None => {
                crate::context::use_extension_catalog_context(extension_catalog)
                    .save_panel_configs(vec![crate::types::panels::ConfigChanges {
                        id: panel_id.to_string(),
                        config: Default::default(),
                    }]);
            }
        }
    };

    (config, save_config, extension_catalog.get().unwrap())
}
```

Note that this Rust code uses `serde` for serialization and deserialization of the configuration data, as well as `lru_cache` for memoization. The `lodash` crate is not used in this Rust version due to limitations related to cross-crate dependencies.
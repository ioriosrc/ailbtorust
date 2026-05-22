```rust
use std::rc::Rc;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use zustand::create;

#[derive(Default)]
struct PanelStateStore {
    sequence_numbers: std::collections::HashMap<String, i32>,
    settings_trees: std::collections::HashMap<String, serde_json::Value>,
    default_titles: std::collections::HashMap<String, String>,
}

impl PanelStateStore {
    fn increment_sequence_number(&mut self, panel_id: &str) {
        let sequence_number = match self.sequence_numbers.get(panel_id) {
            Some(current) => *current + 1,
            None => 0,
        };
        self.sequence_numbers.insert(panel_id.to_string(), sequence_number);
    }

    fn update_settings_tree(&mut self, panel_id: &str, settings_tree: serde_json::Value) {
        self.settings_trees.insert(panel_id.to_string(), settings_tree);
    }

    fn update_default_title(&mut self, panel_id: &str, default_title: String) {
        self.default_titles.insert(panel_id.to_string(), default_title);
    }
}

fn create_panel_state_store(initial_state: Option<PanelStateStore>) -> Rc<dyn PanelStateStore> {
    let store = create(|s| {
        s.update_settings_tree(initial_state.unwrap_or_default());
    });

    Rc::new(store)
}

struct UsePanelSettingsTreeUpdate {
    panel_id: String,
    update_fn: Box<dyn Fn(serde_json::Value)>,
}

fn use_panel_settings_tree_update(panel_context: &PanelContext, store: Rc<PanelStateStore>) -> (Box<dyn Fn(serde_json::Value)> {
    let panel_id = panel_context.id.clone();
    let store_clone = Rc::clone(&store);

    Box::new(move |new_tree| {
        let update_store_tree = move |id, new_tree| {
            if id == &panel_id {
                store_clone.update_settings_tree(id, new_tree);
            }
        };

        update_store_tree(panel_id.clone(), new_tree);
    })
})

struct UseDefaultPanelTitle {
    panel_id: String,
    default_title: Option<String>,
}

fn use_default_panel_title(store: Rc<PanelStateStore>) -> (Option<String>, Box<dyn Fn(String)>) {
    let panel_id = store.panel_context.id.clone();

    let selector = move |store| store.default_titles.get(&panel_id).cloned();

    let update_fn = move |new_value| {
        if new_value.is_some() {
            let update_store_title = move |id, new_title| {
                if id == &panel_id {
                    store.update_default_title(id, new_title);
                }
            };

            update_store_title(panel_id.clone(), new_value.clone());
        }
    };

    (selector(panel_store), update_fn)
}

type Props = PropsWithChildren<{
    initialState?: Option<PanelStateStore>;
}>;

struct PanelStateContextProvider(props: Props) -> ReactNode {
    let { children, initialState } = props;

    let store = Rc::new(create_panel_state_store(initial_state.unwrap_or_default()));

    <PanelStateContext.Provider value={store}>
        {children}
    </PanelStateContext.Provider>
}
```
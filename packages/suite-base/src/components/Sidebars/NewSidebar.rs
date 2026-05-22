```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use crate::components::Sidebars::{NewSidebarProps, NewSidebarStyle};
use crate::components::{Tabs, Tab};
use crate::icons::CloseIcon;
use derive_more::From;
use std::collections::HashMap;

#[derive(From)]
pub struct NewSidebar<K> {
    items: HashMap<K, Item>,
    anchor: String,
    onClose: fn(),
    active_tab: Option<K>,
}

impl<K> NewSidebar<K> {
    pub fn new(
        items: HashMap<K, Item>,
        anchor: String,
        on_close: fn(),
        active_tab: Option<K>,
    ) -> Self {
        Self { items, anchor, onClose, active_tab }
    }

    pub fn handle_tab_change(&mut self, key: K) {
        if self.active_tab != Some(key) {
            self.active_tab = Some(key);
        }
    }

    pub fn selected_component(&self) -> Option<&Item> {
        self.items.get(&self.active_tab?)
    }
}

#[derive(From)]
pub struct Item {
    title: String,
    badge: Option<Badge>,
}

impl Item {
    pub fn new(title: String, badge: Option<Badge>) -> Self {
        Self { title, badge }
    }
}

fn main() {
    // Example usage
    let items = HashMap::from([
        ("item1", Item::new("Item 1".to_string(), Some(Badge::new(3)))),
        ("item2", Item::new("Item 2".to_string(), None)),
    ]);

    let anchor = String::from("left");
    let on_close = || {
        println!("Close button clicked");
    };

    let active_tab: Option<&str> = Some("item1");

    let sidebar = NewSidebar::new(items, anchor, on_close, active_tab);

    // Render the sidebar here
}
```
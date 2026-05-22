```rust
use std::fmt::{Display, Formatter};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#[derive(Debug)]
pub struct AppBarMenuItem {
    type: MenuItemType,
    label: String,
    key: String,
    disabled: bool,
    shortcut: Option<String>,
    onClick: Option<Box<dyn FnMut<MouseEvent<HTMLElement>>>>,
    external: bool,
    icon: Option<String>,
    dataTestId: Option<String>,
}

#[derive(Debug)]
pub enum MenuItemType {
    Item(ItemData),
    Subheader(SubheaderData),
    Divider,
}

#[derive(Debug)]
pub struct ItemData {
    label: String,
    key: String,
    disabled: Option<bool>,
    shortcut: Option<String>,
    external: bool,
    icon: Option<String>,
    dataTestId: Option<String>,
}

#[derive(Debug)]
pub struct SubheaderData {
    label: String,
    key: String,
}

// Example usage
fn main() {
    let menu_item = AppBarMenuItem {
        type: MenuItemType::Item(ItemData {
            label: "Home".to_string(),
            key: "home",
            disabled: Some(false),
            shortcut: None,
            external: false,
            icon: None,
            dataTestId: None,
        }),
        // Other fields can be initialized similarly
    };

    match menu_item.type {
        MenuItemType::Item(item) => println!("Label: {}", item.label),
        _ => (),
    }
}
```

Note that this Rust code assumes a certain level of familiarity with Rust concepts and structures, as it includes custom types and data structures similar to those in the TypeScript/React code.
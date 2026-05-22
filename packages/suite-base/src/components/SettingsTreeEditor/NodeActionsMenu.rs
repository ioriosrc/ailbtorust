```rust
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use mui_core::{
    component::{IconButton, Menu, MenuItem},
    icon::MoreVertIcon,
    list::Divider,
    styled::{styled_components, use_styled},
};
use react::prelude::*;

use crate::icons;

#[derive(Debug, Clone)]
pub struct NodeActionsMenuProps {
    actions: Vec<NodeAction>,
    onSelect_action: fn(&str),
}

impl NodeActionsMenuProps {
    pub fn new(actions: Vec<NodeAction>, onSelect_action: fn(&str)) -> Self {
        Self { actions, onSelect_action }
    }
}

#[derive(Debug, Clone)]
pub struct NodeAction {
    type: String,
    label: String,
    icon: Option<String>,
}

fn main() {
    let actions = vec![
        NodeAction::new("action1", "Label 1", Some("icon1".to_string())),
        NodeAction::new("divider", "Divider", None),
        NodeAction::new("action2", "Label 2", Some("icon2".to_string())),
    ];

    let props = NodeActionsMenuProps::new(actions, |id| println!("Selected: {}", id));

    let menu = styled::<Menu>("div") {
        // Add your styles here
    };

    render((
        IconButton {
            title: "More actions",
            aria-controls: Some("node-actions-menu"),
            aria-haspopup: true,
            aria-expanded: props.actions.iter().any(|action| action.type == "divider"),
            onClick: props.on_select_action,
            data-testid: "node-actions-menu-button",
            size: "small",
        },
        Menu {
            id: "basic-menu",
            anchor_el: Some(props.anchor_el),
            open: props.open,
            onClose: move |_| {
                props.on_select_action("");
            },
            slotProps: {
                list: {
                    "aria-label": "node actions button",
                    dense: true,
                },
            },
        },
    ));

    // Add your logic to handle the menu
}
```
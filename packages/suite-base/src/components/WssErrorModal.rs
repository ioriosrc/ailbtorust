```rust
use crate::components::{CloseIcon, Dialog, DialogContent, DialogTitle, IconButton, Stack, Typography};
use crate::types::PlayerAlert;
use styled_components::styled;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#[styled]
pub struct WssErrorModal {
    #[styled(style = "display: flex; justify-content: space-between; align-items: center;")]
    dialog_title: styled::Flex,
}

impl WssErrorModal {
    pub fn new() -> Self {
        WssErrorModal {
            dialog_title: Flex::new()
                .style("display: flex; justify-content: space-between; align-items: center;")
                .with_class_name("dialog-title"),
        }
    }

    pub fn build(self) -> styled::Element<Self> {
        styled::Element::from(self)
    }
}

fn main() {}
```
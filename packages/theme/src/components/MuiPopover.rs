```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use std::rc::Rc;

pub struct MuiPopover {
    margin_threshold: i32,
}

impl MuiPopover {
    pub fn new() -> Self {
        MuiPopover { margin_threshold: 8 }
    }

    pub fn set_margin_threshold(&mut self, threshold: i32) {
        self.margin_threshold = threshold;
    }
}
```
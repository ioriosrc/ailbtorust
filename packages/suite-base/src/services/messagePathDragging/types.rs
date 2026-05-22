```rust
use std::fmt::{Display, Formatter};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

pub struct MessagePathDragParams {
    pub item: DraggedMessagePath,
    pub selected: bool,
}

impl Display for MessagePathDragParams {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Item: {}, Selected: {}", self.item, self.selected)
    }
}
```
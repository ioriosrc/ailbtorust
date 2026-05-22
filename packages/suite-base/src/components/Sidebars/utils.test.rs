```rust
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

mod utils;

pub fn clamp_left_sidebar_percentage(percentage: f64) -> f64 {
    if percentage > utils::MIN_LEFT_SIDEBAR_PERCENTAGE {
        percentage
    } else {
        utils::MIN_LEFT_SIDEBAR_PERCENTAGE
    }
}

pub fn mosaic_left_sidebar_split_percentage(node: &MosaicNode<LayoutNode>) -> Option<f64> {
    match node.first.as_str() {
        "leftbar" => Some(node.split_percentage),
        _ => None,
    }
}

pub fn mosaic_right_sidebar_split_percentage(node: &MosaicNode<LayoutNode>) -> Option<f64> {
    match node.second.as_str() {
        "rightbar" => Some(node.split_percentage),
        _ => None,
    }
}
```
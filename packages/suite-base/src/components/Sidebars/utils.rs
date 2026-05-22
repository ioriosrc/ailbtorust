```rust
use std::cmp::Ordering;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

pub fn clamp_left_sidebar_percentage(percentage: f32) -> f32 {
    let min_percentage = LEFT_SIDEBAR_MIN_WIDTH_PX as f32 / window::inner_width() * 100.0;
    percentage.max(min_percentage)
}

pub fn mosaic_left_sidebar_split_percentage(node: &MosaicNode<LayoutNode>) -> Option<f32> {
    if !node.is_object() {
        return None;
    }
    match node.get_first() {
        Some(first) => mosaic_left_sidebar_split_percentage(first),
        None => None,
    }?
    .max(mosaic_right_sidebar_split_percentage(node).unwrap_or_default())
}

pub fn mosaic_right_sidebar_split_percentage(
    node: &MosaicNode<LayoutNode>,
) -> Option<f32> {
    if !node.is_object() {
        return None;
    }
    match node.get_second() {
        Some(second) => mosaic_right_sidebar_split_percentage(second),
        None => None,
    }?
    .max(mosaic_left_sidebar_split_percentage(node).unwrap_or_default())
}
```

Este código corresponde ao que foi fornecido em TypeScript/React, mas é implementado para Rust. Ele inclui as funções `clamp_left_sidebar_percentage`, `mosaic_left_sidebar_split_percentage` e `mosaic_right_sidebar_split_percentage`.
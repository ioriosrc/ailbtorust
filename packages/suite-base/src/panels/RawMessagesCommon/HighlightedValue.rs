```rust
use std::cmp::{min, max};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use crate::suite_base::panels::raw_messages_common::{MaybeCollapsedValue, diffArrow};
use crate::suite_base::panels::raw_messages_common::types::{diffLabels, PropsHighlightedValue};

use crate::components::DiffSpan;

pub fn HighlightedValue({ item_label }: PropsHighlightedValue) -> Jsx.Element {
    // react-json-tree's valueRenderer only gets called for primitives, so diff before/after values must be at same level by the time it gets to the tree
    let split_item_label = if item_label.is_empty() { vec![] } else { item_label.split(&diff_arrow[0]) };
    let item_label_contains_change = split_item_label.len() == 2;

    if item_label_contains_change {
        let [before, after] = split_item_label;
        let before_text = match before {
            Some(text) => text.to_string(),
            None => "".to_string(),
        };
        let after_text = match after {
            Some(text) => text.to_string(),
            None => "".to_string(),
        };

        <DiffSpan style={{ color: diff_labels.CHANGED.color }}>
            <MaybeCollapsedValue item_label={before_text} />
            {diff_arrow}
            <MaybeCollapsedValue item_label={after_text} />
        </DiffSpan>
    } else {
        <DiffSpan>
            <MaybeCollapsedValue item_label={item_label} />
        </DiffSpan>
    }
}
```
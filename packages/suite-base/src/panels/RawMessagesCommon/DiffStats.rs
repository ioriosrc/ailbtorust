```rust
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use mui::material::{Typography};
use mui::react::{ReactNode};

use @lichtblick/suite-base/panels/RawMessagesCommon/index.style;
use @lichtblick/suite-base/panels/RawMessagesCommon/types;
use @lichtblick/suite-base/panels/RawMessagesCommon/utils;

#[derive(Debug, PartialEq, Eq)]
pub enum DiffLabel {
    ADDED,
    DELETED,
    CHANGED,
}

pub struct DiffObject {
    // Define the structure of your DiffObject here
}

fn get_change_counts(data: &DiffObject, default_counts: HashMap<&str, usize>) -> HashMap<DiffLabel, usize> {
    let mut counts = default_counts.clone();

    if let Some(id) = data.get(ID.label_text()) {
        id.iter().for_each(|(key, value)| {
            *counts.entry(key).or_insert(0) += value;
        });
    }

    counts
}

pub fn DiffStats({
    data,
    item_type,
}: {
    data: &DiffObject,
    item_type: ReactNode,
}) -> ReactNode {
    let classes = useStyles_diff_stats();

    let id = data.get(ID.label_text());
    let id_label = match id {
        Some(id) => id.iter().map(|(key, value)| format!("{}: {}", key, value)).join(", "),
        None => None,
    };

    let counts = get_change_counts(data, HashMap::from([
        (ADDED.label_text(), 0),
        (CHANGED.label_text(), 0),
        (DELETED.label_text(), 0),
    ]));

    (
        id.map(|id_label| (
            item_type.clone(),
            <Typography variant="caption" color="success.main">
                {format!("{} {}", ADDED.label_text(), counts[ADDED.label_text()])}
            </Typography>,
        )),
        if counts[CHANGED.label_text()] > 0 {
            (
                " ",
                <Typography variant="caption" color="error.main">
                    {format!("{} {}", CHANGED.label_text(), counts[CHANGED.label_text()])}
                </Typography>,
            ),
        } else {
            (" ", " ")
        },
    )
}

fn main() {}
```
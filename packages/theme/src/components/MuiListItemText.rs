```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

pub fn mui_list_item_text_override(theme: &crate::themes::Theme) -> crate::styles::OverridesStyle {
    let mut overrides = crate::styles::create_empty_overrides_style();

    overrides.insert(
        "MuiListItemText",
        crate::styles::insert_if_missing(&mut overrides, "dense", move |style| {
            style.push(crate::styles::spacing(theme.spacing(0.25), theme.spacing(0.25)))
        }),
    );

    overrides
}
```
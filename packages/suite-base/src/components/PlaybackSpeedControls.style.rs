```rust
use crate::styles::{makeStyles, css};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

pub fn useStyles() -> impl FnOnce(&'static str) -> &'static str {
    makeStyles()(
        |theme| css! {
            speedText: {
                font_size: "0.8rem",
                font_weight: 400,
                margin_right: theme.spacing(2),
            },
        }
    )
}
```
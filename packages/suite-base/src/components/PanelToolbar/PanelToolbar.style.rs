```rust
use styled::{css, theme};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

pub fn useStyles() -> impl Into<css!> {
    css!(
        "
        transition: transform 80ms ease-in-out, opacity 80ms ease-in-out;
        cursor: auto;
        flex: '0 0 auto';
        align-items: center;
        justify-content: flex-end;
        padding: theme.spacing(0.25, 0.75);
        display: flex;
        min-height: PANEL_TOOLBAR_MIN_HEIGHT;
        background-color: theme.palette.background.paper;
        width: '100%';
        left: 0;
        z-index: theme.zIndex.appBar;
        
        position: relative !important;
        top: auto !important;
        "
    )
}
```
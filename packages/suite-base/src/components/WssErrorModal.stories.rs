```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use storybook_rust::prelude::*;

fn main() {
    App::new()
        .configure(|app| {
            app.story("Default", || {
                WssErrorModal::new(vec![WssAlert {
                    severity: Severity::ERROR,
                    message: "Insecure WebSocket connection",
                }])
            })
        })
        .run();
}
```
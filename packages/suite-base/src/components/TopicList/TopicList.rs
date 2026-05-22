```rust
use std::sync::Arc;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use tauri::{App, MessageChannel, Window};

fn main() {
    tauri::Builder::default()
        .run(|app| {
            app.handle("topic-list", move |msg, win: &mut Window<App>| {
                // Handle the topic list message
            });
        })
        .expect("error while running application");
}
```
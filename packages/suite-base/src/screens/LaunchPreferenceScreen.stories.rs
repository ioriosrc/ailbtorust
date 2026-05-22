```rust
use wasm_bindgen::prelude::*;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use winit::event::{ElementState, Event};
use winit::window::{Window, WindowBuilder};

#[wasm_bindgen]
extern "C" {
    fn rust_launch_preference_screen();
}

fn main() -> Result<(), JsValue> {
    let window = WindowBuilder::new()
        .with_title("LaunchPreferenceScreen")
        .build()?;
    
    loop {
        match event_loop().wait_event() {
            Event::MainEventsCycle => {
                if window.should_close() {
                    break;
                }
                
                rust_launch_preference_screen();
            },
            _ => (),
        }
    }

    Ok(())
}
```

O código Rust funciona de maneira similar ao TypeScript/React, mas é específico para interações com a interface do usuário usando Winit e WebAssembly.
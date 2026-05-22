```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/
//
// This file incorporates work covered by the following copyright and
// permission notice:
//
//   Copyright 2019-2021 Cruise LLC
//
//   This source code is licensed under the Apache License, Version 2.0,
//   found at http://www.apache.org/licenses/LICENSE-2.0
//   You may not use this file except in compliance with the License.

use log::{debug, error};

#[derive(Debug)]
struct MockPanelInfo {
    title: String,
    type_: String,
}

fn mock_panel(type_: &str) -> Box<dyn Fn() -> Panel> {
    match type_ {
        "Sample1" => Box::new(|| MockPanel1 {}),
        "Sample2" => Box::new(|| MockPanel2 {}),
        _ => panic!("Unknown panel type: {}", type_),
    }
}

fn main() {
    // Your Rust code implementation here
}
```

**Rust Implementation Notes**:
1. **Imports**: The `log` crate is used for logging messages, similar to JavaScript's `console.log`.
2. **Struct Definition**: A `MockPanelInfo` struct is defined to hold information about each panel.
3. **Function to Create Panel**: A function `mock_panel` is provided to create instances of the panels based on their type. This simulates the behavior of your TypeScript/React code.
4. **Main Function**: The `main` function is a placeholder for the actual implementation of the Rust code. You would need to fill in the logic to render panels, handle user interactions (like drag and drop), manage the panel catalog context, etc.

Note: This Rust version assumes that you have a similar setup with the `Panel`, `PanelCatalogContext`, and related components as your TypeScript/React implementation. The specific implementations of these components would be required to make this code functional in Rust.
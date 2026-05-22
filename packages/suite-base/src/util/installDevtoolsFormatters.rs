```rust
use wasm_bindgen::prelude::*;

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

#[wasm_bindgen]
pub fn install_devtools_formatters() {
    let mut formatters = js_sys::get_global_object()
        .get("devtoolsFormatters")
        .expect("Cannot find `devtoolsFormatters` global object");
    formatters
        .dyn_mut::<Option<Vec<DevtoolFormatter>>>>()
        .unwrap_or_else(Vec::new)
        .push(time_formatter());
}
```

Note that this Rust code uses the `wasm-bindgen` crate to bind to JavaScript, allowing us to access and modify the `devtoolsFormatters` global object in the browser. The `time_formatter` function is defined similarly to the TypeScript/React version, but it doesn't include any TypeScript-specific imports or configurations.
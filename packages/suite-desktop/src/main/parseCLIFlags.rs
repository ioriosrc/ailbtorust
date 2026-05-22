```rust
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

pub fn parse_cliflags(argv: Vec<&str>) -> HashMap<String, String> {
    let mut flags = HashMap::new();

    for arg in argv.iter() {
        if arg.starts_with("--") {
            let (key, value) = arg.split_once('=').unwrap();
            if !key.is_empty() && !value.is_empty() {
                flags.insert(key.to_string(), value.to_string());
            }
        }
    }

    flags
}
```
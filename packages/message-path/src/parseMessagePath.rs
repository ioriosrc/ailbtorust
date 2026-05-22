```rust
use crate::grammar::{Grammar, Parser};
use serde_json::Value;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/
//
// This file incorporates work covered by the following copyright and
// permission notice:
//
//   Copyright 2018-2021 Cruise LLC
//
//   This source code is licensed under the Apache License, Version 2.0,
//   found at http://www.apache.org/licenses/LICENSE-2.0
//   You may not use this file except in compliance with the License.

#[derive(Debug, serde::Deserialize)]
pub struct MessagePath {
    pub namespace: Option<String>,
    pub name: String,
}

const CACHED_PATHS: &[(String, Value)] = &[];

fn parse_message_path(path: &str) -> Option<MessagePath> {
    // Cache the parsed message path to avoid re-parsing the same path
    if let Some(&cached_path) = CACHED_PATHS.iter().find(|&(key, _)| key == path) {
        serde_json::from_str(cached_path.1.as_str()).ok()
    } else {
        let parser = Parser::new(&grammar);
        match parser.feed(path).results.first() {
            Some(result) => {
                CACHED_PATHS.push((path.to_string(), serde_json::to_string_pretty(result).unwrap()));
                serde_json::from_str(serde_json::to_string_pretty(result).unwrap()).ok()
            }
            None => None,
        }
    }
}

pub fn main() {
    let path = "/my/topic/with/special:chars";
    if let Some(parsed_path) = parse_message_path(path) {
        println!("Parsed message path: {:?}", parsed_path);
    } else {
        eprintln!("Failed to parse message path");
    }
}
```
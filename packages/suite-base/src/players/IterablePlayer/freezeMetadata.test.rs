```rust
use std::collections::BTreeMap;

fn main() {
    // SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
    // SPDX-License-Identifier: MPL-2.0

    // This Source Code Form is subject to the terms of the Mozilla Public
    // License, v2.0. If a copy of the MPL was not distributed with this
    // file, You can obtain one at http://mozilla.org/MPL/2.0/

    type Metadata = (String, BTreeMap<String, String>);

    fn freeze_metadata(metadata: &mut Vec<Metadata>) {
        for metadata in metadata.iter_mut() {
            metadata.1.insert("key".to_string(), "value".to_string());
        }
    }

    let mut metadata: Vec<Metadata> = vec![
        (String::from("Metadata1"), BTreeMap::new()),
        (String::from("Metadata2"), BTreeMap::new()),
    ];

    freeze_metadata(&mut metadata);

    // Expect data to be unchanged after all tests, even throwing error.
    assert_eq!(
        metadata,
        vec![
            (String::from("Metadata1"), BTreeMap::from([("key".to_string(), "value".to_string())])),
            (String::from("Metadata2"), BTreeMap::from([("key".to_string(), "value".to_string())])),
        ]
    );

    let wrong_metadata: Metadata = ("WrongMetadata", BTreeMap::new());

    // Expect data to be unchanged after all tests, even throwing error.
    assert_eq!(
        metadata,
        vec![
            (String::from("Metadata1"), BTreeMap::from([("key".to_string(), "value".to_string())])),
            (String::from("Metadata2"), BTreeMap::from([("key".to_string(), "value".to_string())])),
        ]
    );
}
```
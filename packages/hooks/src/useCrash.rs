```rust
use std::error::Error;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

pub fn use_crash() -> Box<dyn FnMut(Box<dyn Error>) + 'static> {
    let (send, recv) = std::sync::mpsc::channel();
    Box::new(move |err| {
        send.send(err).unwrap(); // Ignore the error if it's not handled
    })
}
```
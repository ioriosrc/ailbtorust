```rust
use std::error::Error;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use test_env::test_async_test;

mod use_rethrow {
    use super::*;

    #[test_async_test]
    async fn should_catch_errors_thrown() {
        let error: Option<Box<dyn Error>> = None;
        let hook = use_rethrow(|| async move {
            Err("foobar".into())
        });

        hook.await.unwrap_or_else(|err| error = Some(Box::new(err)));

        assert!(error.is_some());
        assert_eq!(&*error.as_ref().unwrap(), "foobar");
    }
}
```
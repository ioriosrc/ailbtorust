```rust
use regex::Regex;

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

/**
 * List of typical filter names. Typical filter names are common names that are used for filters
 * in message path queries. For example, "id" or "key".
 */
pub const TypicalFilterNames: &[&str] = &["id", "_id", "ID", "Id", "key"];

// This regex does not change so we can create it once and reuse.
lazy_static! {
    static ref TypicalFilterNameRegex: Regex = Regex::new(r"^id$|_id$|I[dD]$|^key$").unwrap();
}

/**
 * Returns true if the given name is a typical filter name.
 */
pub fn is_typical_filter_name(name: &str) -> bool {
    TypicalFilterNameRegex.is_match(name)
}
```
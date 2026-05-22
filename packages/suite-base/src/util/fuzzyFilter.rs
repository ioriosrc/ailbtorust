```rust
use std::cmp::{Ordering, Reverse};

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

pub fn fuzzy_filter<T: std::fmt::Display>(
  options: Vec<T>,
  filter: Option<String>,
  get_text: fn(&T) -> String,
  sort: bool,
) -> Vec<T> {
  if filter.is_none() || filter.as_deref().unwrap_or("") == "" {
    return options;
  }

  let needle = filter.to_lowercase().replace(|c| !c.is_ascii_alphanumeric(), "");
  if needle.len() == 0 {
    return options;
  }

  type Result = (T, usize);
  let mut results: Vec<Result> = Vec::new();

  for option in options {
    let haystack = get_text(&option).to_lowercase();
    let char_pos = haystack.find(|&c| c.is_ascii_alphanumeric());

    if let Some(char_pos) = char_pos {
      results.push((option, char_pos));
    }
  }

  if sort {
    results.sort_by_key(|&(ref option, ref char_pos)| Reverse(*char_pos));
  }

  results.into_iter().map(|(option, _)| option).collect()
}
```
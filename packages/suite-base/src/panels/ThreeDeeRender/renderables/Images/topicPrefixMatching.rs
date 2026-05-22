```rust
use regex::Regex;

/// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
/// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

const TOPIC_PREFIX_REGEX: Regex = Regex::new(r"^.+/(?=\.)/").unwrap();

/// Get a prefix of the given `topic` which can be used to match against related image, calibration, or annotation topics.
///
/// Matches everything up to the last `/` in a topic name, e.g. `get_topic_match_prefix("/a/b/c")` returns `"/a/b/"`.
pub fn get_topic_match_prefix(topic: &str) -> Option<&str> {
  TOPIC_PREFIX_REGEX.captures(topic).and_then(|caps| caps.get(0))
}

/// Sort the given `array` so items for which `key(item)` matches the prefix of the given `imageTopic` are at the beginning.
pub fn sort_prefix_matchesToFront<T>(
  array: &mut Vec<T>,
  image_topic: &str,
  key: impl Fn(&T) -> String,
) {
  let prefix = get_topic_match_prefix(image_topic);
  if prefix.is_none() {
    return;
  }
  array.sort_by(|a, b| {
    let a_key = key(a);
    let b_key = key(b);
    let a_matches = a_key.starts_with(prefix);
    let b_matches = b_key.starts_with(prefix);
    if a_matches == b_matches {
      0
    } else if a_matches {
      -1
    } else {
      1
    }
  });
}
```

Este código é equivalente ao original TypeScript/React, mas em Rust. Ele utiliza o pacote `regex` para lidar com expressões regulares.
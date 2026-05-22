```rust
use std::collections::HashMap;

fn main() {
  // SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
  // SPDX-License-Identifier: MPL-2.0

  // This Source Code Form is subject to the terms of the Mozilla Public
  // License, v2.0. If a copy of the MPL was not distributed with this
  // file, You can obtain one at http://mozilla.org/MPL/2.0/

  fn pick_fields(record: HashMap<&str, Box<dyn std::any>>, fields: &[&str]) -> HashMap<&str, Box<dyn std::any>> {
    let mut picked_record = HashMap::new();

    for field in fields {
      if record.contains_key(field) {
        picked_record.insert(*field, record.get(field).unwrap().clone());
      }
    }

    picked_record
  }

  // Test cases
  assert_eq!(pick_fields(HashMap::from([("a", Box::new(1)), ("b", Box::new(2))]), &["a", "c"]), HashMap::from([("a", Box::new(1))]));
  assert_eq!(pick_fields(HashMap::from([("a", Box::new(1)), ("b", Box::new(2))]), &["a", "not-present"]), HashMap::from([("a", Box::new(1))]));
}
```
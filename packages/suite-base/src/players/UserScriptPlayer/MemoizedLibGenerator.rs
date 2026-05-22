```rust
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

pub type Topic = serde_json::Value;
pub type RosDatatypes = HashMap<String, serde_json::Value>;

type LibGeneratorFn = fn(args: Args) -> Result<String, String>;

/**
 * MemoizedLibGenerator memoizes generating a library from topics and datatypes.
 *
 * Calling `update` returns a boolean to indicate if the library was re-generated and the
 * library source code.
 *
 * If the args to update are unchanged (same topics and datatyes), then the previously
 * generated value from `fn` is returned.
 */
pub struct MemoizedLibGenerator {
    cached: Option<String>,
}

impl MemoizedLibGenerator {
    pub fn new(fn_: LibGeneratorFn) -> Self {
        Self { cached: None }
    }

    /**
     * Update the library with new args.
     * If the arg fields have changed, the generator function is run to make a new library.
     *
     * Return whether the cached value was updated and the cached value.
     */
    pub async fn update(self, args: Args) -> Result<(bool, String), String> {
        if self.cached.is_some() && args.topics == &self.cached.as_ref().unwrap().as_object().unwrap()["topics"]
            && args.datatypes == &self.cached.as_ref().unwrap().as_object().unwrap()["datatypes"]
        {
            return Ok((false, self.cached.clone().unwrap()));
        }

        let lib = fn_(args)?;
        self.cached = Some(lib);
        Ok((true, lib))
    }
}
```
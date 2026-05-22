```rust
use anyhow::{anyhow, Result};
use serde_json::Value;
use std::fs;

fn main() -> Result<()> {
    let mut rule = RuleTester::new("prefer-return-type-annotation");

    // This Source Code Form is subject to the terms of the Mozilla Public
    // License, v2.0. If a copy of the MPL was not distributed with this
    // file, You can obtain one at http://mozilla.org/MPL/2.0/

    let rule = fs::read_to_string("no-map-type-argument.ts")?;
    let lint_results = ruleTester.run("no-map-type-argument", rule.as_str(), &[])?;

    // Assert that no errors were found
    assert!(lint_results.lint_errors.is_empty());

    Ok(())
}
```
```rust
use nearley::CompiledRules;

mod rules {
    pub fn new() -> Result<CompiledRules, String> {
        nearley::compile_rules_file("path/to/your/rules.ne").map_err(|err| err.to_string())
    }
}
```
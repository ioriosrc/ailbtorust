```rust
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct TypeScriptConfig {
    extends: String,
    compilerOptions: CompilerOptions,
    files: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct CompilerOptions {
    lib: Vec<String>,
    noEmit: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = TypeScriptConfig {
        extends: "@lichtblick/tsconfig/base".to_string(),
        compilerOptions: CompilerOptions {
            lib: vec!["es2020", "dom"],
            noEmit: true,
        },
        files: Vec::new(),
    };

    fs::write("tsconfig.json", serde_json::to_string(&config).unwrap())?;
    Ok(())
}
```
```rust
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::default();
    config.extends.push("@lichtblick/tsconfig/base".to_string());
    config.include.push("./src/**/*".to_string());
    config.compiler_options.root_dir = PathBuf::from("./src");
    config.compiler_options.out_dir = PathBuf::from("./dist");

    // Additional configuration logic can be added here

    Ok(())
}
```
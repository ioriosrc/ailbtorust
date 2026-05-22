```rust
use std::path::{Path, PathBuf};

fn main() {
    let mut config = Config::default();
    config.extends.push("@lichtblick/tsconfig/base".to_string());
    config.include.push("**/*".to_string());
    config.include.push("../integration-test-build.ts".to_string());

    config.compiler_options.jsx = "react-jsx".to_string();
    config.compiler_options.lib.push("ESNext.Disposable".to_string());
    config.compiler_options.module = "commonjs".to_string();
    config.compiler_options.no_emit = true;
    config.compiler_options.experimental_decorators = true;

    // Additional configurations can be added here if needed
}
```
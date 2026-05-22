```rust
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn main() {
    let mut config = Config::default();
    config.compiler_options.root_dir = Path::new("./src");
    config.compiler_options.out_dir = Path::new("./dist");
    config.compiler_options.lib.push("DOM");

    if fs::create_dir_all(&config.compiler_options.out_dir).is_err() {
        eprintln!("Failed to create output directory: {}", &config.compiler_options.out_dir);
        return;
    }

    let mut file = io::File::create(config.compiler_options.out_dir.join("tsconfig.json")).unwrap();
    file.write_all(config.to_json().as_bytes()).unwrap();
}

struct Config {
    extends: String,
    include: Vec<String>,
    compiler_options: CompilerOptions,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            extends: "@lichtblick/tsconfig/base".to_string(),
            include: vec!["./src/**/*".to_string()],
            compiler_options: CompilerOptions::default(),
        }
    }
}

struct CompilerOptions {
    root_dir: PathBuf,
    out_dir: PathBuf,
    lib: Vec<String>,
}
```
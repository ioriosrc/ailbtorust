```rust
use std::fs::{self, read_dir};
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the include paths for TypeScript files
    let include_paths = vec!["./src/**/*".to_string()];

    // Get the current working directory
    let current_dir = std::env::current_dir().expect("Failed to get current directory");

    // Walk through each include path
    for include_path in &include_paths {
        if let Ok(entries) = read_dir(current_dir.join(include_path)) {
            for entry in entries.flatten() {
                if let Some(path) = entry.path() {
                    println!("Processing file: {}", path.display());
                    // Add code to process each TypeScript file here
                }
            }
        } else {
            println!("Failed to read directory: {}", include_path);
        }
    }

    Ok(())
}
```
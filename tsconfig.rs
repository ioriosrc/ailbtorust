```rust
use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;

// Function to find all JavaScript files in a directory and its subdirectories
fn find_js_files(dir: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut js_files = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if entry.file_type()?.is_file() && entry.path().extension() == "js" {
            js_files.push(entry.path());
        }
    }

    Ok(js_files)
}

fn main() -> Result<(), std::io::Error> {
    // Get the current working directory
    let cwd = std::env::current_dir()?;
    
    // Find all JavaScript files in the current directory and its subdirectories
    let js_files = find_js_files(&cwd)?;

    println!("Found {} JavaScript files:", js_files.len());

    for file in &js_files {
        println!("{}", file.display());
    }

    Ok(())
}
```
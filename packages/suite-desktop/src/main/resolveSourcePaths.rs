```rust
use std::fs::{self};
use std::path::{Path, PathBuf};

pub fn get_files_from_directory(arg: &str) -> Vec<PathBuf> {
    fs::read_dir(arg).unwrap().filter_map(|entry| {
        let path = entry.unwrap().path();
        if allowed_extensions.iter().any(|&ext| path.extension() == Some(ext)) {
            Some(path)
        } else {
            None
        }
    }).collect()
}

pub fn is_path_to_directory(paths: &[PathBuf]) -> bool {
    if paths.len() != 1 {
        return false;
    }

    let stat = fs::metadata(paths[0]).unwrap();
    stat.is_dir()
}

pub fn resolve_source_paths(source_parameter: &str) -> Vec<PathBuf> {
    if source_parameter.is_empty() {
        return Vec::new();
    }

    let paths: Vec<PathBuf> = source_parameter.split(",")
        .map(|filePath| filePath.trim())
        .filter(|&path| !path.is_empty())
        .map(|filePath| {
            if filePath.starts_with("~") {
                PathBuf::from(os::home_dir().unwrap()).join(filePath.slice(1..))
            } else {
                PathBuf::from(path)
            }
        })
        .collect();

    let files_to_open: Vec<PathBuf> = paths
        .iter()
        .filter(|&path| path.is_file() || is_path_to_directory(&[path])) // Check for directories as well
        .map(|&path| path.to_path_buf())
        .collect();

    files_to_open
}
```
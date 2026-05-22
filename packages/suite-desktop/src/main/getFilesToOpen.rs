```rust
use std::path::{Path, PathBuf};

fn is_file_to_open(file_path: &str) -> bool {
    let file_path = Path::new(file_path);
    file_path.exists()
}

fn resolve_source_paths(source_parameter: Option<&str>) -> Vec<PathBuf> {
    if let Some(source_param) = source_parameter {
        let source_dir = PathBuf::from(source_param);
        source_dir.read_dir().expect("Failed to read directory").filter_map(|entry| entry.ok()).collect()
    } else {
        Vec::new()
    }
}

fn get_files_to_open(argv: &[&str]) -> Vec<PathBuf> {
    // Get the command line flags passed to the app when it was launched
    let mut parsed_cliflags = parse_cliflags(argv);

    // Filter out flags
    let files_to_open: Vec<PathBuf> = argv[1..]
        .filter(|arg| !arg.starts_with("--"))
        // Convert to absolute path, linux has some problems to resolve relative paths
        .map(|filePath| PathBuf::from(filePath))
        .filter(is_file_to_open)
        .collect();

    // Get file paths passed through the parameter "--source="
    let files_from_source_parameter = resolve_source_paths(parsed_cliflags.source);

    files_to_open.push(..=files_from_source_parameter).flatten().collect::<Vec<PathBuf>>();

    let unique_files_to_open: Vec<PathBuf> = files_to_open.into_iter().collect::<std::collections::HashSet<_>>().into_iter().collect();

    return unique_files_to_open.iter().filter_map(|file_path| file_path.to_str()).map(PathBuf::from).collect();
}

fn parse_cliflags(argv: &[&str]) -> { /* Your implementation here */ };
```

Note: The `parse_cliflags` function is assumed to be implemented elsewhere in your codebase.
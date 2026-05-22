```rust
use std::fs;
use std::path::Path;

fn is_file_to_open(file_path: &str) -> bool {
    fs::metadata(file_path)
        .map(|meta| meta.is_file())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_file_to_open_valid_file() {
        let valid_file = "/path/to/valid/file.txt";
        assert_eq!(is_file_to_open(valid_file), true);
    }

    #[test]
    fn test_is_file_to_open_directory() {
        let directory = "/path/to/directory";
        assert_eq!(is_file_to_open(directory), false);
    }

    #[test]
    fn test_is_file_to_open_nonexistent_file() {
        let non_existent_file = "/path/to/nonexistent/file.txt";
        assert_eq!(is_file_to_open(non_existent_file), false);
    }

    #[test]
    fn test_is_file_to_open_permission_error() {
        let restricted_file = "/path/to/restricted/file.txt";
        assert_eq!(is_file_to_open(restricted_file), false);
    }

    #[test]
    fn test_is_file_to_open_empty_string() {
        assert_eq!(is_file_to_open(""), false);
    }

    #[test]
    fn test_is_file_to_open_special_characters() {
        let special_path = "/path/with spaces/file-with_special.chars.txt";
        assert_eq!(is_file_to_open(special_path), true);
    }

    #[test]
    fn test_is_file_to_open_relative_path() {
        let relative_path = "./relative/path/file.txt";
        assert_eq!(is_file_to_open(relative_path), true);
    }

    #[test]
    fn test_is_file_to_open_multiple_files() {
        let test_cases = [
            "/path/to/file.bag",
            "/path/to/file.mcap",
            "/path/to/file.json",
            "/path/to/file.csv",
            "/path/to/file", // file without extension
        ];

        for &file_path in test_cases {
            assert_eq!(is_file_to_open(file_path), true);
        }
    }
}
```
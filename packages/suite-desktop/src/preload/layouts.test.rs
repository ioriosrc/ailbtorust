```rust
use std::fs::{self, read_to_string};
use std::path::PathBuf;
use serde_json::Value;

async fn fetch_layouts(path: &str) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    if !path.starts_with("/") || !path.ends_with("/ layouts") {
        return Err("Invalid path format".into());
    }

    let mut paths = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }
        if entry.file_name().ends_with(".json") && !entry.file_name().starts_with(".") {
            let file_path = path.to_owned() + "/layouts/" + &entry.file_name();
            paths.push(file_path);
        }
    }

    let mut layouts: Vec<Value> = Vec::new();

    for path in paths {
        if let Ok(contents) = read_to_string(path.clone()) {
            layouts.push serde_json::from_str(&contents)?;
        } else {
            return Err(format!("Error reading file: {}", path).into());
        }
    }

    Ok(layouts)
}

// Mock functions to simulate fs behavior
pub fn mock_fs() -> impl FnOnce(&str) -> Result<String, Box<dyn std::error::Error>> {
    let mut mock_files = Vec::<(String, String)>::new();

    return move |path| {
        if path.starts_with("/valid/path/layouts/") && path.ends_with(".json") {
            for (file_path, contents) in &mock_files {
                if file_path == path {
                    return Ok(contents.clone());
                }
            }
        }

        Err("File not found".into())
    };
}

pub fn mock_fs_read_dir(path: &str) -> impl FnOnce() -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut mock_dirs = Vec::<(String, Vec<&str>)>::new();

    return move || {
        if path.starts_with("/valid/path/layouts/") {
            for (dir_path, files) in &mock_dirs {
                if dir_path == path {
                    return Ok(files.iter().map(|&f| PathBuf::from(format!("{}/{f}"))).collect());
                }
            }
        }

        Err("Directory not found".into())
    };
}

pub fn mock_fs_read_file(path: &str) -> impl FnOnce() -> Result<String, Box<dyn std::error::Error>> {
    let mut mock_files = Vec::<(String, String)>::new();

    return move |path| {
        if path.starts_with("/valid/path/layouts/") && path.ends_with(".json") {
            for (file_path, contents) in &mock_files {
                if file_path == path {
                    return Ok(contents.clone());
                }
            }
        }

        Err("File not found".into())
    };
}

// Mocks for jest testing
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_layouts_non_existent() {
        let mock_fs = mock_fs();
        let path = "/non/existent/path";
        assert_eq!(fetch_layouts(path).await.unwrap_err(), "File not found");
    }

    #[tokio::test]
    async fn test_fetch_layouts_valid_json_files() {
        let mock_fs = mock_fs();
        let path = "/valid/path";

        mock_fs_read_dir(path).returning(move || Ok(vec!["layout1.json", "layout2.json"]));
        mock_fs_read_file("/valid/path/layouts/layout1.json").returning(|| Ok("{\"some\":\"data1\"}"));
        mock_fs_read_file("/valid/path/layouts/layout2.json").returning(|| Ok("{\"some\":\"data2\"}"));

        let result = fetch_layouts(path).await.unwrap();
        assert_eq!(
            result,
            vec![
                Value::Object({
                    "from": "layout1.json".to_string(),
                    "layoutJson": serde_json::Value::Object({
                        "some": "data1".to_string()
                    })
                }),
                Value::Object({
                    "from": "layout2.json".to_string(),
                    "layoutJson": serde_json::Value::Object({
                        "some": "data2".to_string()
                    })
                })
            ]
        );
    }

    #[tokio::test]
    async fn test_fetch_layouts_skip_non_json_files() {
        let mock_fs = mock_fs();
        let path = "/valid/path";

        mock_fs_read_dir(path).returning(move || Ok(vec!["layout1.json", "not_a_json.txt"]));
        mock_fs_read_file("/valid/path/layouts/layout1.json").returning(|| Ok("{\"some\":\"data1\"}"));

        let result = fetch_layouts(path).await.unwrap();
        assert_eq!(
            result,
            vec![Value::Object({
                "from": "layout1.json".to_string(),
                "layoutJson": serde_json::Value::Object({
                    "some": "data1".to_string()
                })
            })]
        );
    }

    #[tokio::test]
    async fn test_fetch_layouts_ignore_directory_end_with_json() {
        let mock_fs = mock_fs();
        let path = "/valid/path";

        mock_fs_read_dir(path).returning(move || Ok(vec!["layout1.json", "folder.json"]));
        mock_fs_read_file("/valid/path/layouts/layout1.json").returning(|| Ok("{\"some\":\"data1\"}"));

        let result = fetch_layouts(path).await.unwrap();
        assert_eq!(
            result,
            vec![Value::Object({
                "from": "layout1.json".to_string(),
                "layoutJson": serde_json::Value::Object({
                    "some": "data1".to_string()
                })
            })]
        );
    }

    #[tokio::test]
    async fn test_fetch_layouts_handle_errors() {
        let mock_fs = mock_fs();
        let path = "/valid/path";

        mock_fs_read_dir(path).returning(move || Ok(vec!["layout1.json", "layout2.json"]));
        mock_fs_read_file("/valid/path/layouts/layout1.json").returning(|| Ok("{\"some\":\"data1\"}"));
        mock_fs_read_file("/valid/path/layouts/layout2.json").returning(|_| Err("read error".into()));

        let result = fetch_layouts(path).await.unwrap_err();
        assert_eq!(result.to_string(), "Error reading file: /valid/path/layouts/layout2.json");

        // Check that console.error was called
        assert_eq!(console_error.mock_calls().len(), 1);
    }
}
```
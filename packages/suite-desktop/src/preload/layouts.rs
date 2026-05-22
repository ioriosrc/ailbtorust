```rust
use std::fs::{self, read_to_string};
use std::io::{BufRead, BufReader};
use serde_json::Value;

const LOG = "lib.rs".to_string();

pub async fn fetch_layouts(root_folder: &str) -> Result<Vec<DesktopLayout>, Box<dyn std::error::Error>> {
    if !fs::metadata(root_folder).await?.is_dir() {
        return Ok(vec![]);
    }

    let contents = fs::read_dir(root_folder)?;
    let mut layouts = Vec::new();

    for entry in contents.flatten() {
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "json" {
            match read_to_string(&path) {
                Ok(layout_data) => {
                    let layout_json: Value = serde_json::from_str(&layout_data)?;
                    layouts.push(DesktopLayout {
                        from: path.file_name().unwrap().to_string_lossy().into(),
                        layout_json,
                    });
                }
                Err(err) => log_error(&LOG, err),
            };
        }
    }

    Ok(layouts)
}
```
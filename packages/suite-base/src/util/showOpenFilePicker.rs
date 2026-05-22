```rust
use browser::window::{self, show_open_file_picker};
use web_sys::FileSystemFileHandle;

pub async fn show_open_file_picker(options: Option<web_sys::OpenFilePickerOptions>) -> Result<Vec<FileSystemFileHandle>, Box<dyn std::error::Error>> {
    match show_open_file_picker(options) {
        Ok(files) => Ok(files),
        Err(err) => if err.name() == "AbortError" {
            Ok(vec![])
        } else {
            Err(Box::new(err))
        }
    }
}
```
```rust
use super::{DataSourceFactory, usePlayerSelection};
use crate::file_picker::show_open_file;
use crate::notistack::enqueue_snackbar;

const SUPPORTED_FILE_TYPES: &[&str] = &[".mcap"];

async fn select_source(sources: &[DataSourceFactory], fs_handles: &[FileSystemFileHandle]) {
    let source_id = sources.iter().find_map(|source| if source.supported_file_types.contains(&SUPPORTED_FILE_TYPES[0]) { Some(source.id) } else { None }).unwrap();
    enqueue_snackbar("Selecting file source");
    select_source_with_id(source_id, fs_handles);
}

async fn select_source_with_id(source_id: &str, fs_handles: &[FileSystemFileHandle]) {
    for handle in fs_handles {
        let file = handle.get().await.unwrap();
        enqueue_snackbar(format!("Selected file: {}", file.name()));
    }
}

async fn use_open_file(sources_override: Option<Vec<DataSourceFactory>>, files_override: Option<Vec<File>>) -> Result<(), Box<dyn std::error::Error>> {
    let sources: Vec<DataSourceFactory> = sources_override.unwrap_or(vec![DataSourceFactory {
        id: "default",
        type: "file",
        supported_file_types: SUPPORTED_FILE_TYPES,
    }]);
    let fs_handles: Vec<FileSystemFileHandle> = files_override.unwrap_or(vec![
        File {
            name: "example.mcap".to_string(),
            kind: std::fs::FileType::RegularFile,
            size: 0,
            modified_time: std::time::SystemTime::now(),
        },
    ]);

    show_open_file(fs_handles).await?;

    select_source(&sources, &fs_handles)?;

    Ok(())
}
```
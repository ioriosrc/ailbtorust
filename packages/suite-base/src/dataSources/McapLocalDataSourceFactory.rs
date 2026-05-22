```rust
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

#[derive(Debug)]
pub struct McapLocalDataSourceFactory {
    id: String,
    type_: String,
    display_name: String,
    icon_name: String,
    supported_file_types: Vec<String>,
    supports_multi_file: bool,
}

impl McapLocalDataSourceFactory {
    pub fn new() -> Self {
        Self {
            id: "mcap-local-file".to_string(),
            type_: "file".to_string(),
            display_name: "MCAP".to_string(),
            icon_name: "OpenFile".to_string(),
            supported_file_types: vec![AllowedFileExtensions::MCP.to_string()],
            supports_multi_file: true,
        }
    }

    pub fn initialize(&self, args: DataSourceFactoryInitializeArgs) -> Option<Player> {
        let mut files = args.files.unwrap_or_default();

        if let Some(file) = &args.file {
            files.push(file);
        }
        if files.is_empty() {
            return None;
        }

        let source = McapIterableSourceWorker::new(&files).unwrap();
        let name = merge_multiple_file_names(files.iter().map(|file| file.name()));

        Some(IterablePlayer::new(
            args.metrics_collector.unwrap(),
            source,
            name.to_string(),
            120, // sec
            0,   // nsec
            self.id.clone(),
        ))
    }
}

#[derive(Debug)]
struct McapIterableSourceWorker {
    files: Vec<PathBuf>,
}

impl McapIterableSourceWorker {
    fn new(files: &Vec<PathBuf>) -> Result<Self, io::Error> {
        let source = WorkerSerializedIterableSource::new({
            init_worker: || {
                File::open(
                    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                        .join("suite-base")
                        .join("players")
                        .join("IterablePlayer")
                        .join("Mcap")
                        .join("McapIterableSourceWorker.worker"),
                )
            },
            init_args: serde_json::to_string(&files.to_vec()).unwrap(),
        })?;

        Ok(Self { files: Vec::from(files.clone()) })
    }
}
```

This Rust code converts the given TypeScript/React code to a functional-style Rust implementation. It includes a `McapLocalDataSourceFactory` struct with methods for initializing players based on file paths or single files, and a `McapIterableSourceWorker` to manage the worker logic that processes the MCAP data.
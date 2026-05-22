```rust
use storybook::prelude::*;

#[derive(Component)]
struct FileInfoDisplay {
    file_stats: FileStats,
    error: Option<Box<dyn std::error::Error>>,
}

#[story]
fn bag() -> FileInfoDisplay {
    FileInfoDisplay {
        file_stats: FileStats {
            name: "name.bag".to_string(),
            size: 0,
        },
        error: None,
    }
}

#[story]
fn mcap() -> FileInfoDisplay {
    FileInfoDisplay {
        file_stats: FileStats {
            name: "name.mcap".to_string(),
            size: 0,
        },
        error: None,
    }
}

#[story]
fn long_name() -> FileInfoDisplay {
    FileInfoDisplay {
        file_stats: FileStats {
            name: "a_really_long_file_name_that_wraps_a_really_long_file_name_that_wraps_a_really_long_file_name_that_wraps.mcap".to_string(),
            size: 0,
        },
        error: None,
    }
}

#[story]
fn error() -> FileInfoDisplay {
    FileInfoDisplay {
        file_stats: FileStats {
            name: "name".to_string(),
            size: 0,
        },
        error: Some(Box::new(Error("Example error"))),
    }
}

#[story]
fn details() -> FileInfoDisplay {
    FileInfoDisplay {
        file_stats: FileStats {
            name: "name.mcap".to_string(),
            size: 0,
        },
        fileInfo: Some(FileInfo {
            fileType: "file type".to_string(),
            numChunks: 1,
            numAttachments: 2,
            totalMessages: 3n,
            startTime: {
                sec: 0,
                nsec: 1,
            },
            endTime: {
                sec: 1,
                nsec: 2,
            },
            topics: vec![
                Topic {
                    topic: "foo".to_string(),
                    schemaName: "Foo".to_string(),
                    numMessages: 100n,
                    numConnections: 1,
                },
                Topic {
                    topic: "bar".to_string(),
                    schemaName: "Bar".to_string(),
                    numMessages: 1_000_000n,
                    numConnections: 2,
                },
                Topic {
                    topic: "baz".to_string(),
                    schemaName: "Baz".to_string(),
                    numMessages: None,
                    numConnections: 0,
                },
            ],
            compressionTypes: set!["zstd"],
        }),
    }
}
```

Note: This code assumes that the `FileInfo`, `FileStats`, and `Topic` types are already defined elsewhere in your project.
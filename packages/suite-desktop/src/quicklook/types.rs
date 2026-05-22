```rust
use std::collections::HashSet;

pub type TopicInfo = {
    topic: String,
    schema_name: String,
    num_messages: Option<BigInt>,
    num_connections: usize,
};

pub type FileInfo = {
    load_more_info: fn(report_progress: &mut dyn FnMut(i32)) -> Box<dyn Future<Item = FileInfo>>;
    file_type: Option<&str>,
    num_chunks: Option<usize>,
    num_attachments: Option<usize>,
    total_messages: Option<BigInt>,
    start_time: Option<i64>,
    end_time: Option<i64>,
    topics: Option<Vec<TopicInfo>>,
    compression_types: HashSet<String>,
};
```
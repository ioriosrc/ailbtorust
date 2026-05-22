```rust
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom};

type BagInfo = HashMap<String, TopicInfo>;
type TopicInfo = HashMap<String, (i64, i64)>;
type ConnectionIndex = usize;

struct FileInfo {
    fileType: Option<&'static str>,
    totalMessages: i64,
    numChunks: usize,
    startTime: Option<i64>,
    endTime: Option<i64>,
    topics: Vec<(String, TopicInfo)>,
}

#[derive(Debug)]
struct BagReader {
    file: File,
}

impl BagReader {
    fn new(file: File) -> Self {
        BagReader { file }
    }

    async fn read_chunk_info(&mut self) -> io::Result<Vec<ChunkInfo>> {
        let mut buffer = Vec::new();
        self.file.read_to_end(&mut buffer).await?;
        Ok(buffer)
    }

    async fn parse_chunk_info(&self, chunk_data: &[u8]) -> Result<Vec<(ConnectionIndex, i64)>, BagReaderError> {
        // Parse chunk information logic here
        unimplemented!()
    }
}

async fn get_bag_info(file_path: &str) -> io::Result<FileInfo> {
    let file = fs::File::open(file_path)?;
    let mut bag_reader = BagReader::new(file);
    let chunk_infos = bag_reader.read_chunk_info().await?;

    let mut topic_infos_by_topic: HashMap<String, TopicInfo> = HashMap::new();
    for chunk in chunk_infos {
        for (conn, count) in chunk.connections {
            if !topic_infos_by_topic.contains_key(&chunk.topic) {
                topic_infos_by_topic.insert(chunk.topic.clone(), {
                    schema_name: None,
                    num_messages: 0,
                    num_connections: 1,
                });
            } else {
                let info = &mut topic_infos_by_topic[&chunk.topic];
                if info.schema_name.is_none() {
                    info.schema_name = Some(Some(chunk.topic));
                }
                info.num_messages += count;
                info.num_connections += 1;
            }
        }
    }

    let mut topics: Vec<(String, TopicInfo)> = topic_infos_by_topic.into_iter().collect();
    topics.sort_unstable_by_key(|(topic, _)| topic);

    Ok(FileInfo {
        fileType: None,
        total_messages: topics.iter().map(|(_, info)| info.num_messages).sum(),
        num_chunks: chunk_infos.len(),
        startTime: Some(chunk_infos[0].start_time),
        endTime: Some(chunk_infos.last().unwrap().end_time),
        topics,
    })
}

#[derive(Debug)]
struct BagReaderError {
    message: String,
}
```
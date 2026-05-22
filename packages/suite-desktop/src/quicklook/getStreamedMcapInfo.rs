```rust
use std::collections::{HashSet, HashMap};
use std::fs;
use std::io::{self, Seek, Write};

use bytes::Bytes;

#[derive(Debug)]
struct McapInfo {
    total_messages: u64,
    num_chunks: usize,
    num_attachments: usize,
    start_time: Option<chrono::NaiveDateTime>,
    end_time: Option<chrono::NaiveDateTime>,
    compression_types: HashSet<String>,
    topic_infos_by_topic: HashMap<String, TopicInfo>,
    topic_names_by_channel_id: HashMap<usize, String>,
    schema_namesById: HashMap<usize, String>,
}

#[derive(Debug)]
struct TopicInfo {
    topic: String,
    schema_name: String,
    num_messages: u64,
    num_connections: usize,
    connection_ids: HashSet<usize>,
}

impl From<&McapTypes::TypedMcapRecord> for TopicInfo {
    fn from(record: &McapTypes::TypedMcapRecord) -> Self {
        TopicInfo {
            topic: record.channel.topic.to_string(),
            schema_name: record.schema.name.to_string(),
            num_messages: 0,
            num_connections: 1,
            connection_ids: HashSet::new(),
        }
    }
}

fn process_mcap_record(info: &mut McapInfo, record: &McapTypes::TypedMcapRecord) {
    match record.type_() {
        McapTypes::Chunk => {
            info.num_chunks += 1;
            info.compression_types.insert(record.compression());
        },
        McapTypes::Attachment => {
            info.num_attachments += 1;
        },
        McapTypes::Schema => {
            info.schema_names_by_id.insert(record.id(), record.name().to_string());
        },
        McapTypes::Channel => {
            info.topic_names_by_channel_id
                .entry(record.channel.id())
                .or_insert_with(|| record.channel.topic.to_string())
                .with_key(|id| id)
                .and_modify(|topic_info| {
                    if topic_info.schema_name != record.schema.name().to_string() {
                        topic_info.schema_name = "(multiple)";
                    }
                    topic_info.num_connections += 1;
                    topic_info.connection_ids.insert(*id);
                });
        },
        McapTypes::Message => {
            let topic = info.topic_names_by_channel_id.get(record.channel.id());
            if let Some(topic_info) = topic.and_then(|topic_info| {
                topic_info.clone()
            }) {
                topic_info.num_messages += 1;
            }
            info.total_messages += 1;

            let timestamp = chrono::NaiveDateTime::from_secs_f64(f64::from(record.log_time()));
            if !info.start_time || timestamp < info.start_time {
                info.start_time = Some(timestamp);
            }
            if !info.end_time || timestamp > info.end_time {
                info.end_time = Some(timestamp);
            }
        },
        _ => (),
    }
}

pub async fn get_streamed_mcap_info<R>(
    file: &fs::File,
    mcap_reader: GenericMcapStreamReader<R>,
    process_record: impl Fn(&mut McapInfo, &McapTypes::TypedMcapRecord),
    fileType: &str,
    report_progress: impl Fn(f64) -> (),
) -> Result<FileInfo, io::Error> {
    let mut info = McapInfo {
        total_messages: 0,
        num_chunks: 0,
        num_attachments: 0,
        start_time: None,
        end_time: None,
        compression_types: HashSet::new(),
        topic_infos_by_topic: HashMap::new(),
        topic_names_by_channel_id: HashMap::new(),
        schema_names_by_id: HashMap::new(),
    };

    const CHUNK_SIZE: usize = 1024 * 1024;
    let mut bytes_read = 0;

    for offset in (0..file.size()).step_by(CHUNK_SIZE) {
        let buffer = file.read_at(offset, None)?;
        mcap_reader.append(buffer.as_slice());
        while let Some(record) = mcap_reader.next_record() {
            process_record(&mut info, record);
        }
        bytes_read += buffer.len();
        report_progress(bytes_read as f64 / file.size() as f64);
    }

    let topics: Vec<&TopicInfo> = info.topic_infos_by_topic.values().collect();
    topics.sort_unstable_by(|a, b| a.topic.cmp(&b.topic));

    Ok(FileInfo {
        total_messages: info.total_messages,
        num_chunks: info.num_chunks,
        num_attachments: info.num_attachments,
        start_time: info.start_time.unwrap_or(chrono::NaiveDateTime::min_value()),
        end_time: info.end_time.unwrap_or(chrono::NaiveDateTime::max_value()),
        compression_types: info.compression_types,
        topic_infos_by_topic: info.topic_infos_by_topic,
        topic_names_by_channel_id: info.topic_names_by_channel_id,
        schema_names_by_id: info.schema_names_by_id,
        fileType,
        topics,
    })
}

#[derive(Debug)]
struct FileInfo {
    total_messages: u64,
    num_chunks: usize,
    num_attachments: usize,
    start_time: chrono::NaiveDateTime,
    end_time: chrono::NaiveDateTime,
    compression_types: HashSet<String>,
    topic_infos_by_topic: HashMap<String, TopicInfo>,
    topic_names_by_channel_id: HashMap<usize, String>,
    schema_names_by_id: HashMap<usize, String>,
    fileType: &'static str,
    topics: Vec<&TopicInfo>,
}
```
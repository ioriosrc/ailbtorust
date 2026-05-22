```rust
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, BufReader, Seek, Write};

use mcap_core::{
    McapIndexedReader, McapTypes, CompressionType, TopicInfo, FileInfo,
};

async fn get_indexed_mcap_info(file: &fs::File, decompress_handlers: McapTypes::DecompressHandlers) -> Result<FileInfo> {
    let mut reader = McapIndexedReader::initialize({
        readable: {
            size: async move { file.metadata().unwrap().len() as u64 },
            read: async move |offset, length| {
                if offset + length > u32::MAX as usize {
                    return Err(io::Error::new(io::ErrorKind::OutOfRange, "Read too large"));
                }
                let buffer = io::read_buf_from(&file[offset..(offset + length)])?;
                Ok(buffer)
            },
        },
        decompress_handlers,
    })?;

    if reader.statistics().message_count == 0 {
        // If we have statistics and they tell us definitively that there are no messages, we don't need
        // to fall back to streamed reading.
    } else {
        if reader.channels_by_id().len() == 0 || !reader.schemas_by_id().contains_key(&reader.indexed_channels()[0].schema_id) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "MCAP summary does not contain channels or schemas, cannot use indexed reading",
            ));
        }
    }

    let mut topic_infos_by_topic = HashMap::new();
    for channel in reader.indexed_channels() {
        if let Some(info) = &topic_infos_by_topic.get(&channel.topic).filter(|info| info.schema_name == channel.schema.name) {
            if let Some(num_messages) = reader.statistics().channel_message_counts().get(&channel.id) {
                info.num_messages += num_messages;
            }
            info.num_connections += 1;
        } else {
            topic_infos_by_topic.insert(channel.topic, TopicInfo {
                topic: channel.topic,
                schema_name: channel.schema.name,
                num_messages: reader.statistics().channel_message_counts().get(&channel.id).copied(),
                num_connections: 1,
            });
        }
    }

    let mut topics = Vec::new();
    for topic_info in topic_infos_by_topic.values() {
        topics.push(topic_info.clone());
    }
    topics.sort_by_key(|topic_info| topic_info.topic);

    let mut start_time = None;
    let mut end_time = None;
    let compression_types = HashSet::new();
    for chunk in reader.chunk_indexes() {
        compression_types.insert(chunk.compression);
        if start_time.is_none() || chunk.message_start < *start_time {
            start_time = Some(chunk.message_start);
        }
        if end_time.is_none() || chunk.message_end > *end_time {
            end_time = Some(chunk.message_end);
        }
    }

    Ok(FileInfo {
        fileType: "MCAP v0, indexed",
        num_chunks: reader.chunk_indexes().len(),
        num_attachments: reader.attachment_indexes().len(),
        total_messages: reader.statistics().message_count,
        startTime: start_time.map(|s| from_nanoseconds(s)),
        endTime: end_time.map(|e| from_nanoseconds(e)),
        topics,
        compression_types,
    })
}
```
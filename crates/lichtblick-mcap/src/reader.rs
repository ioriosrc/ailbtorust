// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use base64::Engine as _;
use lichtblick_core::error::LichtblickError;
use lichtblick_core::time::Time;
use lichtblick_core::types::{MessageEvent, RosDatatypes, Topic, TopicStats};
use std::collections::HashMap;

use crate::schema::parse_schema;

/// Initialization result from opening an MCAP file.
pub struct McapInitResult {
    pub start_time: Time,
    pub end_time: Time,
    pub topics: Vec<Topic>,
    pub datatypes: RosDatatypes,
    pub topic_stats: HashMap<String, TopicStats>,
    pub profile: String,
}

/// Reads and parses MCAP files.
pub struct McapReader {
    /// Raw MCAP data (mapped or loaded).
    data: Vec<u8>,
}

impl McapReader {
    /// Create a reader from raw bytes.
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    /// Create a reader from anything that implements Read.
    pub fn from_reader<R: std::io::Read>(mut reader: R) -> Result<Self, LichtblickError> {
        let mut data = Vec::new();
        reader
            .read_to_end(&mut data)
            .map_err(|e| LichtblickError::Mcap(format!("Failed to read MCAP data: {}", e)))?;
        Ok(Self { data })
    }

    /// Initialize and return metadata about the MCAP file.
    pub fn initialize(&self) -> Result<McapInitResult, LichtblickError> {
        let summary = mcap::Summary::read(&self.data)
            .map_err(|e| LichtblickError::Mcap(format!("Failed to read MCAP summary: {}", e)))?;

        let summary = summary
            .ok_or_else(|| LichtblickError::Mcap("MCAP file has no summary section".into()))?;

        let mut topics = Vec::new();
        let mut datatypes = RosDatatypes::new();
        let mut topic_stats = HashMap::new();
        let mut start_time = Time::from_nanos(u64::MAX);
        let mut end_time = Time::ZERO;
        let mut profile = String::new();

        // Process channels and schemas from summary
        // summary.schemas is HashMap<u16, Arc<Schema<'_>>>
        // summary.channels is HashMap<u16, Arc<Channel<'_>>>

        for (_channel_id, channel) in &summary.channels {
            let topic_name = channel.topic.clone();

            let (schema_name, schema_encoding) = if let Some(schema) = &channel.schema {
                // Parse schema into datatypes
                if let Ok(parsed) =
                    parse_schema(&schema.name, &schema.encoding, &schema.data)
                {
                    for (name, def) in parsed {
                        datatypes.insert(name, def);
                    }
                }
                (schema.name.clone(), Some(schema.encoding.clone()))
            } else {
                (String::new(), None)
            };

            topics.push(Topic {
                name: topic_name,
                schema_name,
                encoding: Some(channel.message_encoding.clone()),
                schema_encoding,
            });
        }

        // Get stats from summary
        if let Some(stats) = &summary.stats {
            start_time = Time::from_nanos(stats.message_start_time);
            end_time = Time::from_nanos(stats.message_end_time);

            for (channel_id, count) in &stats.channel_message_counts {
                if let Some(channel) = summary.channels.get(channel_id) {
                    topic_stats.insert(
                        channel.topic.clone(),
                        TopicStats {
                            num_messages: *count,
                            first_message_time: Some(start_time),
                            last_message_time: Some(end_time),
                        },
                    );
                }
            }
        }

        // Detect profile from metadata or schema encoding
        if datatypes.keys().any(|k| k.contains('/')) {
            profile = "ros2".to_string();
        } else {
            profile = "mcap".to_string();
        }

        Ok(McapInitResult {
            start_time,
            end_time,
            topics,
            datatypes,
            topic_stats,
            profile,
        })
    }

    /// Iterate messages in a time range for given topics.
    pub fn read_messages(
        &self,
        topics: &[String],
        start: Time,
        end: Time,
    ) -> Result<Vec<MessageEvent>, LichtblickError> {
        let mut messages = Vec::new();

        for msg in mcap::MessageStream::new(&self.data)
            .map_err(|e| LichtblickError::Mcap(format!("Failed to create stream: {}", e)))?
        {
            let msg =
                msg.map_err(|e| LichtblickError::Mcap(format!("Read error: {}", e)))?;

            let log_time = Time::from_nanos(msg.log_time);
            let publish_time = Time::from_nanos(msg.publish_time);

            if log_time < start {
                continue;
            }
            if log_time > end {
                break;
            }

            if !topics.is_empty() && !topics.contains(&msg.channel.topic) {
                continue;
            }

            // Deserialize message data based on encoding
            let message_data = deserialize_message(
                &msg.data,
                &msg.channel.message_encoding,
                msg.channel.schema.as_ref().map(|s| s.encoding.as_str()),
            )?;

            messages.push(MessageEvent {
                topic: msg.channel.topic.clone(),
                receive_time: log_time,
                publish_time,
                size_in_bytes: msg.data.len(),
                schema_name: msg
                    .channel
                    .schema
                    .as_ref()
                    .map(|s| s.name.clone())
                    .unwrap_or_default(),
                message: message_data,
            });
        }

        Ok(messages)
    }

    /// Get the last message on each topic before the given time (for seek backfill).
    pub fn get_backfill_messages(
        &self,
        topics: &[String],
        time: Time,
    ) -> Result<Vec<MessageEvent>, LichtblickError> {
        let mut last_by_topic: HashMap<String, MessageEvent> = HashMap::new();

        for msg in mcap::MessageStream::new(&self.data)
            .map_err(|e| LichtblickError::Mcap(format!("Failed to create stream: {}", e)))?
        {
            let msg =
                msg.map_err(|e| LichtblickError::Mcap(format!("Read error: {}", e)))?;

            let log_time = Time::from_nanos(msg.log_time);
            if log_time > time {
                break;
            }

            if !topics.contains(&msg.channel.topic) {
                continue;
            }

            let message_data = deserialize_message(
                &msg.data,
                &msg.channel.message_encoding,
                msg.channel.schema.as_ref().map(|s| s.encoding.as_str()),
            )?;

            last_by_topic.insert(
                msg.channel.topic.clone(),
                MessageEvent {
                    topic: msg.channel.topic.clone(),
                    receive_time: log_time,
                    publish_time: Time::from_nanos(msg.publish_time),
                    size_in_bytes: msg.data.len(),
                    schema_name: msg
                        .channel
                        .schema
                        .as_ref()
                        .map(|s| s.name.clone())
                        .unwrap_or_default(),
                    message: message_data,
                },
            );
        }

        Ok(last_by_topic.into_values().collect())
    }
}

/// Deserialize raw message bytes based on encoding.
fn deserialize_message(
    data: &[u8],
    message_encoding: &str,
    schema_encoding: Option<&str>,
) -> Result<serde_json::Value, LichtblickError> {
    match message_encoding {
        "json" => serde_json::from_slice(data)
            .map_err(|e| LichtblickError::Mcap(format!("JSON deserialization error: {}", e))),
        "cdr" | "ros1msg" | "protobuf" | "flatbuffer" => {
            // For binary encodings, store as base64 until we have proper deserializers
            Ok(serde_json::json!({
                "__raw_encoding": message_encoding,
                "__schema_encoding": schema_encoding,
                "__data_base64": base64::engine::general_purpose::STANDARD.encode(data),
                "__size": data.len(),
            }))
        }
        _ => {
            // Unknown encoding - store raw
            Ok(serde_json::json!({
                "__raw_encoding": message_encoding,
                "__data_base64": base64::engine::general_purpose::STANDARD.encode(data),
                "__size": data.len(),
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_debug_osi() {
        println!("--- DEBUG OSI TEST START ---");
        let path = "/Users/CTW03722/Downloads/SanDiego_san_diego_sc7_urban_splits_and_parking_lot.xosc.mcap";
        let mut file = File::open(path).expect("Failed to open MCAP");
        let mut data = Vec::new();
        file.read_to_end(&mut data).expect("Failed to read MCAP");

        let reader = McapReader::new(data);
        let init_result = reader.initialize().expect("Failed to initialize MCAP");
        println!("Topics count: {}", init_result.topics.len());

        let target_topic = "ConvertedTrace";
        let target_schema = "osi3.SensorView";

        // Find the schema bytes
        let mut schema_bytes = None;
        for topic in &init_result.topics {
            if topic.name == target_topic {
                println!("Found topic: {} with schema: {}", topic.name, topic.schema_name);
            }
        }

        // Get schema from the summary
        let summary = mcap::Summary::read(&reader.data).unwrap().unwrap();
        for schema in summary.schemas.values() {
            if schema.name == target_schema {
                println!("Found schema in summary: {}, data length: {}", schema.name, schema.data.len());
                schema_bytes = Some(schema.data.clone());
            }
        }

        let schema_data = schema_bytes.expect("osi3.SensorView schema not found in MCAP");

        // Try compiling the DescriptorPool
        println!("Compiling DescriptorPool...");
        match prost_reflect::DescriptorPool::decode(schema_data.as_ref()) {
            Ok(pool) => {
                let message_desc = pool.get_message_by_name(target_schema);
                match message_desc {
                    Some(desc) => {
                        println!("Message descriptor found: {}", desc.full_name());
                        
                        // Read messages and decode the first one on the target topic
                        let messages = reader.read_messages(&[target_topic.to_string()], init_result.start_time, init_result.end_time).unwrap();
                        println!("Total messages on target topic: {}", messages.len());
                        if let Some(msg) = messages.first() {
                            println!("Reading first message: time_ns={}, size={}", msg.receive_time.to_nanos(), msg.size_in_bytes);
                            
                            // Try decoding the protobuf payload
                            // In reader.rs, msg.message contains a serde_json::Value representing the deserialized message.
                            // But msg.message for binary encodings is just a JSON containing base64 data!
                            // Let's get the raw bytes from base64
                            let raw_base64 = msg.message.get("__data_base64").and_then(|v| v.as_str()).unwrap();
                            use base64::Engine as _;
                            let raw_bytes = base64::engine::general_purpose::STANDARD.decode(raw_base64).unwrap();
                            
                            match prost_reflect::DynamicMessage::decode(desc, raw_bytes.as_slice()) {
                                Ok(dynamic_msg) => {
                                    println!("DynamicMessage decoded successfully!");
                                    
                                    // Print fields of SensorView
                                    use prost_reflect::ReflectMessage;
                                    let sensor_view_desc = dynamic_msg.descriptor();
                                    for field in sensor_view_desc.fields() {
                                        let value = dynamic_msg.get_field(&field);
                                        if value.is_default(&field.kind()) {
                                            continue;
                                        }
                                        println!("  Field: {} -> {:?}", field.name(), value);
                                    }
                                }
                                Err(e) => {
                                    println!("FAILED to decode DynamicMessage: {}", e);
                                }
                            }
                        } else {
                            println!("No messages found on target topic.");
                        }
                    }
                    None => {
                        println!("Message descriptor NOT found for: {}", target_schema);
                    }
                }
            }
            Err(e) => {
                println!("FAILED to compile DescriptorPool: {}", e);
            }
        }
        panic!("Force test failure to print output");
        println!("--- DEBUG OSI TEST END ---");
    }
}


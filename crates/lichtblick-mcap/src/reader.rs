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

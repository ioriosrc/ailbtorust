// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use lichtblick_core::error::LichtblickError;
use lichtblick_core::time::Time;
use lichtblick_core::types::{MessageEvent, RosDatatypes, Topic, TopicStats};
use std::collections::HashMap;

use crate::reader::{McapInitResult, McapReader};

/// MCAP-based iterable source (implements the source pattern for the player).
pub struct McapIterableSource {
    reader: McapReader,
    init_result: Option<McapInitResult>,
}

/// Initialization metadata.
pub struct SourceInitialization {
    pub start_time: Time,
    pub end_time: Time,
    pub topics: Vec<Topic>,
    pub datatypes: RosDatatypes,
    pub topic_stats: HashMap<String, TopicStats>,
    pub profile: String,
}

/// Result from message iteration.
pub enum IteratorResult {
    /// A message event.
    MessageEvent(MessageEvent),
    /// A time stamp indicating progress through the file.
    Stamp(Time),
}

impl McapIterableSource {
    /// Create from raw MCAP bytes.
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            reader: McapReader::new(data),
            init_result: None,
        }
    }

    /// Initialize the source, reading headers and metadata.
    pub fn initialize(&mut self) -> Result<SourceInitialization, LichtblickError> {
        let result = self.reader.initialize()?;
        let init = SourceInitialization {
            start_time: result.start_time,
            end_time: result.end_time,
            topics: result.topics.clone(),
            datatypes: result.datatypes.clone(),
            topic_stats: result.topic_stats.clone(),
            profile: result.profile.clone(),
        };
        self.init_result = Some(result);
        Ok(init)
    }

    /// Get messages in a time range for the given topics.
    pub fn message_iterator(
        &self,
        topics: &[String],
        start: Time,
        end: Time,
    ) -> Result<Vec<IteratorResult>, LichtblickError> {
        let messages = self.reader.read_messages(topics, start, end)?;
        Ok(messages
            .into_iter()
            .map(IteratorResult::MessageEvent)
            .collect())
    }

    /// Get the last message for each topic before the given time (seek backfill).
    pub fn get_backfill_messages(
        &self,
        topics: &[String],
        time: Time,
    ) -> Result<Vec<MessageEvent>, LichtblickError> {
        self.reader.get_backfill_messages(topics, time)
    }
}

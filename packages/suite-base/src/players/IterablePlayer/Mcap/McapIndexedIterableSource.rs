```rust
use mcap_core::{McapIndexedReader, McapTypes};
use log::Logger;
use lighblick::rostime::{fromNanoSec, toNanoSec, compare};

pub struct McapIndexedIterableSource {
    reader: McapIndexedReader,
}

impl McapIndexedIterableSource {
    pub fn new(reader: McapIndexedReader) -> Self {
        Self { reader }
    }

    async fn initialize(&self) -> Initialization {
        let mut start_time = None;
        let mut end_time = None;

        for chunk in self.reader.chunk_indexes() {
            if start_time.is_none() || chunk.message_start < *start_time {
                start_time = Some(chunk.message_start);
            }
            if end_time.is_none() || chunk.message_end > *end_time {
                end_time = Some(chunk.message_end);
            }
        }

        let mut topic_stats = HashMap::new();
        let mut topics_by_name = HashMap::new();
        let datatypes: HashMap<String, RosDatatype> = HashMap::new();
        let alerts: Vec<PlayerAlert> = Vec::new();
        let metadata: Vec<Metadata> = Vec::new();

        let mut publishers_by_topic = HashMap::new();

        for channel in self.reader.channelsById().values() {
            let schema = self.reader.schemasById.get(&channel.schema_id);
            if channel.schema_id != 0 && schema.is_none() {
                alerts.push(PlayerAlert {
                    severity: "error",
                    message: format!(
                        "Missing schema info for schema id {} (channel {}, topic {})",
                        channel.schema_id, channel.id, channel.topic
                    ),
                });
                continue;
            }

            let parsed_channel = parse_channel(&channel.message_encoding, schema);
            self.#channel_info_by.insert(channel.id, {
                channel,
                parsed_channel,
                schema_name: schema.map(|s| s.name).unwrap_or_default(),
            });

            let topic = topics_by_name.entry(channel.topic).or_insert({
                TopicWithDecodingInfo {
                    name: channel.topic,
                    schema_name: schema.map(|s| s.name).unwrap_or_default(),
                    message_encoding: channel.message_encoding,
                    schema_data: schema.map(|s| s.data),
                    schema_encoding: schema.map(|s| s.encoding),
                }
            });

            let num_messages = self.reader.statistics().channel_message_counts.get(&channel.id);
            if num_messages.is_some() {
                topic_stats.insert(channel.topic, TopicStats {
                    num_messages: num_messages.unwrap(),
                });
            }

            // Track the publisher for this topic. "callerid" is defined in the MCAP ROS 1 Well-known
            // profile at <https://mcap.dev/specification/appendix.html>. We skip the profile check to
            // allow non-ROS profiles to utilize this functionality as well
            let publisher_id = channel.metadata.get("callerid").unwrap_or(&channel.id).to_string();
            let publishers = publishers_by_topic.entry(channel.topic).or_insert_with(|| HashSet::new());
            publishers.insert(publisher_id);

            // Final datatypes is an unholy union of schemas across all channels
            for (name, datatype) in parsed_channel.datatypes {
                datatypes.insert(name.to_string(), datatype);
            }
        }

        self.#start = start_time.unwrap_or_default().into();
        self.#end = end_time.unwrap_or(self.#start);

        let metadata_generator = self.reader.read_metadata();
        let mut metadata_iterator = metadata_generator.next();

        while !metadata_iterator.is_done() {
            metadata.push(Metadata {
                name: metadata_iterator.value.name,
                metadata: metadata_iterator.value.metadata.into_iter().collect(),
            });
            metadata_iterator.next();
        }

        Initialization {
            start: self.#start,
            end: self.#end,
            topics: topics_by_name.values().cloned().collect(),
            datatypes,
            profile: self.reader.header.profile.to_string(),
            alerts,
            metadata,
            publishers_by_topic,
            topic_stats,
        }
    }

    pub async fn message_iterator(&self, args: MessageIteratorArgs) -> AsyncIter<MessageEvent<Uint8Array>> {
        let topics = args.topics;
        let start = args.start.unwrap_or(self.#start);
        let end = args.end.unwrap_or(self.#end);

        if topics.is_empty() || start.is_none() || end.is_none() {
            return Ok(AsyncIter::empty());
        }

        let topic_names: Vec<&str> = topics.iter().map(|t| t.as_str()).collect();

        async fn read_messages_in_topic(
            reader: &McapIndexedReader,
            topic: &str,
            start: u64,
            end: u64,
        ) -> AsyncIter<MessageEvent<Uint8Array>> {
            let mut messages: Vec<MessageEvent<Uint8Array>> = Vec::new();
            for message in reader.read_messages({
                start: fromNanoSec(start),
                end: fromNanoSec(end),
                topics: [topic],
                reverse: true,
                validate_crcs: false,
            }) {
                if let Some(channel_info) = self.#channel_info_by.get(&message.channel_id) {
                    messages.push(MessageEvent {
                        topic: channel_info.channel.topic.to_string(),
                        receive_time: fromNanoSec(message.log_time),
                        publish_time: fromNanoSec(message.publish_time),
                        message: message.data,
                        size_in_bytes: message.data.len() as u64,
                        schema_name: channel_info.schema_name.unwrap_or_default(),
                    });
                } else {
                    log::error!("Missing channel info for channel: {}", message.channel_id);
                }
            }

            messages.sort_by_key(|msg| compare(msg.receive_time, start));
            Ok(AsyncIter::from_iter(messages))
        }

        let mut topics_iterator = topics.into_iter();
        let mut messages_iterator = futures::stream::iter(topic_names)
            .filter_map(move |topic| async move {
                if let Some(iter) = read_messages_in_topic(&self.reader, topic, start, end).await {
                    Ok(iter)
                } else {
                    log::error!("Failed to read messages for topic: {}", topic);
                    Err(())
                }
            })
            .buffer_unordered(10);

        futures::stream::select_many(messages_iterator, async move {
            while let Some(result) = await messages_iterator.next() {
                yield result;
            }
        })
    }

    pub async fn get_backfill_messages(&self, args: GetBackfillMessagesArgs) -> Vec<MessageEvent<Uint8Array>> {
        let { topics, time } = args;

        let mut messages: Vec<MessageEvent<Uint8Array>> = Vec::new();
        for topic in topics.iter().map(|t| t.as_str()) {
            let mut last_message_time = None;
            for message in self.reader.read_messages({
                end: fromNanoSec(time),
                topics: [topic],
                reverse: true,
                validate_crcs: false,
            }) {
                if let Some(channel_info) = self.#channel_info_by.get(&message.channel_id) {
                    last_message_time = Some(message.log_time);
                    messages.push(MessageEvent {
                        topic: channel_info.channel.topic.to_string(),
                        receive_time: fromNanoSec(message.log_time),
                        publish_time: fromNanoSec(message.publish_time),
                        message: message.data,
                        size_in_bytes: message.data.len() as u64,
                        schema_name: channel_info.schema_name.unwrap_or_default(),
                    });
                } else {
                    log::error!("Missing channel info for channel: {}", message.channel_id);
                }
            }

            if let Some(last_message_time) = last_message_time {
                let start = fromNanoSec(last_message_time);
                messages.sort_by_key(|msg| compare(msg.receive_time, start));
            }
        }
        messages
    }

    pub fn get_start(&self) -> Time {
        self.#start
    }

    pub fn get_end(&self) -> Time {
        self.#end
    }
}
```
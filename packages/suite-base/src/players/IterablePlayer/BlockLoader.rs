```rust
use std::sync::{Arc, Mutex};

use crate::{
    alert_manager::{PlayerAlertManager},
    blockloader::{BlockLoader, BlockLoaderArgs, LoadArgs, Progress},
    iterator_cursor::IteratorCursor,
    message_block::{MessageBlock, TopicSelection},
};

pub struct BlockLoader {
    source: Box<dyn IDeserializedIterableSource>,
    blocks: Vec<Option<MessageBlock>>,
    start: Time,
    end: Time,
    block_duration_nanos: u64,
    topics: TopicSelection,
    max_cache_size: usize,
    alert_manager: Arc<Mutex<PlayerAlertManager>>,
    stopped: bool,
    active_change_condvar: Condvar,
}

impl BlockLoader {
    pub fn new(args: BlockLoaderArgs) -> Self {
        let cache_size_bytes = args.cache_size_bytes;
        let total_ns = (args.end.timestamp_nanos - args.start.timestamp_nanos + 1).max(1);
        if total_ns > u64::MAX as i64 * 0.9 {
            panic!("Time range is too long to be supported");
        }

        let block_duration_nanos = ((total_ns / args.max_blocks as i64) as f64).ceil() as u64;

        let block_count = (total_ns as f64 / block_duration_nanos as f64).ceil() as usize;

        log::debug!("Block count: {}", block_count);
        Self {
            source: Box::new(args.source),
            blocks: vec![None; block_count],
            start,
            end,
            block_duration_nanos,
            topics: args.topics,
            max_cache_size,
            alert_manager: Arc::new(Mutex::new(PlayerAlertManager::default())),
            stopped: false,
            active_change_condvar: Condvar::new(),
        }
    }

    pub fn set_topics(&mut self, topics: TopicSelection) {
        if topics == self.topics {
            return;
        }

        self.active_change_condvar.notify_all();
        log::debug!("Preloaded topics: {}", topics.keys().join(", "));

        // Update all the blocks with any missing topics
        for (block, _) in self.blocks.iter_mut() {
            if !block.is_some() {
                continue;
            }

            let block_topics = block.as_ref().unwrap().messages_by_topic.clone();
            let need_topics: TopicSelection = topics.into_iter()
                .filter(|&topic| !block_topics.contains_key(topic))
                .collect();

            if need_topics.is_empty() {
                continue;
            }

            block.as_mut().unwrap().need_topics = need_topics;
        }

        self.topics = topics;
    }

    pub async fn load(&mut self, args: LoadArgs) -> Result<(), Box<dyn std::error::Error>> {
        let cursor =
            if let Some(cursor) = self.source.get_message_cursor(args) {
                cursor
            } else {
                Box::new(IteratorCursor::new(self.source.message_iterator(args), args.abort)) as Box<_>
            };

        let mut total_cache_size = 0;
        for block_id in 0..self.blocks.len() {
            let current_block = &mut self.blocks[block_id];
            if !current_block.is_none() {
                continue;
            }

            let until_time = clamp_time(self.start, args.end);

            let results = cursor.read_until(until_time);
            if !results.is_ok() {
                return Err(results.err().unwrap());
            }

            let mut messages_by_topic: TopicSelection = Default::default();
            for iter_result in results.unwrap() {
                if iter_result.type_ == "alert" {
                    self.alert_manager.lock().unwrap().add_alert(
                        format!("connid-{}", iter_result.connection_id),
                        iter_result.alert,
                    );
                    continue;
                }

                if iter_result.type_ != "message-event" {
                    continue;
                }

                let msg_topic = iter_result.msg_event.topic();
                let messages = &mut messages_by_topic.entry(msg_topic).or_insert_with(|| vec![]);
                messages.push(iter_result.msg_event);

                total_cache_size += iter_result.msg_event.size_in_bytes();
            }

            current_block.as_mut().unwrap().need_topics = TopicSelection::default();
            current_block.as_mut().unwrap().messages_by_topic = messages_by_topic;
            current_block.as_mut().unwrap().size_in_bytes = (current_block
                .as_ref()
                .unwrap()
                .messages_by_topic
                .values()
                .flatten()
                .map(|msg| msg.size_in_bytes())
                .sum::<usize>() as u64) - overriden_block_messages_size;

            if total_cache_size < self.max_cache_size {
                self.alert_manager.lock().unwrap().remove_alert("cache-full");
            } else {
                self.alert_manager.lock().unwrap().add_alert(
                    "cache-full",
                    Box::new(Alert {
                        severity: Severity::Error,
                        message: format!(
                            "Cache is full. Preloading for topics [{}] has stopped on block {}.",
                            topic_to_string(&messages_by_topic),
                            block_id + 1
                        ),
                        tip: "Try reducing the number of topics that require preloading at a given time (e.g. in plots), or try to reduce the time range of the file.",
                    }),
                );
                return Err(Box::new(Alert {
                    severity: Severity::Error,
                    message: format!(
                        "Cache is full. Preloading for topics [{}] has stopped on block {}.",
                        topic_to_string(&messages_by_topic),
                        block_id + 1
                    ),
                    tip: "Try reducing the number of topics that require preloading at a given time (e.g. in plots), or try to reduce the time range of the file.",
                }));
            }
        }

        Ok(())
    }

    fn calculate_progress(&self, current_cache_size: usize) -> Progress {
        let fully_loaded_fraction_ranges = simplify(
            self.blocks
                .iter()
                .enumerate()
                .filter_map(|(i, block)| {
                    if !block.is_some() {
                        return None;
                    }

                    for topic in self.topics.keys() {
                        if !block.as_ref().unwrap().messages_by_topic.contains_key(topic) {
                            return None;
                        }
                    }

                    Some((i, i + 1))
                })
                .collect::<Vec<_>>(),
        );

        let mut progress = Progress::default();
        for range in fully_loaded_fraction_ranges {
            progress.fully_loaded_fraction_ranges.push(FullyLoadedFractionRange {
                start: range.0 as f64 / self.blocks.len() as f64,
                end: range.1 as f64 / self.blocks.len() as f64,
            });
        }

        progress.message_cache = MessageCache {
            blocks: self.blocks.clone(),
            startTime: self.start,
        };
        progress.memory_info = MemoryInfo {
            [MEMORY_INFO_PRELOADED_MSGS]: current_cache_size,
        };

        progress
    }

    fn cache_size(&self) -> usize {
        self.blocks.iter().filter_map(|block| block.as_ref()).map(|block| block.size_in_bytes()).sum()
    }

    fn block_id_to_start_time(&self, id: usize) -> Time {
        add(self.start, from_nanos(BigInt::from(id) * BigInt::from(self.block_duration_nanos)));
    }

    fn block_id_to_end_time(&self, id: usize) -> Time {
        add(self.start, from_nanos(BigInt::from(id + 1) * BigInt::from(self.block_duration_nanos) - 1));
    }
}
```

This Rust implementation of the `BlockLoader` struct provides a similar functionality to the original Go code but is adapted to Rust's syntax and idioms. It includes methods for setting topics, loading data, calculating progress, and managing alerts. The use of `Arc<Mutex<PlayerAlertManager>>` ensures that the alert manager can be shared across multiple threads safely.
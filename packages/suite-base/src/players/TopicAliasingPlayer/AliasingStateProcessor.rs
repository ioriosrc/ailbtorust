```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use std::collections::{HashMap, HashSet};

pub struct AliasMap {
    mappings: HashMap<String, Vec<String>>,
}

impl AliasMap {
    pub fn new() -> Self {
        AliasMap { mappings: HashMap::new() }
    }

    pub fn add_mapping(&mut self, source_topic: &str, alias: &str) {
        if let Some(mut aliases) = self.mappings.get_mut(source_topic) {
            aliases.push(alias.to_string());
        } else {
            self.mappings.insert(source_topic.to_string(), vec![alias.to_string()]);
        }
    }

    pub fn get_mappings(&self, source_topic: &str) -> Option<&Vec<String>> {
        self.mappings.get(source_topic)
    }
}

pub struct AliasingStateProcessor {
    alerts: Vec<PlayerAlert>,
    mapping: AliasMap,
    inverse_mapping: AliasMap,
    block_processors: Vec<BlockTopicProcessor>,
}

impl AliasingStateProcessor {
    pub fn new(mapping: AliasMap, alerts: Option<Vec<PlayerAlert>>) -> Self {
        let alerts = alerts.unwrap_or_default();
        let inverse_mapping = invert_alias_map(&mapping);
        let block_processors = mapping
            .mappings
            .iter()
            .map(|(source_topic, aliases)| BlockTopicProcessor::new(source_topic.to_string(), aliases))
            .collect();

        AliasingStateProcessor {
            alerts,
            mapping,
            inverse_mapping,
            block_processors,
        }
    }

    pub fn process(&self, player_state: PlayerState, subscriptions: Vec<SubscribePayload>) -> PlayerState {
        let mut new_state = player_state.clone();

        if let Some(active_data) = &mut new_state.active_data {
            active_data.topics = self.alias_topics(&active_data.topics);
            active_data.messages = self.alias_messages(&active_data.messages);
            if let Some(published_topics) = &mut active_data.published_topics {
                active_data.published_topics = self.alias_published_topics(&published_topics);
            }
            if let Some(subscribed_topics) = &mut active_data.subscribed_topics {
                active_data.subscribed_topics = self.alias_subscribed_topics(subscriptions, subscriptions.clone());
            }

            active_data.topic_stats = self.alias_topic_stats(&active_data.topic_stats);
        }

        if let Some(progress) = &mut new_state.progress {
            if let Some(message_cache) = &mut progress.message_cache {
                message_cache.blocks = self.alias_blocks(&message_cache.blocks);
            }
        }

        new_state.alerts = self.add_alerts(new_state.alerts.clone());

        return new_state;
    }

    fn alias_subscriptions(subscriptions: Vec<SubscribePayload>) -> Vec<SubscribePayload> {
        subscriptions
            .into_iter()
            .map(|sub| sub.transform(self.inverse_mapping.get(&sub.topic).unwrap_or(&[sub.topic])))
            .collect()
    }

    fn add_alerts(existing: Vec<PlayerAlert>) -> Vec<PlayerAlert> {
        existing.into_iter().chain(self.alerts.clone()).collect()
    }

    fn alias_blocks(blocks: Vec<MessageBlock>) -> Vec<MessageBlock> {
        blocks
            .iter()
            .map(|block| self.alias_block(block, 0))
            .collect()
    }

    fn alias_block(&self, block: &MessageBlock, idx: usize) -> MessageBlock {
        if let Some(messages_by_topic) = &block.messages_by_topic {
            let mapped_messages = messages_by_topic
                .iter()
                .map(|msg| self.alias_message(msg))
                .collect();

            return MessageBlock {
                topic: msg.topic,
                messages: mapped_messages,
                index: idx,
            };
        }

        block.clone()
    }

    fn alias_message(&self, msg: &MessageEvent) -> MessageEvent {
        if let Some(mappings) = self.mapping.get(&msg.topic) {
            for &alias in mappings.iter() {
                return MessageEvent {
                    topic: alias.to_string(),
                    content: msg.content.to_string(),
                    index: msg.index,
                };
            }
        }

        msg.clone()
    }

    fn alias_published_topics(&self, topics: &Map<String, Set<String>>) -> Map<String, Set<String>> {
        let mut mapped_topics = HashMap::new();

        for (topic, values) in topics.iter() {
            let mapped_values = values
                .iter()
                .map(|value| self.mapping.get(value).unwrap_or(&[value]))
                .flatten()
                .collect::<HashSet<&str>>();

            mapped_topics.insert(topic.to_string(), mapped_values);
        }

        mapped_topics
    }

    fn alias_subscribed_topics(
        &self,
        topics: Vec<SubscribePayload>,
        subscriptions: Vec<SubscribePayload>,
    ) -> Map<String, Set<String>> {
        let subscriptions_by_topic = HashMap::from_iter(subscriptions.into_iter().map(|sub| (sub.topic.clone(), vec![sub])));
        let mut mapped_topics = HashMap::new();

        for (topic, values) in topics.iter() {
            let mappings = self.mapping.get(&topic).unwrap_or(&[topic]);
            let mut mapped_values = HashSet::new();
            for &alias in mappings.iter() {
                if subscriptions_by_topic.contains_key(alias) {
                    mapped_values.extend(subscriptions_by_topic[alias].clone());
                } else if !values.is_empty() {
                    mapped_values.insert(alias);
                }
            }

            mapped_topics.insert(topic.to_string(), mapped_values);
        }

        mapped_topics
    }

    fn alias_topics(&self, topics: Vec<Topic>) -> Vec<Topic> {
        let mut mapped_topics = Vec::new();

        for topic in topics.iter() {
            if let Some(mappings) = self.mapping.get(&topic.name) {
                for &alias in mappings.iter() {
                    mapped_topics.push(Topic {
                        name: alias.to_string(),
                        aliased_from_name: topic.name,
                    });
                }
            } else {
                mapped_topics.push(topic.clone());
            }
        }

        mapped_topics
    }

    fn alias_topic_stats(&self, stats: Vec<TopicStats>) -> Vec<TopicStats> {
        let mut mapped_stats = Vec::new();

        for stat in stats.iter() {
            if let Some(mappings) = self.mapping.get(&stat.topic) {
                for &alias in mappings.iter() {
                    mapped_stats.push(TopicStats {
                        topic: alias.to_string(),
                        // Assuming TopicStats has a `clone` method
                    });
                }
            } else {
                mapped_stats.push(stat.clone());
            }
        }

        mapped_stats
    }
}

// Inverts a mapping, used to reverse map incoming subscriptions to subscriptions we pass
// through to the wrapped player.
fn invert_alias_map(alias_map: &AliasMap) -> AliasMap {
    let mut inverted = AliasMap::new();

    for (source_topic, aliases) in alias_map.mappings.iter() {
        for &alias in aliases.iter() {
            let new_values = inverted.get_mut(&alias).unwrap_or_else(|| vec![]);
            new_values.push(source_topic.to_string());
        }
    }

    inverted
}
```
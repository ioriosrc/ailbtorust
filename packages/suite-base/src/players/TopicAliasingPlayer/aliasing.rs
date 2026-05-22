```rust
use std::collections::{HashMap, HashSet};

type TopicAliasMap = HashMap<String, Vec<String>>;
type MessageBlocks = Option<Vec<MessageBlock>>;
const EMPTY_ALIAS_MAP: TopicAliasMap = HashMap::new();

pub type TopicAliasFunctions = Vec<{ extension_id: String; alias_function: fn(&Topic) -> Vec<String> }>;

pub type AliasingInputs = {
    alias_functions: TopicAliasFunctions;
    topics: Option<&[Topic]};
    variables: GlobalVariables;
};

fn alias_blocks(blocks: MessageBlocks, mapping: TopicAliasMap) -> MessageBlocks {
    if mapping.is_empty() {
        return blocks;
    }

    let mut mapped_blocks: Vec<MessageBlock> = Vec::new();
    
    for block in blocks.into_iter().filter_map(|block| {
        if block.is_none() {
            Some(block)
        } else {
            let msg = block.unwrap();
            let mut messages_by_topic: HashMap<String, Vec<MessageEvent>> = HashMap::new();

            for topic in &msg.messages_by_topic {
                let mappings = mapping.get(topic);
                if let Some(mappings) = mappings {
                    for mapped_topic in mappings {
                        messages_by_topic.entry(mapped_topic).or_insert_with(Vec::new)
                            .push(msg.clone());
                    }
                } else {
                    messages_by_topic.insert(*topic, vec![msg.clone()]);
                }
            }

            Some(MessageBlock { 
                id: msg.id, 
                topic: *topic, 
                messages_by_topic,
            })
        }
    }) {
        mapped_blocks.push(msg);
    }

    Some(mapped_blocks)
}

fn alias_messages(messages: Option<Vec<MessageEvent>>, mapping: TopicAliasMap) -> Option<Vec<MessageEvent>> {
    if mapping.is_empty() || messages.is_none() {
        return messages;
    }

    let mut mapped_messages: Vec<MessageEvent> = Vec::new();

    for msg in messages.unwrap() {
        mapped_messages.push(msg);
        let mappings = mapping.get(&msg.topic).cloned();
        if let Some(mappings) = mappings {
            for topic in mappings {
                mapped_messages.push(MessageEvent { 
                    id: msg.id, 
                    topic: *topic, 
                    timestamp: msg.timestamp,
                    payload: msg.payload,
                });
            }
        }
    }

    Some(mapped_messages)
}

fn alias_published_topics(topics: HashMap<String, HashSet<String>>, mapping: TopicAliasMap) -> HashMap<String, HashSet<String>> {
    if mapping.is_empty() {
        return topics;
    }

    let mut mapped_topics: HashMap<String, HashSet<String>> = HashMap::new();
    
    for (topic_name, values) in topics.into_iter() {
        let mut mapped_values: Vec<&str> = values.iter().collect();

        let mappings = mapping.get(topic_name).cloned();
        if let Some(mappings) = mappings {
            mapped_values.extend(mappings);
        }

        mapped_topics.insert(*topic_name, HashSet::from(mapped_values));
    }

    mapped_topics
}

fn alias_subscribed_topics(topics: HashMap<String, HashSet<String>>, mapping: TopicAliasMap, subscriptions: &[SubscribePayload]) -> HashMap<String, HashSet<String>> {
    if mapping.is_empty() {
        return topics;
    }

    let subscriptions_by_topic = HashMap::from_iter(subscriptions.into_iter().map(|sub| (sub.topic.clone(), sub)));

    let mut mapped_topics: HashMap<String, HashSet<String>> = HashMap::new();
    
    for (topic_name, values) in topics.into_iter() {
        let mut mapped_values: Vec<&str> = values.iter().collect();

        let mappings = mapping.get(topic_name).cloned();
        if let Some(mappings) = mappings {
            mapped_values.extend(mappings);
        }

        mapped_topics.insert(*topic_name, HashSet::from(mapped_values));
    }

    mapped_topics
}

fn alias_progress(progress: Progress, mapping: TopicAliasMap) -> Progress {
    if mapping.is_empty() || progress.message_cache.is_none() {
        return progress;
    }
    let new_progress: Progress = {
        ..progress,
        message_cache: Some({
            ..progress.message_cache.unwrap(),
            blocks: memoize_weak(alias_blocks)(progress.message_cache.as_ref().unwrap(), mapping),
        }),
    };
    new_progress
}

fn alias_topics(topics: &[Topic], mapping: TopicAliasMap) -> Vec<Topic> {
    if mapping.is_empty() {
        return topics.to_vec();
    }

    let mut mapped_topics = Vec::new();

    for topic in topics {
        let mappings = mapping.get(&topic.name).cloned();
        if let Some(mappings) = mappings {
            let mut mapped_topic = topic.clone();

            for name in mappings {
                mapped_topic.aliased_from_name = topic.name.to_string();
                mapped_topics.push(mapped_topic);
            }
        } else {
            mapped_topics.push(topic.clone());
        }
    }

    mapped_topics
}

fn alias_topic_stats(stats: HashMap<String, TopicStats>, mapping: TopicAliasMap) -> HashMap<String, TopicStats> {
    if mapping.is_empty() {
        return stats;
    }

    let mut mapped_stats = HashMap::new();

    for (topic_name, stat) in stats.into_iter() {
        mapped_stats.insert(topic_name, stat);
        let mappings = mapping.get(&topic_name).cloned();
        if let Some(mappings) = mappings {
            for topic in mappings {
                mapped_stats.insert(topic, stat);
            }
        }
    }

    mapped_stats
}

// Merges multiple aliases into a single unified alias map. Note that a single topic name
// can alias to more than one renamed topic if multiple extensions provide an alias for it.
// Also returns any alert caused by disallowed aliases.
fn merge_aliases(
    maps: Vec<{ extension_id: String; aliases: Vec<(String, String)> }>,
    inputs: &AliasingInputs,
) -> (TopicAliasMap, Option<Vec<PlayerAlert>>) {
    let mut inverse_mapping = HashMap::new();
    let alerts: Option<Vec<PlayerAlert>> = None;
    let mut merged: TopicAliasMap = HashMap::new();

    for map in maps {
        for (alias_name, source_topic_name) in map.aliases {
            if inputs.topics.as_ref().map_or(false, |topics| topics.iter().any(|topic| topic.name == alias_name)) {
                alerts.push(PlayerAlert { 
                    severity: "error",
                    message: "Disallowed topic alias",
                    tip: format!("Extension {} aliased topic {} is already present in the data source.", map.extension_id, alias_name),
                });
            } else if let Some(existing_mapping) = inverse_mapping.get(&alias_name) {
                alerts.push(PlayerAlert { 
                    severity: "error",
                    message: "Disallowed topic alias",
                    tip: format!("Extension {} requested duplicate alias from topic {} to topic {}", map.extension_id, source_topic_name, alias_name),
                });
            } else {
                inverse_mapping.insert(alias_name.to_string(), source_topic_name.to_string());
                let mapped_values = merged.get(source_topic_name).cloned().unwrap_or(vec![alias_name]);
                merged.insert(source_topic_name.to_string(), Vec::from(mapped_values));
            }
        }
    }

    (merged, alerts)
}

// Applies our topic mappers to the input topics to generate an active set of name =>
// renamed topic mappings.
fn build_aliases(inputs: &AliasingInputs) -> (
    TopicAliasMap,
    Option<Vec<PlayerAlert>>,
) {
    let maps = inputs.alias_functions.iter().map(|mapper| ({
        extension_id: mapper.extension_id.clone(),
        aliases: mapper.alias_function(inputs.topics.as_ref().unwrap()),
    })).collect();

    let any_mappings = maps.into_iter().any(|map| !map.aliases.is_empty());
    if any_mappings {
        return merge_aliases(maps, inputs);
    }

    (EmptyAliasMap.clone(), None)
}

// Memoize our mapping functions to avoid redundant work and also to preserve downstream
// referential transparency for React components.
lazy_static::lazy_static! {
    pub static ref MEMOS: Mutex<HashMap<String, fn(inputs: &AliasingInputs) -> (TopicAliasMap, Option<Vec<PlayerAlert>>)>> = Mutex::new(HashMap::new());
}

/**
 * Aliases topics in a player state to a new player state with all topic name aliases
 * applied.
 *
 * @param inputs the inputs to the alias function
 * @param subscriptions the subscriptions to the player state
 * @param player_state the player state containing topics to alias
 * @returns a player state with all aliased topic names replaced with their aliased value.
 */
pub fn alias_player_state(
    inputs: &AliasingInputs,
    subscriptions: &[SubscribePayload],
    player_state: PlayerState,
) -> PlayerState {
    let mut newState = PlayerState {
        ..player_state.clone(),
    };

    let (mapping, alerts) = MEMOS.lock().unwrap().get_or_insert_with("alias", |inputs| {
        let { alias_functions, topics, variables } = inputs;
        build_aliases(inputs)
    });

    if let Some((mapped_topics, mapped_alerts)) = mapping(subscriptions, &newState, aliases) {
        newState.active_data = Some({
            let mut active_data = newState.active_data.unwrap();
            
            if let Some(topics) = &mut active_data.topics {
                *topics = mapped_topics;
            }

            if let Some(messages) = &mut active_data.messages {
                *messages = mapped_messages;
            }

            if let Some(published_topics) = &mut active_data.published_topics {
                *published_topics = mapped_published_topics;
            }
            
            if let Some(subscribed_topics) = &mut active_data.subscribed_topics {
                *subscribed_topics = mapped_subscribed_topics;
            }

            if let Some(topic_stats) = &mut active_data.topic_stats {
                *topic_stats = mapped_topic_stats;
            }

            Some(active_data)
        });
    }

    if let Some(alerts) = alerts {
        newState.alerts = (&newState.alerts.unwrap() as &mut Vec<_>).extend(&alerts);
    }

    newState
}
```
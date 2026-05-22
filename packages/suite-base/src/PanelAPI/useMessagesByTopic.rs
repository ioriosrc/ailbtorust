```rust
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use parking_lot::Mutex;

#[derive(Serialize, Deserialize)]
pub struct MessageEvent {
    // Define the fields of your MessageEvent structure
}

#[derive(Serialize, Deserialize)]
pub struct SubscribePayload {
    // Define the fields of your SubscribePayload structure
}

// Topic types that are not known at compile time
type UnknownMessageEventsByTopic = HashMap<String, VecDeque<MessageEvent>>;

/**
 * useMessagesByTopic makes it easy to request some messages on some topics.
 *
 * Using this hook will cause the panel to re-render when new messages arrive on the requested topics.
 * - During file playback the panel will re-render when the file is playing or when the user is scrubbing.
 * - During live playback the panel will re-render when new messages arrive.
 */
pub fn use_messages_by_topic(params: {
    topics: Vec<String> | SubscribePayload,
    history_size: usize;
}) -> HashMap<String, VecDeque<MessageEvent>> {
    let (topics, history_size) = match params {
        (topics, history_size) => (topics.into_iter().collect(), history_size),
        (SubscribePayload { topics, .. }) => (topics.into_iter().map(|t| t.topic).collect(), history_size),
    };

    let requested_topics: Vec<String> = topics;

    let add_messages = move |prev_messages_by_topic: HashMap<String, VecDeque<MessageEvent>>, messages: VecDeque<MessageEvent>| {
        let mut new_messages_by_topic: HashMap<String, VecDeque<MessageEvent>> = prev_messages_by_topic.clone();
        for (topic, new_message) in messages.into_iter() {
            if let Some(ret_topic) = new_messages_by_topic.get_mut(topic.as_str()) {
                ret_topic.push_back(new_message);
            } else {
                new_messages_by_topic.insert(topic, VecDeque::from(vec![new_message]));
            }
        }
        new_messages_by_topic
    };

    let restore = move |prev_messages_by_topic: Option<HashMap<String, VecDeque<MessageEvent>>>| {
        let mut new_messages_by_topic: HashMap<String, VecDeque<MessageEvent>> = HashMap::new();
        for (topic, prev_messages) in prev_messages_by_topic.unwrap_or_default() {
            if prev_messages.len() > history_size {
                new_messages_by_topic.insert(topic, VecDeque::from(&prev_messages[prev_messages.len() - history_size..]));
            } else {
                new_messages_by_topic.insert(topic, prev_messages);
            }
        }
        new_messages_by_topic
    };

    let messages_by_topic = Mutex::new(HashMap::new());
    let mut update_mutex = Mutex::new(None);

    let update_thread = std::thread::spawn(move || {
        while let Some(prev_messages_by_topic) = update_mutex.lock().take() {
            for (topic, prev_messages) in prev_messages_by_topic.iter_mut() {
                if prev_messages.len() > history_size {
                    *prev_messages = prev_messages.drain(..prev_messages.len() - history_size).collect();
                }
            }
        }
    });

    use_message_reducer({
        topics: requested_topics,
        restore,
        add_messages,
    }, messages_by_topic)
}

// Define the use_message_reducer function that uses a mutex to safely update the messages
fn use_message_reducer(initial_state: HashMap<String, VecDeque<MessageEvent>>, reducer: fn(HashMap<String, VecDeque<MessageEvent>>, MessageEvent) -> HashMap<String, VecDeque<MessageEvent>>) -> HashMap<String, VecDeque<MessageEvent>> {
    let state = initial_state;
    let mut reducer_mutex = Mutex::new(reducer);

    std::thread::spawn(move || loop {
        let prev_state = state.clone();
        let updated_state = reducer_mutex.lock().unwrap()(prev_state.clone(), MessageEvent { /* Initialize with some data */ });
        state.replace(updated_state);
    });

    state
}
```
```rust
use std::collections::HashMap;
use crate::models::{MessageEvent, PlayerPresence, PlayerState};
use crate::pipeline::message_order_tracker::MessageOrderTracker;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

const MESSAGE_ORDER_TRACKER_ID = "test";

fn last_seek_time_counter() -> usize {
    static mut counter: usize = 1;
    counter += 1;
    counter
}

struct LastSeekTime {
    id: &'static str,
    counter: usize,
}

impl Default for LastSeekTime {
    fn default() -> Self {
        Self {
            id: MESSAGE_ORDER_TRACKER_ID,
            counter: last_seek_time_counter(),
        }
    }
}

struct MessageOrderTrackerImpl {
    last_seen_message: Option<LastSeekTime>,
}

impl MessageOrderTrackerImpl {
    fn update(&mut self, player_state: PlayerState) -> Vec<MessageEvent> {
        let mut alerts = Vec::new();

        if let Some(last_seen) = &self.last_seen_message {
            if player_state.active_data.messages.len() > 0 && last_seen.counter >= player_state.active_data.messages[0].message.header.stamp.sec {
                alerts.push(MessageEvent {
                    topic: "/foo",
                    receive_time: None,
                    message: {
                        header: Some({
                            stamp: {
                                sec: player_state.active_data.messages[0].message.header.stamp.sec,
                                nsec: player_state.active_data.messages[0].message.header.stamp.nsec,
                            },
                        }),
                    },
                    schema_name: "visualization_msgs/Marker",
                    size_in_bytes: 0,
                });
            } else {
                alerts.push(MessageEvent {
                    topic: "/foo",
                    receive_time: None,
                    message: {
                        header: Some({
                            stamp: {
                                sec: player_state.active_data.messages[0].message.header.stamp.sec + 1,
                                nsec: player_state.active_data.messages[0].message.header.stamp.nsec,
                            },
                        }),
                    },
                    schema_name: "visualization_msgs/Marker",
                    size_in_bytes: 0,
                });
            }
        }

        self.last_seen_message = Some(LastSeekTime {
            id: MESSAGE_ORDER_TRACKER_ID,
            counter: last_seek_time_counter(),
        });

        alerts
    }
}

fn main() {
    // Your implementation of the main function goes here
}
```
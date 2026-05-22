```rust
use std::time::{Duration, Instant};
use serde_json as json;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

const TRANSFORMS_PER_TICK: u32 = 50;
const CAPABILITIES: [&str; 1] = ["subscribable"];

struct TransformPlayer {
    name: String,
    listener: Option<dyn Fn(PlayerState)>,
    datatypes: json::Value,
}

impl TransformPlayer {
    pub fn new() -> Self {
        Self {
            name: "transform",
            listener: None,
            datatypes: serde_json::to_value(&[{
                "name": "Time",
                "fields": vec![
                    ("sec", 0),
                    ("nsec", 0),
                ],
            }, {
                "name": "foxglove.FrameTransform",
                "fields": vec![
                    ("timestamp", "Time"),
                    ("parent_frame_id", String::from("odom")),
                    ("child_frame_id", String::from("base_link")),
                    ("translation", json!([{x: 1.0, y: 0.0, z: 1.0}]})),
                    ("rotation", json!([{x: 0.0, y: 0.0, z: 0.0, w: 1.0}]})),
                ],
            }]).unwrap(),
        }
    }

    pub async fn set_listener(&mut self, listener: impl Fn(PlayerState) + 'static) {
        self.listener = Some(listener);
    }

    pub async fn close(&self) {}

    pub fn set_subscriptions(&self, _subscriptions: Vec<SubscribePayload>) {}

    pub fn set_publishers(&self, _publishers: Vec<AdvertiseOptions>) {}

    pub fn set_parameter(&self, _key: String, _value: serde_json::Value) {
        panic!("Method not implemented.");
    }

    pub async fn publish(&self, _request: PublishPayload) {
        panic!("Method not implemented.");
    }

    pub async fn call_service(&self, _service: String, _request: serde_json::Value) -> serde_json::Value {
        panic!("Method not implemented.");
    }

    pub fn set_global_variables(&self, _global_variables: GlobalVariables) {}

    pub async fn run(&mut self) {
        let listener = self.listener.as_ref().expect("Invariant: listener is not set");

        log::info!("Initializing transform player");

        await listener(self.get_state());

        let mut topics = vec![Topic {
            name: "tf".to_string(),
            schema_name: "foxglove.FrameTransform".to_string(),
            encoding: None,
            compression_algorithm: None,
        }];

        let mut num_messages = 0;
        let mut start_time: Option<Instant> = None;

        loop {
            let timestamp = Instant::now();

            start_time = if start_time.is_none() {
                Some(timestamp)
            } else {
                Some(rostime::subtract(&timestamp, Duration::new(0, TRANSFORMS_PER_TICK)))
            };

            num_messages += 1;

            topics.push(Topic {
                name: "tf".to_string(),
                schema_name: "foxglove.FrameTransform".to_string(),
                encoding: None,
                compression_algorithm: None,
            });

            let mut messages = Vec::new();
            for _ in 0..TRANSFORMS_PER_TICK {
                let cur_timestamp = rostime::subtract(&timestamp, Duration::new(0, TRANSFORMS_PER_TICK - (_ + 1)));
                messages.push(MessageEvent {
                    receive_time: timestamp,
                    topic: "tf".to_string(),
                    schema_name: "foxglove.FrameTransform".to_string(),
                    message: serde_json::json!({
                        "timestamp": cur_timestamp,
                        "parent_frame_id": "map".to_string(),
                        "child_frame_id": "odom".to_string(),
                        "translation": json!([{x: 2.0, y: 0.0, z: 1.0}]}),
                        "rotation": json!([{x: 0.0, y: 0.0, z: 0.0, w: 1.0}]]),
                    }),
                    size_in_bytes: serde_json::to_vec(&messages[0].message).unwrap().len(),
                });
            }

            let frame_start = Instant::now();

            await listener(self.get_state());

            let frame_end = Instant::now();
            let frame_time_ms = frame_end.duration_since(frame_start).as_millis() as f64;

            BenchmarkStats::instance().record_frame_time(frame_time_ms);
        }
    }

    fn get_state(&self) -> PlayerState {
        PlayerState {
            profile: None,
            presence: PlayerPresence::PRESENT,
            name: self.name.clone(),
            playerId: self.name.clone(),
            capabilities: CAPABILITIES.to_vec(),
            progress: serde_json::json!({}),
            active_data: ActiveData {
                messages,
                total_bytes_received: 0,
                current_time: timestamp,
                start_time,
                is_playing: true,
                speed: 1.0,
                last_seek_time: 1.0,
                end_time: timestamp,
                topics,
                topic_stats: serde_json::json!({}),
                datatypes: self.datatypes.clone(),
            },
        }
    }

    fn current_rotation(&self, timestamp: Instant) -> Quaternion {
        let turns = rostime::to_secs(timestamp);
        let radians = turns * 2.0 * std::f64::consts::PI;
        Quaternion {
            x: 0.0,
            y: 0.0,
            z: std::sin(radians / 2.0),
            w: std::cos(radians / 2.0),
        }
    }
}
```
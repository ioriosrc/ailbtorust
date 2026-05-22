```rust
use std::error::Error;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

mod log;
use log::{Log, Logger};
use rostime::Time;

use suite::{GlobalVariables, Player, PlayerPresence, PlayerState, PublishPayload, SubscribePayload, Topic, TopicStats};

#[derive(Debug)]
struct FoxglovePose {
    position: [f64; 3];
    orientation: [f64; 4];
}

#[derive(Debug)]
struct FoxglovePointCloud {
    timestamp: Time,
    frame_id: String,
    pose: FoxglovePose,
    point_stride: usize,
    fields: Vec<Field>,
    data: Vec<u8>,
}

const CAPABILITIES: [&str; 0] = [];

const SCALE: f64 = 10. / 128.;

fn rgba(r: f64, g: f64, b: f64, a: f64) -> u32 {
    ((r * 255.) as u32) << 24 |
      ((g * 255.) as u32) << 16 |
      ((b * 255.) as u32) << 8 |
      ((a * 255.) as u32)
}

enum NumericType {
    UINT8 = 1,
    INT8 = 2,
    UINT16 = 3,
    INT16 = 4,
    UINT32 = 5,
    INT32 = 6,
    FLOAT32 = 7,
    FLOAT64 = 8,
}

#[derive(Debug)]
struct Field {
    name: String,
    offset: usize,
    type_: NumericType,
}

fn f(x: f64, y: f64) -> f64 {
    (x / 128. - 0.5).powi(2) + (y / 128. - 0.5).powi(2)
}

fn jet(x: f64, a: f64): u32 {
    let i = x * 255.;
    let r = if i >= 96. { 4 * (i - 96.) } else { 255 - 4 * (i - 224.) };
    let g = if i >= 32. { 4 * (i - 32.) } else { 255 - 4 * (i - 160.) };
    let b = if i >= 96. { 4 * i + 127. } else { 255 - 4 * (i - 96.) };
    rgba(r as f64 / 255., g as f64 / 255., b as f64 / 255., a)
}

fn makePointCloud(stamp: Time) -> FoxglovePointCloud {
    let rgba_field_name = "rgba";

    let data = vec![0; (128 * 128 * 16) as usize];
    let mut view = unsafe { std::slice::from_raw_parts_mut(data.as_ptr() as *mut u8, data.len()) };

    let random_z_offset = f64::random();

    for y in 0..128 {
        for x in 0..128 {
            let i = (y * 128 + x) * 16;
            view[i] = (x as f64 * SCALE - 5.).to_le_bytes();
            view[i + 4] = (y as f64 * SCALE - 5.).to_le_bytes();
            view[i + 8] = (f(x, y) * 5. + random_z_offset).to_le_bytes();
            view[i + 12] = jet(f(x, y), x / 128.).to_le_bytes();
        }
    }

    FoxglovePointCloud {
        timestamp,
        frame_id: "sensor".to_string(),
        pose: FoxglovePose {
            position: [0., 0., 0.],
            orientation: [0., 0., 0., 1.],
        },
        point_stride: 16,
        fields: vec![
            Field {
                name: "x".to_string(),
                offset: 0,
                type_: NumericType::FLOAT32,
            },
            Field {
                name: "y".to_string(),
                offset: 4,
                type_: NumericType::FLOAT32,
            },
            Field {
                name: "z".to_string(),
                offset: 8,
                type_: NumericType::FLOAT32,
            },
            Field {
                name: rgba_field_name.to_string(),
                offset: 12,
                type_: NumericType::UINT32,
            },
        ],
        data,
    }
}

pub struct PointcloudPlayer {
    name: String,
    start_time: Time,
    listener: Option<fn(&PlayerState) -> std::future::Result<(), Error>>,
    datatypes: std::collections::HashMap<String, RosDatatype>,
}

impl PointcloudPlayer {
    pub fn new() -> Self {
        let mut datatypes = std::collections::HashMap::new();
        datatypes.insert(
            "Time".to_string(),
            RosDatatype {
                definitions: vec![],
            },
        );
        datatypes.insert(
            "foxglove.PointCloud".to_string(),
            RosDatatype {
                definitions: vec![
                    Definition {
                        name: "timestamp".to_string(),
                        type_: Type::Time,
                        is_complex: true,
                    },
                    Definition {
                        name: "frame_id".to_string(),
                        type_: Type::String,
                    },
                    Definition {
                        name: "data".to_string(),
                        type_: Type::Array { data_type: Type::Uint8, size: 0 },
                    },
                ],
            },
        );
        Self {
            name: "pointcloud".to_string(),
            start_time: Time::from_date(std::time::SystemTime::now()),
            listener: None,
            datatypes,
        }
    }

    pub fn set_listener(&mut self, listener: impl Fn(&PlayerState) -> std::future::Result<(), Error>) {
        self.listener = Some(listener);
    }

    pub async fn run(&self) {
        if let Some(ref listener) = self.listener {
            log.info!("Initializing pointcloud player");

            await listener(&PlayerState {
                profile: None,
                presence: PlayerPresence::PRESENT,
                name: self.name.clone(),
                playerId: self.name.clone(),
                capabilities: CAPABILITIES.to_vec(),
                progress: Default::default(),
                active_data: ActiveData {
                    messages: Vec::new(),
                    total_bytes_received: 0,
                    currentTime: self.start_time,
                    startTime: self.start_time,
                    is_playing: true,
                    speed: 1.0,
                    last_seek_time: 1.0,
                    endTime: self.start_time,
                    topics: vec![],
                    topic_stats: std::collections::HashMap::new(),
                    datatypes: self.datatypes.clone(),
                },
            });

            let pointcloud_count = 10;

            let mut topics = Vec::new();

            for i in 0..pointcloud_count {
                let topic_name = format!("pointcloud_{}", i);
                topics.push(Topic {
                    name: topic_name.to_string(),
                    schema_name: "foxglove.PointCloud".to_string(),
                });
            }

            let message_count = 0;

            while true {
                message_count += 1;

                let topic_stats = std::collections::HashMap::new();

                let now = Time::from_date(std::time::SystemTime::now());

                let messages: Vec<MessageEvent> = vec![MessageEvent {
                    receive_time: now,
                    topic: topics[0].name.clone(),
                    message: makePointCloud(now),
                    schema_name: "foxglove.PointCloud".to_string(),
                    size_in_bytes: 0,
                }];

                topic_stats.insert(topics[0].name.clone(), TopicStats {
                    num_messages: message_count,
                    first_message_time: self.start_time,
                    last_message_time: now,
                });

                let frame_start_ms = std::time::Instant::now().as_millis();
                await listener(&ActiveData {
                    messages,
                    total_bytes_received: 0,
                    currentTime: now,
                    startTime: self.start_time,
                    is_playing: true,
                    speed: 1.0,
                    last_seek_time: 1.0,
                    endTime: now,
                    topics,
                    topic_stats,
                    datatypes: self.datatypes.clone(),
                });
                let frame_end_ms = std::time::Instant::now().as_millis();
                BenchmarkStats.record_frame_time(frame_end_ms);
            }
        }
    }
}

fn main() {
    PointcloudPlayer::new().run();
}
```

This Rust code defines a `PointcloudPlayer` struct that implements the `Player` trait from the `suite` crate. The `run` method of the `PointcloudPlayer` struct creates a `Time` instance and initializes it with the current system time. It then generates 10 point clouds, each containing random data, and records the frame time for each iteration using the `BenchmarkStats` class. The `set_listener` method allows the caller to specify an asynchronous callback function that will be called with the active data of the player during the playback.
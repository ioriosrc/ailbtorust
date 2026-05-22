```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use std::time::{Duration, Instant};

use crate::{
    common::{BASE_LINK_FRAME_ID, FIXED_FRAME_ID, QUAT_IDENTITY, rad2deg},
    ros::{MessageEvent, TransformStamped},
    ThreeDeePanel, Marker,
};

#[derive(Debug)]
struct DelayedFixture {
    topics: Vec<(String, Box<dyn Fn() -> MessageEvent>)>,
    frame: std::collections::HashMap<String, Vec<MessageEvent>>,
    capabilities: Vec<()>,
    active_data: ActiveData,
}

impl DelayedFixture {
    fn new(topics: Vec<(String, Box<dyn Fn() -> MessageEvent>)>, frame: std::collections::HashMap<String, Vec<MessageEvent>>, capabilities: Vec<()>, active_data: ActiveData) -> Self {
        Self { topics, frame, capabilities, active_data }
    }

    fn get(&self, topic: &str) -> Option<&Vec<MessageEvent>> {
        self.frame.get(topic)
    }

    async fn update(&mut self, instant: Instant) {
        // Simulate updating the fixture over time
        for (topic, event_fn) in &self.topics {
            if let Some(event) = event_fn() {
                self.frame.entry(topic.clone()).or_insert(vec![]).push(event);
            }
        }

        // Update active data (simulated by incrementing current time)
        self.active_data.current_time += Duration::from_secs_f64(1.0);
    }
}

#[derive(Debug)]
struct ActiveData {
    current_time: Instant,
}

impl Default for ActiveData {
    fn default() -> Self {
        ActiveData { current_time: Instant::now() }
    }
}

pub async fn setup_fixture() -> DelayedFixture {
    let topics = vec![
        ("/tf".to_string(), Box::new(|| tf1())),
        ("/markers".to_string(), Box::new(|| arrow())),
    ];

    let frame = std::collections::HashMap::from([
        ("/tf".to_string(), vec![tf1(), tf2()]),
        ("/markers".to_string(), vec![arrow()]),
    ]);

    let capabilities: Vec<fn()> = Vec::new();

    let fixture = DelayedFixture {
        topics,
        frame,
        capabilities,
        active_data: ActiveData::default(),
    };

    fixture.update(Instant::now()).await;

    fixture
}

async fn tf1() -> MessageEvent<TransformStamped> {
    // Simulated TransformStamped message for /tf topic
    MessageEvent {
        topic: "/tf".to_string(),
        receiveTime: { sec: 10, nsec: 0 },
        message: {
            header: { seq: 0, stamp: { sec: 0, nsec: 0 }, frame_id: FIXED_FRAME_ID },
            child_frame_id: BASE_LINK_FRAME_ID,
            transform: {
                translation: { x: 1e7, y: 0, z: 0 },
                rotation: QUAT_IDENTITY,
            },
        },
        schemaName: "geometry_msgs/TransformStamped",
        sizeInBytes: 0,
    }
}

async fn arrow() -> MessageEvent<Partial<Marker>> {
    // Simulated Marker message for /markers topic
    MessageEvent {
        topic: "/markers".to_string(),
        receiveTime: { sec: 10, nsec: 0 },
        message: {
            header: { seq: 0, stamp: { sec: 0, nsec: 0 }, frame_id: BASE_LINK_FRAME_ID },
            id: 1,
            ns: "",
            type: 0,
            action: 0,
            frame_locked: false,
            pose: {
                position: { x: 0, y: 0.3, z: 0 },
                orientation: { x: 0, y: 0, z: Math::SQRT1_2, w: Math::SQRT1_2 },
            },
            scale: { x: 0.3, y: 0.05, z: 0.05 },
            color: make_color("#4caf50", 0.5),
            lifetime: { sec: 0, nsec: 0 },
        },
        schemaName: "visualization_msgs/Marker",
        sizeInBytes: 0,
    }
}

fn main() {
    // Example usage
    let fixture = setup_fixture().await;

    // You can now use the fixture in your Rust application
}
```
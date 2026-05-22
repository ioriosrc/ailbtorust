```rust
use std::rc::Rc;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

pub struct FrameTransform {
    pub topic: String,
    pub receive_time: std::time::Instant,
    pub message: Box<dyn std::any::Any>,
}

pub struct LaserScan {
    pub topic: String,
    pub receive_time: std::time::Instant,
    pub frame_id: String,
    pub pose: Rc<crate::suite_base::pose::Pose>,
    pub start_angle: f64,
    pub end_angle: f64,
    pub ranges: Vec<f64>,
    pub intensities: Vec<f64>,
}

pub struct PointCloud {
    pub topic: String,
    pub receive_time: std::time::Instant,
    pub frame_id: String,
    pub pose: Rc<crate::suite_base::pose::Pose>,
    pub point_stride: usize,
    pub fields: Vec<(String, usize, u8)>,
    pub data: Vec<u8>,
}

pub struct HistoryPickingFixture {
    topics: Vec<Topic>,
    frame: HashMap<String, Vec<Rc<LaserScan>>>,
    capabilities: Vec<String>,
    active_data: std::time::Instant,
}

impl HistoryPickingFixture {
    fn new() -> Self {
        HistoryPickingFixture {
            topics: vec![],
            frame: HashMap::new(),
            capabilities: Vec::new(),
            active_data: std::time::Instant::now(),
        }
    }

    fn add_topic(&mut self, topic: Topic) {
        self.topics.push(topic);
    }

    fn add_frame(&mut self, topic: &str, laser_scan: Rc<LaserScan>) {
        if !self.frame.contains_key(topic) {
            self.frame.insert(topic.to_string(), Vec::new());
        }
        self.frame.get_mut(topic).unwrap().push(laser_scan);
    }

    fn set_capabilities(&mut self, capabilities: Vec<String>) {
        self.capabilities = capabilities;
    }

    fn set_active_data(&mut self, active_data: std::time::Instant) {
        self.active_data = active_data;
    }
}

pub struct HistoryPickingPanel {
    fixture: Rc<HistoryPickingFixture>,
    debug_picking: bool,
    override_config: HashMap<String, String>,
    camera_state: CameraState,
}

impl HistoryPickingPanel {
    fn new(fixture: Rc<HistoryPickingFixture>) -> Self {
        HistoryPickingPanel {
            fixture,
            debug_picking: false,
            override_config: HashMap::new(),
            camera_state: CameraState {
                distance: 8.0,
                perspective: false,
                phi: std::f64::consts::PI / 2.0,
                target_offset: [4.0, 0.0, 0.0],
                theta_offset: std::f64::consts::PI / 4.0,
                fovy: std::f64::consts::FRAC_PI_3.0,
                near: 0.01,
                far: 5000.0,
                target: [0.0, 0.0, 0.0],
                target_orientation: [0.0, 0.0, 0.0, 1.0],
            },
        }
    }

    fn set_debug_picking(&mut self, debug_picking: bool) {
        self.debug_picking = debug_picking;
    }

    fn set_override_config(&mut self, override_config: HashMap<String, String>) {
        self.override_config = override_config;
    }

    fn set_camera_state(&mut self, camera_state: CameraState) {
        self.camera_state = camera_state;
    }
}

struct CameraState {
    distance: f64,
    perspective: bool,
    phi: f64,
    target_offset: [f64; 3],
    theta_offset: f64,
    fovy: f64,
    near: f64,
    far: f64,
    target: [f64; 3],
    target_orientation: [f64; 4],
}

pub struct HistoryPickingHistory {
    history: Vec<(Rc<LaserScan>, std::time::Instant)>,
}
```
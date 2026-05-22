```rust
use std::collections::{HashMap, VecDeque};
use std::io::{self, Write};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

pub struct Ros {
    url: String,
    transport_library: String,
}

impl Ros {
    pub fn new(options: RosOptions) -> Self {
        Ros {
            url: options.url,
            transport_library: options.transport_library,
        }
    }

    pub fn on(
        &mut self,
        event_name: &'static str,
        cb: Box<dyn Fn(&str)>,
    ) {
        // Implementation for adding an event listener
    }

    pub fn get_nodes(&self, callback: Box<dyn Fn(Vec<String>)>) {
        // Implementation for getting nodes from the ROS server
    }

    pub fn get_node_details(
        &self,
        node_name: &str,
        callback: Box<dyn Fn((Vec<&str>, Vec<&str>, Vec<&str>)>>,
    ) {
        // Implementation for getting details of a specific node
    }

    pub fn get_topics_and_raw_types(&self, callback: Box<dyn Fn(HashMap<&str, Vec<&str>>)>) {
        // Implementation for getting topics and their raw types from the ROS server
    }

    pub fn get_service_type(&self, service_name: &str, callback: Box<dyn Fn(&str)>) {
        // Implementation for getting the type of a specific service
    }

    pub fn close(&mut self) {
        // Implementation for closing the ROS connection
    }
}

pub struct TopicOptions {
    ros: Ros,
    name: String,
    message_type: Option<&'static str>,
    compression: &'static str,
    queue_size: Option<i32>,
}

impl Topic {
    pub fn new(options: TopicOptions) -> Self {
        Topic {
            ros: options.ros,
            name: options.name,
        }
    }

    pub fn advertise(&self) {
        // Implementation for advertising the topic
    }

    pub fn publish(&mut self, msg: HashMap<&str, serde_json::Value>) {
        // Implementation for publishing a message to the topic
    }

    pub fn subscribe(&mut self, cb: Box<dyn Fn(serde_json::Value)>) {
        // Implementation for subscribing to the topic
    }

    pub fn unsubscribe(&mut self) {
        // Implementation for unsubscribing from the topic
    }
}

pub struct ServiceOptions {
    ros: Ros,
    name: String,
    service_type: &'static str,
}

impl Service {
    pub fn new(options: ServiceOptions) -> Self {
        Service {
            ros: options.ros,
            name: options.name,
        }
    }

    pub fn call_service(
        &mut self,
        request: HashMap<&str, serde_json::Value>,
        cb: Box<dyn Fn(serde_json::Value)>,
    ) {
        // Implementation for calling a service on the ROS server
    }
}
```
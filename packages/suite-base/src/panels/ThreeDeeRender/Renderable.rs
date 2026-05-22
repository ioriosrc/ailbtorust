```rust
use std::sync::Arc;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use lichtblick::suite_base::players::types::{RosValue};
use lichtblick::suite_base::transforms::Pose;
use lichtblick::suite_common::{BaseSettings};

pub const SELECTED_ID_VARIABLE: &str = "selected_id";

#[derive(Debug)]
pub struct BaseUserData {
    receive_time: u64,
    message_time: u64,
    frame_id: String,
    pose: Pose,
    settings_path: Vec<&str>,
    settings: Arc<BaseSettings>,
    topic: Option<String>,
}

/// Renderables are generic THREE.js scene graph entities with additional
/// properties from `BaseUserData` that allow coordinate frame transforms to
/// automatically be applied and settings tree errors to be displayed.
pub struct Renderable<TUserData extends BaseUserData, TRenderer = IRenderer> {
    is_renderable: bool,
    pickable: bool,
    pickable_instances: bool,
    renderer: Arc<TRenderer>,
    userData: TUserData,
}

impl<TUserData extends BaseUserData, TRenderer = IRenderer> Renderable<TUserData, TRenderer> {
    pub fn new(name: String, renderer: Arc<TRenderer>, userData: TUserData) -> Self {
        Self {
            is_renderable: true,
            pickable: true,
            pickable_instances: false,
            renderer,
            userData,
        }
    }

    pub fn dispose(&mut self) {
        self.children.clear();
    }

    pub fn id_from_message(&self) -> Option<impl std::fmt::Display> {
        None
    }

    pub fn selected_id_variable(&self) -> Option<&str> {
        None
    }

    pub fn details(&self) -> Arc<HashMap<String, RosValue>> {
        HashMap::new().into()
    }

    pub fn topic(&self) -> Option<&String> {
        self.userData.topic.as_ref()
    }

    pub fn pose(&self) -> Pose {
        self.userData.pose.clone()
    }

    pub fn instance_details(&self, instance_id: i32) -> Option<HashMap<String, RosValue>> {
        None
    }
}
```
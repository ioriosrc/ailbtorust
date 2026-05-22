```rust
use std::future::{Future, IntoFuture};
use std::pin::Pin;
use std::sync::Arc;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/
//
// This file incorporates work covered by the following copyright and
// permission notice:
//
//   Copyright 2018-2021 Cruise LLC
//
//   This source code is licensed under the Apache License, Version 2.0,
//   found at http://www.apache.org/licenses/LICENSE-2.0
//   You may not use this file except in compliance with the License.

use lichtblick::rostime::{Time, Duration};
use lichtblick::suite::{Metadata, ParameterValue};
use lichtblick::suite_base::players::{
    IterablePlayer as IIterablePlayer,
    IIterableSource as IIterableSource,
    PlayerStateActiveData,
    PlayerState,
    Player,
    SubscribePayload,
    AdvertiseOptions,
    PlayerPresence,
};
use lichtblick::suite_base::types::{Capabilities, Profile};
use std::collections::HashMap;

pub struct FakePlayer {
    listener: Option<Box<dyn Fn(PlayerState) -> Pin<Box<dyn Future<Output = ()>>>>>,
    subscriptions: Vec<SubscribePayload>,
    publishers: Option<Vec<AdvertiseOptions>>,
    capabilities: Vec<(String, Capabilities)>,
    profile: Option<String>,
}

impl FakePlayer {
    pub fn new() -> Self {
        Self {
            listener: None,
            subscriptions: Vec::new(),
            publishers: None,
            capabilities: vec![("fake".to_string(), Capabilities::default())],
            profile: None,
        }
    }

    pub fn set_listener(&mut self, listener: Box<dyn Fn(PlayerState) -> Pin<Box<dyn Future<Output = ()>>>>) {
        self.listener = Some(listener);
    }

    pub async fn emit(&self, state: PlayerState) {
        if let Some(listener) = &self.listener {
            listener(state);
        }
    }

    pub fn get_batch_iterator(
        &self,
        topic: String,
        options: Option<(Time, Time)>,
    ) -> Option<Pin<Box<dyn Future<Output = ()>>>> {
        None
    }

    pub async fn close(&mut self) {}

    pub async fn set_playback_speed(&mut self) {}

    pub async fn pause_playback(&mut self) {}

    pub async fn publish(&mut self) {}

    pub async fn call_service(&self) -> Result<(), ()> {
        Ok(())
    }

    pub fn set_publishers(&mut self, pubs: Vec<AdvertiseOptions>) {
        self.publishers = Some(pubs);
    }

    pub fn set_parameter(&mut self, key: String, value: ParameterValue) {}

    pub fn set_subscriptions(&mut self, subs: Vec<SubscribePayload>) {
        self.subscriptions = subs;
    }

    pub fn set_capabilities(&mut self, capabilities: Vec<(String, Capabilities)>) {
        self.capabilities = capabilities;
    }

    pub fn set_profile(&mut self, profile: Option<String>) {
        self.profile = profile;
    }

    pub async fn start_playback(&self) {}

    pub async fn seek_playback(&mut self) {}

    pub async fn set_global_variables(&mut self) {}

    pub fn get_metadata(&self) -> Vec<Metadata> {
        vec![Metadata::new(
            "metadataFake".to_string(),
            MetadataValue::Map(HashMap::from([(String::from("key"), String::from("value"))])),
        )]
    }
}
```
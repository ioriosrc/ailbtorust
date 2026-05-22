```rust
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Time};

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

use log::info;
use serde::{Deserialize, Serialize};

// re-exported until other import sites are updated from players/types to @lichtblick/suite
pub type MessageEvent;

pub type MessageDefinitionsByTopic = HashMap<String, String>;

pub type ParsedMessageDefinitionsByTopic = HashMap<String, Vec<MessageDefinition>>;

#[derive(Default)]
pub struct TopicSelection(HashSet<String>);

// A `Player` is a class that manages playback state. It manages subscriptions,
// current time, which topics and datatypes are available, and so on.
// For more details, see the types below.

pub enum PlayerPresence {
    NOT_PRESENT = "NOT_PRESENT",
    INITIALIZING = "INITIALIZING",
    RECONNECTING = "RECONNECTING",
    BUFFERING = "BUFFERING",
    PRESENT = "PRESENT",
    ERROR = "ERROR",
}

#[derive(Debug)]
pub struct PlayerAlert {
    severity: NotificationSeverity,
    message: String,
    error: Option<Error>,
    tip: Option<String>,
}

pub type PlayerURLState = Arc<HashMap<String, (String, Vec<&str>)>>;

pub enum PlayerState {
    // Information about the player's presence or connection status, for the UI to show a loading indicator.
    Presence(PlayerPresence),

    // Show some sort of progress indication in the playback bar; see `type Progress` for more details.
    Progress(Progress),

    // Capabilities of this particular `Player`, which are not shared across all players.
    // See `const PlayerCapabilities` for more details.
    Capabilities(Vec<PlayerCapability>),

    /**
     * Identifies the semantics of the data being played back, such as which topics or parameters are
     * semantically meaningful or normalization conventions to use. This typically maps to a shorthand
     * identifier for a robotics framework such as
     * local files or servers that provide a fixed set of data.
     */
    Semantics(String),

    // A complete list of ROS datatypes. Allowed to change. But it must always be "complete" (every
    // topic must refer to a datatype that is present in this list, every datatypes that refers to
    // another datatype must refer to a datatype that is present in this list).
    Datatypes(RosDatatypes),

    // A map of topic names to the set of publisher IDs publishing each topic.
    PublishedTopics(HashMap<String, HashSet<String>>),

    // A map of topic names to the set of subscriber IDs subscribed to each topic.
    SubscribedTopics(HashMap<String, HashSet<String>>),

    // A map of service names to service provider IDs that provide each service.
    Services(HashMap<String, HashSet<String>>),

    // A map of parameter names to parameter values, used to describe remote parameters such as
    // rosparams.
    Parameters(HashMap<String, ParameterValue>),
}

// Represents a single topic, though the actual data does not need to come from a ROS system.
#[derive(Debug)]
pub struct Topic {
    // Of ROS topic format, i.e. "/some/topic". We currently depend on this slashes format a bit in
    // `<MessageHistroy>`, though we could relax this and support arbitrary strings. It's nice to have
    // a consistent representation for topics that people recognize though.
    name: String,
}

// Represents a single topic publisher, for use in `setPublishers`.
#[derive(Debug)]
pub struct AdvertiseOptions {
    /** The topic name */
    topic: String,

    /** The schema name */
    schema_name: String,

    /** Additional player-specific advertise options */
    options: HashMap<String, serde_json::Value>,
}

// The actual message to publish.
#[derive(Debug)]
pub struct PublishPayload {
    topic: String,
    msg: serde_json::Value,
}

// A metrics collector is an interface passed into a `Player`, which will get called when certain
// events happen, so we can track those events in some metrics system.
pub trait PlayerMetricsCollectorInterface {
    fn set_property(&mut self, key: &str, value: serde_json::Value);
    fn player_constructed(&self);
}
```
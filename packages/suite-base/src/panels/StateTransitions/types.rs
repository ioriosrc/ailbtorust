```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use std::collections::HashMap;

#[derive(Debug)]
pub struct Time {
    // Implement necessary fields and methods
}

#[derive(Debug)]
pub enum TimestampMethod {
    // Implement necessary variants
}

#[derive(Debug)]
pub enum MessageAndData {
    // Implement necessary fields and methods
}

pub type SaveConfig<T> = Box<dyn Fn(&mut T) -> Result<(), String>>;

pub struct PathLegendProps {
    pub paths: Vec<StateTransitionPath>;
    pub height_per_topic: f64;
    pub set_focused_path: fn(Vec<String> | Option<Vec<String>>) -> ();
    pub save_config: SaveConfig<StateTransitionConfig>;
}

#[derive(Debug)]
pub enum SeriesActionId {
    ADD = "add-series",
    DELETE = "delete-series",
}

pub type MessageDatasetArgs<'a> = {
    path: StateTransitionPath;
    start_time: Time;
    y: f64;
    path_index: usize;
    blocks: &'a [Option<Vec<MessageAndData>>];
    show_points: bool;
};

#[derive(Debug)]
pub enum ValidQueriedDataValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Bigint(bigint),
}

#[derive(Default, Debug)]
pub struct ImmutableDataset(Vec<Option<Vec<MessageAndData>>>>;
```
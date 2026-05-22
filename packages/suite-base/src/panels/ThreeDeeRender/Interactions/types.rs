```rust
use std::rc::Rc;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/
//
// This file incorporates work covered by the following copyright and
// permission notice:
//
//   Copyright 2019-2021 Cruise LLC

use std::rc::Rc;

#[derive(Debug)]
pub struct RosObject {
    // Implementing the required fields for a RosObject
}

#[derive(Debug)]
pub struct Marker {
    // Implementing the required fields for a Marker
}

pub type InteractionData = {
    topic: Option<String>,
    highlighted: bool,
    original_message: Rc<RosObject>,
    instance_details: Option<Rc<RosObject>>,
};

pub trait Interactive<T> {
    fn interaction_data(&self) -> &InteractionData;
}

pub type SelectedObject = {
    object: Marker,
    instance_index: Option<usize>,
};

pub enum TooltipMode {
    Hidden,
    Following,
    Settled,
    Grace,
    HoverPinned,
    ClickPinned,
}

#[derive(Debug)]
pub struct HoverTooltipProperties {
    entities: Vec<HoverEntityInfo>,
    position: (f64, f64),
    canvas: Option<Rc<dyn FnMut(&mut [u8])>>,
}
```
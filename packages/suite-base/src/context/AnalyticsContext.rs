```rust
use std::rc::Rc;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

pub struct Analytics {
    // Implement your Analytics logic here
}

impl Default for Analytics {
    fn default() -> Self {
        Self {}
    }
}

pub struct NullAnalytics {}

pub const ANALYTICS_CONTEXT: Rc<dyn std::any::Any> = Rc::new(NullAnalytics {});

fn use_analytics() -> &'static dyn IAnalytics {
    static mut INSTANCE: Option<Analytics> = None;

    match &mut INSTANCE {
        Some(analytics) => analytics,
        None => {
            let instance = Analytics::default();
            INSTANCE = Some(instance);
            &instance as &dyn IAnalytics
        }
    }
}

// Assuming IAnalytics is defined somewhere in your project
pub trait IAnalytics {}

// Usage example:
fn main() {
    let analytics = use_analytics();
    // Use the analytics object here
}
```
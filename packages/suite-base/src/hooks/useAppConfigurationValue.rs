```rust
use std::rc::Rc;
use std::sync::{Arc, Mutex};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum AppConfigurationValue {}

pub fn use_app_configuration_value<T: AppConfigurationValue>() -> (Option<Rc<Mutex<T>>>, impl FnOnce(T) + 'static) {
    let app_configuration = Arc::new(Mutex::new(None));

    let mut configuration_value = app_configuration.lock().unwrap();

    let configuration_value_ref = Rc::new(configuration_value.clone());

    use crate::event_listener::{addListener, removeListener};

    add_listener(app_configuration.as_ref(), key, move |val| {
        *configuration_value = Some(val);
    });

    let wrapped_setter = move |value: Option<Rc<Mutex<T>>>| {
        let mut value_mut = configuration_value.lock().unwrap();
        *value_mut = value;
        app_configuration.clone().lock().unwrap().get(key).cloned();
    };

    (configuration_value_ref, wrapped_setter)
}
```
```rust
use std::fmt::Display;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use yew::prelude::*;

struct ErrorBoundary {
    current_error: Option<Error>,
}

#[derive(Debug)]
struct AppError(Box<dyn Display>);

impl AppError {
    fn new(error: Box<dyn Display>) -> Self {
        AppError(error)
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Something went wrong")
    }
}

impl Component for ErrorBoundary {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties) -> Self {
        Self { current_error: None }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self) -> Html<Self> {
        if let Some(current_error) = &self.current_error {
            html! {
                <ErrorDisplay error={current_error} content="Something went wrong. Click here to dismiss." />
            }
        } else {
            html! { { self.props.children.clone() } }
        }
    }
}
```
```rust
use std::error::Error;

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

use crate::{AppError, ErrorDetails};
use std::fmt;

#[derive(Debug)]
pub struct AppError {
    message: String,
    details: Option<ErrorDetails>,
}

impl AppError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            details: None,
        }
    }

    pub fn from_error(err: Error, details: Option<ErrorDetails>) -> Self {
        Self {
            message: err.to_string(),
            details,
        }
    }

    pub fn with_extra_info(&self, extra_info: ErrorDetails) -> Self {
        self.details = Some(extra_info);
        self.clone()
    }

    pub fn as_str(&self) -> &str {
        &self.message
    }

    pub fn get_details(&self) -> Option<&ErrorDetails> {
        self.details.as_ref()
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.details {
            Some(details) => write!(f, "{}: {}", details.message(), self.as_str()),
            None => write!(f, "{}", self.as_str()),
        }
    }

    fn debug(&self, f: &mut fmt::DebugStruct<'_>) {
        f.debug_struct("AppError")
            .field("message", &self.message)
            .field("details", &self.details)
            .finish();
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        self.as_str()
    }

    fn source(&self) -> Option<Box<dyn Error>> {
        match &self.details {
            Some(details) => Some(Box::new(AppError {
                message: details.message().to_string(),
                details: None,
            })),
            None => None,
        }
    }
}
```
```rust
use std::fmt::{Display, Debug};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/
//
// This file incorporates work covered by the following copyright and
// permission notice:
//
//   Copyright 2020-2021 Cruise LLC
//
//   This source code is licensed under the Apache License, Version 2.0,
//   found at http://www.apache.org/licenses/LICENSE-2.0
//   You may not use this file except in compliance with the License.

use std::fmt::{Display, Debug};

type ExpectedResultProps = {
  top: Option<f32>;
  left: Option<f32>;
};

pub struct ExpectedResult<'a> {
  pub children: &'a dyn Display,
  pub top: f32,
  pub left: f32,
}

impl<'a> Display for ExpectedResult<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "ExpectedResult {{ children: {}, top: {}, left: {} }}", self.children, self.top, self.left)
  }
}

impl<'a> Debug for ExpectedResult<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "ExpectedResult {{ children: {}, top: {}, left: {} }}", self.children, self.top, self.left)
  }
}

pub struct ExpectedResultBuilder<'a> {
  pub children: &'a dyn Display,
  pub top: Option<f32>,
  pub left: Option<f32>,
}

impl<'a> ExpectedResultBuilder<'a> {
  pub fn new(children: &'a dyn Display) -> Self {
    Self { children, top: None, left: None }
  }

  pub fn set_top(mut self, top: f32) -> Self {
    self.top = Some(top);
    self
  }

  pub fn set_left(mut self, left: f32) -> Self {
    self.left = Some(left);
    self
  }

  pub fn build(self) -> ExpectedResult<'a> {
    ExpectedResult {
      children: self.children,
      top: self.top.unwrap_or(25.0),
      left: self.left.unwrap_or(0.0),
    }
  }
}
```
```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use styled_components::{StyledComponent, Theme};

#[derive(Debug)]
pub struct MuiCardContent<Theme = Theme> {
  style_overrides: StyledComponent<Theme, "root", ()>,
}

impl<Mu> MuiCardContent<Mu>
where
  Mu: StyledComponent<Theme, "root", ()>,
{
  pub fn new(theme: &Theme) -> Self {
    MuiCardContent {
      style_overrides: styled_components::create_with_theme(
        theme,
        |theme| {
          let mut style = Style::default();
          style.add_class("last-child").set_property(&"padding-bottom", &format!("{}px", theme.spacing(2)));
          style
        },
      ),
    }
  }
}
```
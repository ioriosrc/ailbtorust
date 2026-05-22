```rust
use std::props::PartialEq;

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
//
//   This source code is licensed under the Apache License, Version 2.0,
//   found at http://www.apache.org/licenses/LICENSE-2.0
//   You may not use this file except in compliance with the License.

use crate::suite_base::components::PanelContextType;
use crate::suite_base::types::{PanelConfig, PanelId};
use crate::suite_base::utils::uuid_v4;

pub const DEFAULT_MOCK_PANEL_CONTEXT: PanelContextType<PanelConfig> = {
  type: "foo",
  id: uuid_v4(),
  title: "Foo Panel",
  config: {},
  save_config: || (),
  update_panel_configs: || (),
  open_sibling_panel: || (),
  replace_panel: || (),
  enter_fullscreen: || (),
  exit_fullscreen: || (),
  set_has_fullscreen_descendant: || (),
  is_fullscreen: false,
  connect_toolbar_drag_handle: || (),
  setMessagePathDropConfig: || (),
};

pub struct MockPanelContextProvider {
    children: Box<dyn std::any::Any>,
}

impl PartialEq for MockPanelContextProvider {
    fn eq(&self, other: &Self) -> bool {
        self.children.eq(&other.children)
    }
}

impl MockPanelContextProvider {
    pub fn new(children: impl Into<Box<dyn std::any::Any>>) -> Self {
        Self { children: Box::new(children.into()) }
    }

    #[inline]
    pub fn render(self) -> String {
        format!(
            "<div>
                <PanelContext.Provider value={self}>
                    {children}
                </PanelContext.Provider>
            </div>",
            self = serde_json::to_string(&self).unwrap(),
            children = self.children.as_ref().downcast::<String>().unwrap_or_default()
        )
    }
}

impl std::fmt::Display for MockPanelContextProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}
```
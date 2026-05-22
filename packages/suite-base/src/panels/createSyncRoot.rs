```rust
use std::rc::Rc;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use std::thread;
use std::sync::{Arc, Mutex};

/// Creates a root for rendering React components.
///
/// This function is designed to centralize the creation of React roots
/// for rendering components within a given HTML element.
///
/// @param component The JSX element to be rendered within the root.
/// @param panelElement The HTML element to serve as the container for the root.
/// @returns A function to unmount the root when needed.
pub fn create_sync_root(
  component: Rc<ReactNode>,
  panel_element: Arc<HtmlDivElement>,
) -> Box<dyn Fn()> {
  let mut root = Root::new(panel_element);
  
  thread::spawn(move || {
    // Use queue_microtask to ensure that the unmount occurs after the render cycle
    queue_microtask(|| {
      root.unmount();
      panel_element.remove();
    });
  });

  Box::new(|| ())
}
```
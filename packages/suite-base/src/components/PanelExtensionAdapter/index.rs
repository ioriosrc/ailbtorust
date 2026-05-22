```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

mod PanelExtensionAdapter;
pub use self::PanelExtensionAdapter::*;
mod use_subscribe_message_range;
pub use self::use_subscribe_message_range::*;

pub type Asset = Asset;
pub type BuiltinPanelExtensionContext = BuiltinPanelExtensionContext;
pub type CreateMessageRangeIteratorParams = CreateMessageRangeIteratorParams;
pub type DraggedMessagePath = DraggedMessagePath;
pub type MessagePathDropConfig = MessagePathDropConfig;
pub type MessagePathDropStatus = MessagePathDropStatus;
```
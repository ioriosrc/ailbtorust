```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

pub const RAW_MESSAGES_VIRTUAL_DEFAULT_CONFIG: RawMessagesVirtualPanelConfig = {
    diff_enabled: false,
    diff_method: CustomMethod,
    diff_topic_path: "",
    show_full_message_for_diff: false,
    topic_path: "",
    fontSize: None,
};

pub const EXPANDED_ICON = "▶";

pub const COLLAPSED_ICON = "▼";

pub const ROW_HEIGHT = 24;
pub const TREE_NODE_INDENTATION = 16;
pub const SCROLLL_OVERSCAN = 5;
pub const DEFAULT_FONT_SIZE = 12;
```
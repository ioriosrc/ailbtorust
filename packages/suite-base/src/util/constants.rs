```rust
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

pub const KEY_MAP: HashMap<&str, &str> = [("urls", "url")];

pub const DEFAULT_STUDIO_SCRIPT_PREFIX: &str = "/studio_script/";

pub const JSON_TREE_THEME_COLORS: HashMap<_, HashMap<&str, &str>> = {
    "dark": [("string", "#ffa657"), ("number", "#7ee787"), ("text", "#79c0ff"), ("null", "#ff7b72"), ("label", "#79c0ff")],
    "light": [("string", "#953800"), ("number", "#116329"), ("text", "#0550ae"), ("null", "#cf222e"), ("label", "#0550ae")],
};

pub const TAB_PANEL_TYPE: &str = "Tab";

pub const GLOBAL_REQUEST_QUEUE_MAX_CONCURRENT: usize = 10;
```
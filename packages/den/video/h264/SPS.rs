```rust
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

#[derive(Debug)]
pub struct SPS {
    pub profile_idc: u8,
    pub profile_compatibility: u8,
    pub level_idc: u8,
}

impl SPS {
    fn new(profile_idc: u8, profile_compatibility: u8, level_idc: u8) -> Self {
        SPS {
            profile_idc,
            profile_compatibility,
            level_idc,
        }
    }

    pub fn to_mime(&self) -> String {
        let mut mime = format!("avc1.");
        mime.push_str(format!("{:02X}", self.profile_idc).as_bytes());
        mime.push_str(format!("{:02X}", self.profile_compatibility).as_bytes());
        mime.push_str(format!("{:02X}", self.level_idc).as_bytes());
        mime
    }
}

fn byte_to_hex(val: u8) -> String {
    format!("{:02X}", val)
}
```
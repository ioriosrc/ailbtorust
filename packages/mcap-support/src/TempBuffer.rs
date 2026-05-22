```rust
use std::sync::{Arc, Mutex};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use super::McapTypes;
pub struct TempBuffer {
    buffer: Arc<Mutex<Vec<u8>>>,
}

impl McapTypes::IReadable for TempBuffer {
    fn position(&self) -> u64 {
        self.buffer.lock().unwrap().len() as u64
    }

    async fn write(&mut self, data: &[u8]) -> Result<(), McapTypes::WriteError> {
        let mut buffer = self.buffer.lock().unwrap();
        if buffer.len() + data.len() > buffer.capacity() {
            let new_buffer = vec![0; buffer.capacity()];
            buffer.clone_into(&new_buffer);
            *self.buffer = Arc::new(Mutex::new(new_buffer));
            buffer = self.buffer.lock().unwrap();
        }
        buffer.extend_from_slice(data);
        Ok(())
    }

    async fn size(&self) -> Result<u64, McapTypes::WriteError> {
        Ok(self.buffer.lock().unwrap().len() as u64)
    }
}

impl McapTypes::IWritable for TempBuffer {
    async fn read(&mut self, offset: u64, size: u64) -> Result<Vec<u8>, McapTypes::ReadError> {
        let buffer = self.buffer.lock().unwrap();
        if offset < 0 || offset + size > buffer.len() as u64 {
            Err(McapTypes::ReadError::OutOfBounds)
        }
        Ok(buffer[offset as usize..(offset + size) as usize].to_vec())
    }

    async fn get(&mut self) -> Result<Vec<u8>, McapTypes::WriteError> {
        let buffer = self.buffer.lock().unwrap();
        Ok(buffer.clone_to_vec())
    }
}
```
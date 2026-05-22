```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use std::convert::{TryFrom, TryInto};

struct Bitstream {
    buffer: Vec<u8>,
    position: usize,
}

impl Bitstream {
    fn new(buffer: Vec<u8>) -> Self {
        Self { buffer, position: 0 }
    }

    fn u_1(&mut self) -> u8 {
        let byte = self.buffer[self.position];
        let bit = (byte & 0b01) >> 0;
        self.position += 1;
        bit
    }

    fn u_2(&mut self) -> u8 {
        let byte = self.buffer[self.position];
        let bits = (byte & 0b03) as u8;
        self.position += 2;
        bits
    }

    fn u_3(&mut self) -> u8 {
        let byte = self.buffer[self.position];
        let bits = (byte & 0b07) as u8;
        self.position += 3;
        bits
    }

    fn u_8(&mut self) -> u16 {
        let mut value = 0;
        for _ in 0..8 {
            let bit = self.u_1() as u16;
            value <<= 1;
            value |= bit;
        }
        value
    }

    fn reset(&mut self) {
        self.position = 0;
    }

    fn ue_v(&mut self) -> i32 {
        let mut value = 0;
        let mut shift = 0;
        while self.u_1() == 1 {
            value <<= 1;
            value |= 1;
            shift += 1;
        }
        let extra_bits = self.u_1();
        value <<= shift;
        value -= (extra_bits as i32) * 1 << shift;
        value
    }

    fn se_v(&mut self) -> i32 {
        let mut value = 0;
        let mut shift = 0;
        while self.u_1() == 1 {
            value <<= 1;
            value |= 1;
            shift += 1;
        }
        if self.u_1() == 1 {
            value *= -1;
        }
        let extra_bits = self.u_1();
        value <<= shift;
        value -= (extra_bits as i32) * 1 << shift;
        value
    }

    fn u(&mut self, bits: usize) -> i32 {
        let mut value = 0;
        for _ in 0..bits {
            let bit = self.u_1() as i32;
            value <<= 1;
            value |= bit;
        }
        value
    }

    fn u(16) -> i32 {
        self.u(16).try_into().unwrap()
    }

    fn u(32) -> i32 {
        self.u(32).try_into().unwrap()
    }
}
```
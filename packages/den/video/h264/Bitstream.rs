```rust
use std::vec::Vec;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

/**
 * Bitstream reader for H264 data, handling emulation prevention bytes and
 * decoding exponential Golomb format integers.
 */
pub struct Bitstream {
    buffer: Vec<u8>,
    ptr: usize,
    byte_ptr: usize,
    last_two_bytes: u16,
    max: usize,
}

impl Bitstream {
    /**
     * Construct a bitstream
     * @param stream Buffer containing the stream
     */
    pub fn new(stream: Vec<u8>) -> Self {
        let max = stream.len() << 3;
        Self {
            buffer,
            ptr: 0,
            byte_ptr: 0,
            last_two_bytes: 0,
            max,
        }
    }

    pub fn reset(&mut self) {
        self.ptr = 0;
        self.byte_ptr = 0;
        self.last_two_bytes = 0;
    }

    /**
     * get one bit
     * @returns {number}
     */
    pub fn u1(&mut self) -> bool {
        if self.ptr + 1 > self.max {
            panic!("Bitstream error: bitstream exhausted");
        }
        let byte_offset = self.ptr >> 3;
        let bit_offset = 0x07 - (self.ptr & 0x07);
        let saved_byte_ptr = self.byte_ptr;

        self.byte_ptr = byte_offset;

        if bit_offset != 0 {
            let byte1 = self.buffer[self.byte_ptr]!;
            let byte2 = self.buffer[self.byte_ptr + 1]!;
            let val =
                ((byte1 & ((1 << (8 - bit_offset)) - 1)) << bit_offset) | (byte2 >> (8 - bit_offset));
            self.byte_ptr += 2;
        } else {
            let val = self.buffer[self.byte_ptr]!;
            self.byte_ptr += 1;
        }

        self.ptr += 1;
        val != 0
    }

    /**
     * get two bits
     * @returns {number}
     */
    pub fn u2(&mut self) -> u8 {
        let bit0 = self.u1();
        let bit1 = self.u1();
        (bit0 << 1) | bit1
    }

    /**
     * get three bits
     * @returns {number}
     */
    pub fn u3(&mut self) -> u8 {
        let bit0 = self.u1();
        let bit1 = self.u1();
        let bit2 = self.u1();
        (bit0 << 2) | (bit1 << 1) | bit2
    }

    /**
     * get one byte (as an unsigned number)
     * @returns {number}
     */
    pub fn u8(&mut self) -> u32 {
        if self.ptr + 8 > self.max {
            panic!("Bitstream error: bitstream exhausted");
        }
        let byte_offset = self.ptr >> 3;
        let bit_offset = self.ptr & 0x07;

        if bit_offset != 0 {
            let byte1 = self.buffer[self.byte_ptr]!;
            let byte2 = self.buffer[self.byte_ptr + 1]!;
            let val =
                ((byte1 & ((1 << (8 - bit_offset)) - 1)) << bit_offset) | (byte2 >> (8 - bit_offset));
            self.byte_ptr += 2;
        } else {
            let val = self.buffer[self.byte_ptr]!;
            self.byte_ptr += 1;
        }

        self.ptr += 8;
        u32::from_le_bytes(val.to_be_bytes())
    }

    /**
     * get an unsigned H.264-style variable-bit number
     * in exponential Golomb format
     * @returns {number}
     */
    pub fn ue_v(&mut self) -> u32 {
        let zeros = 0;
        while !self.u1() {
            zeros += 1;
        }
        let val = 1 << zeros;
        for _ in 0..zeros - 1 {
            val |= self.u1();
        }
        val - 1
    }

    /**
     * get a signed h.264-style variable bit number
     * in exponential Golomb format
     * @returns {number} (without negative zeros)
     */
    pub fn se_v(&mut self) -> i32 {
        let codeword = self.ue_v();
        if codeword & 1 == 1 {
            -(codeword >> 1)
        } else {
            codeword >> 1
        }
    }

    /**
     * get n bits
     * @param n
     * @returns {number}
     */
    pub fn u(&mut self, n: usize) -> u32 {
        if n == 8 {
            return self.u8();
        }
        if self.ptr + n > self.max {
            panic!("NALUStream error: bitstream exhausted");
        }
        let val = 0;
        for _ in 0..n {
            val <<= 1;
            val |= self.u1();
        }
        val
    }

    #[allow(clippy::unreadable_literal)]
    fn u8_lut(&self, byte: &u8) -> u32 {
        // LUT to convert a byte to an unsigned 8-bit integer
        match byte {
            0x00 => 0,
            0x01..=0x07 => (byte - 0x01) as u32 * 8 + 8,
            0x08..=0x0F => (byte - 0x08) as u32 * 4 + 4,
            0x10..=0x17 => (byte - 0x10) as u32 * 2 + 2,
            0x18..=0xFF => byte - 0x18 + 1,
            _ => panic!("Invalid byte value"),
        }
    }

    #[allow(clippy::unreadable_literal)]
    fn read_byte(&mut self) -> u32 {
        if self.byte_ptr >= self.buffer.len() {
            panic!("Attempted to read past end of buffer");
        }

        // If the current byte is 0x03 and the last two bytes were zeros, skip over it
        if self.last_two_bytes == 0 && self.buffer[self.byte_ptr] == 0x03 {
            self.byte_ptr += 2;
            return self.u8_lut(&self.buffer[self.byte_ptr - 2]) << 16;
        }

        // Update the last two bytes and return the current byte
        let val = u32::from_le_bytes([self.buffer[self.byte_ptr]!, self.buffer[self.byte_ptr + 1]!]);
        self.last_two_bytes = ((self.last_two_bytes << 8) | val & 0xFF) as u16;
        self.byte_ptr += 2;

        val
    }
}
```
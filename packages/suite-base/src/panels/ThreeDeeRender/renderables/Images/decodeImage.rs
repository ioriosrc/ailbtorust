```rust
use std::sync::Arc;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use web_sys::ImageData;

use crate::{
    compressed_image::{decode_bgr8, decode_raw_image},
    color_mode::get_color_converter,
};

#[derive(Clone)]
pub struct Bitmap {
    data: ImageData,
}

impl Bitmap {
    pub fn new(data: ImageData) -> Self {
        Bitmap { data }
    }

    pub async fn create_from_raw_image(
        raw_image: &RawImage,
        options: Option<ColorModeSettings>,
    ) -> Result<Self, String> {
        let encoder = match raw_image.encoding.as_str() {
            "yuv422" | "uyvy" => decoder::UYVYEncoder {},
            "yuv422_yuy2" | "yuv" => decoder::YUVEncoder {},
            "rgb8" => decoder::RGB8Encoder {},
            "rgba8" => decoder::RGBA8Encoder {},
            "bgra8" => decoder::BGRA8Encoder {},
            "bgr8" => decoder::BGR8Encoder {},
            "32fc1" => decoder::Float1CEncoder {},
            "bayer_rggb8" => decoder::BayerRGGB8Decoder {},
            "bayer_bggr8" => decoder::BayerBGGR8Decoder {},
            "bayer_gbrg8" => decoder::BayerGBRG8Decoder {},
            "bayer_grbg8" => decoder::BayerGRBG8Decoder {},
            "mono8" | "8uc1" => decoder::MonoEncoder {
                options: options.unwrap_or_default(),
            },
            "mono16" | "16uc1" => decoder::Mono16Encoder {
                options: options.unwrap_or_default(),
            },
            _ => return Err("Unsupported encoding".to_string()),
        };
        let data = encoder.encode(raw_image.data.as_bytes());
        Ok(Bitmap::new(ImageData::from_array_unchecked(&data, raw_image.width as u32, raw_image.height as u32)?))
    }
}
```
```rust
use std::vec::Vec;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use foxglove::schemas::{CameraCalibration, CompressedImage};
use foxglove::MessageEvent;

type MakeImageArgs = {
  width: u32;
  height: u32;
  frame_id: String;
  image_topic: String;
  calibration_topic: String;
};

pub fn make_raw_image_and_calibration(args: MakeImageArgs) -> (MessageEvent<Partial<CameraCalibration>>, MessageEvent<Partial<RawImage>>) {
  let { width, height, frame_id, image_topic, calibration_topic, fx = 500, fy = 500 } = args;
  let cx = width / 2;
  let cy = height / 2;
  let calibration_message: MessageEvent<Partial<CameraCalibration>> = {
    topic: calibration_topic,
    receive_time: foxglove::Time::new(0, 0),
    message: {
      timestamp: foxglove::Time::new(0, 0),
      frame_id,
      height,
      width,
      distortion_model: "rational_polynomial",
      D: Vec::new(),
      K: [fx, 0, cx, 0, fy, cy, 0, 0, 1],
      R: Vec::new(),
      P: [fx, 0, cx, 0, 0, fy, cy, 0, 0, 0, 1, 0],
    },
    schema_name: "foxglove.CameraCalibration",
    size_in_bytes: 0,
  };

  let imageData = vec![255; width * height * 3];

  for i in 0..width * height {
    if i % 10 == 0 || i % 2 == 0 {
      imageData[i * 3] = 150;
      imageData[i * 3 + 1] = 150;
      imageData[i * 3 + 2] = 255;
    } else {
      imageData[i * 3] = ((i / 10) % 10) as u8 + ((i / 2) % 2) as u8;
      imageData[i * 3 + 1] = ((i / 10) % 10) as u8 + ((i / 2) % 2) as u8;
      imageData[i * 3 + 2] = ((i / 10) % 10) as u8 + ((i / 2) % 2) as u8;
    }
  }

  let raw_camera_message: MessageEvent<Partial<RawImage>> = {
    topic: image_topic,
    receive_time: foxglove::Time::new(10, 0),
    message: {
      timestamp: foxglove::Time::new(0, 0),
      frame_id,
      width,
      height,
      step: 3 * width,
      encoding: "rgb8",
      data: imageData.as_slice(),
    },
    schema_name: "foxglove.RawImage",
    size_in_bytes: 0,
  };

  (calibration_message, raw_camera_message)
}

pub async fn make_compressed_image_and_calibration(args: MakeImageArgs) -> (MessageEvent<Partial<CameraCalibration>>, MessageEvent<Partial<CompressedImage>>) {
  let (calibration_message, raw_camera_message) = make_raw_image_and_calibration(args);

  let canvas = web_sys::window().unwrap()
    .create_element("canvas")?
    .unwrap();

  canvas.set_attribute("width", &args.width.to_string())?;
  canvas.set_attribute("height", &args.height.to_string())?;

  let ctx = canvas.get_context("2d")?.unwrap();

  if !ctx.is_ok() {
    panic!("Failed to get canvas context");
  }

  let imageData = web_sys::BlobBuilder::new()
    .append_slice(&raw_camera_message.message.data)?
    .unwrap();

  let image_blob = imageData
    .array_buffer()
    .await?;

  let camera_message: MessageEvent<Partial<CompressedImage>> = {
    topic: raw_camera_message.topic,
    receive_time: foxglove::Time::new(10, 0),
    message: {
      timestamp: raw_camera_message.message.timestamp,
      frame_id,
      data: image_blob.as_slice(),
      format: "png",
    },
    schema_name: "foxglove.CompressedImage",
    size_in_bytes: 0,
  };

  (calibration_message, camera_message)
}
```
```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

const SUBSCRIPTION_WARNING_SUPPRESSION_MS: u32 = 2000;

const ZERO_TIME: [u32; 2] = [0, 0];
const GET_ALL_PARAMS_REQUEST_ID: &str = "get-all-params";
const GET_ALL_PARAMS_PERIOD_MS: u32 = 15000;
const ROS_ENCODINGS: &'static [&'static str] = ["ros1", "cdr"];
const SUPPORTED_PUBLICATION_ENCODINGS: &'static [&'static str] = ["json", ..ROS_ENCODINGS];
const FALLBACK_PUBLICATION_ENCODING: &str = "json";
const SUPPORTED_SERVICE_ENCODINGS: &'static [&'static str] = ["json", ..ROS_ENCODINGS];

// When the tab is inactive setTimeout's are throttled to at most once per second.
// Because the MessagePipeline listener uses timeouts to resolve its promises, it throttles our ability to
// emit a frame more than once per second. In the websocket player this was causing
// an accumulation of messages that were waiting to be emitted, this could keep growing
// indefinitely if the rate at which we emit a frame is low enough.
const CURRENT_FRAME_MAXIMUM_SIZE_BYTES: u32 = 400 * 1024 * 1024;
```
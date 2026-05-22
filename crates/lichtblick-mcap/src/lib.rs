// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

//! MCAP file reading and schema parsing for Lichtblick.
//!
//! Supports reading MCAP files with JSON, Protobuf, ROS1, ROS2, and Flatbuffer schemas.

pub mod reader;
pub mod schema;
pub mod source;

pub use reader::McapReader;
pub use source::McapIterableSource;

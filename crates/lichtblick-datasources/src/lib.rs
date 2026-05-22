// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

//! Data source factories for creating players from various inputs.

pub mod factory;
pub mod mcap_local;
pub mod foxglove_websocket;

pub use factory::{DataSourceFactory, DataSourceType, DataSourceFactoryArgs};
pub use mcap_local::McapLocalDataSourceFactory;
pub use foxglove_websocket::FoxgloveWebSocketDataSourceFactory;

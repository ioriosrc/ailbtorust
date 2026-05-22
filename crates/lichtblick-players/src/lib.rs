// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

//! Player implementations for Lichtblick.
//!
//! Players are responsible for managing data playback from various sources.

pub mod iterable_player;
pub mod traits;
pub mod websocket_player;

pub use iterable_player::IterablePlayer;
pub use traits::Player;
pub use websocket_player::FoxgloveWebSocketPlayer;

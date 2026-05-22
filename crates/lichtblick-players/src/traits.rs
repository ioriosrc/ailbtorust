// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use lichtblick_core::player::PlayerState;
use lichtblick_core::time::Time;
use lichtblick_core::types::{PublishPayload, ServiceCallPayload, ServiceCallResponse, SubscribePayload};
use lichtblick_core::error::LichtblickError;

/// Callback for player state changes.
pub type PlayerListener = Box<dyn Fn(PlayerState) + Send + Sync>;

/// Core Player trait - all player implementations must implement this.
pub trait Player: Send + Sync {
    /// Set the listener callback for state updates.
    fn set_listener(&mut self, listener: PlayerListener);

    /// Update topic subscriptions.
    fn set_subscriptions(&mut self, subscriptions: Vec<SubscribePayload>);

    /// Start playback.
    fn start_playback(&mut self);

    /// Pause playback.
    fn pause_playback(&mut self);

    /// Seek to a specific time.
    fn seek_playback(&mut self, time: Time);

    /// Set playback speed (1.0 = realtime).
    fn set_playback_speed(&mut self, speed: f64);

    /// Publish a message (if supported).
    fn publish(&mut self, _payload: PublishPayload) -> Result<(), LichtblickError> {
        Err(LichtblickError::Player("Publish not supported".into()))
    }

    /// Call a service (if supported).
    fn call_service(
        &mut self,
        _payload: ServiceCallPayload,
    ) -> Result<ServiceCallResponse, LichtblickError> {
        Err(LichtblickError::Player("Service calls not supported".into()))
    }

    /// Close the player and release resources.
    fn close(&mut self);

    /// Get the player name.
    fn name(&self) -> &str;
}

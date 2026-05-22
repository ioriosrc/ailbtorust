// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use leptos::prelude::*;
use lichtblick_core::player::PlayerState;
use lichtblick_core::types::{MessageEvent, Topic, RosDatatypes};

/// Reactive player state accessible to all panels/components.
#[derive(Clone, Copy)]
pub struct ReactivePlayerState {
    pub player_state: RwSignal<Option<PlayerState>>,
    pub topics: RwSignal<Vec<Topic>>,
    pub current_messages: RwSignal<Vec<MessageEvent>>,
}

impl ReactivePlayerState {
    pub fn new() -> Self {
        Self {
            player_state: RwSignal::new(None),
            topics: RwSignal::new(Vec::new()),
            current_messages: RwSignal::new(Vec::new()),
        }
    }

    /// Update from a player state emission.
    pub fn update_from_player(&self, state: PlayerState) {
        if let Some(active) = &state.active_data {
            self.topics.set(active.topics.clone());
            self.current_messages.set(active.messages.clone());
        }
        self.player_state.set(Some(state));
    }
}

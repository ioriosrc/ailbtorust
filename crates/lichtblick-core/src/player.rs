// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::time::Time;
use crate::types::*;

/// Player capability flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlayerCapability {
    PlaybackControl,
    SetSpeed,
    Publish,
    CallServices,
    GetParameters,
    SetParameters,
}

/// Player presence state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerPresence {
    NotPresent,
    Initializing,
    Reconnecting,
    Buffering,
    Present,
    Error,
}

/// Active data from the player (when playing/loaded).
#[derive(Debug, Clone)]
pub struct ActiveData {
    /// Messages for the current frame.
    pub messages: Vec<MessageEvent>,
    /// Current playback time.
    pub current_time: Time,
    /// Start of the data range.
    pub start_time: Time,
    /// End of the data range.
    pub end_time: Time,
    /// Whether playback is active.
    pub is_playing: bool,
    /// Current playback speed (1.0 = realtime).
    pub speed: f64,
    /// All available topics.
    pub topics: Vec<Topic>,
    /// All datatypes / schemas.
    pub datatypes: RosDatatypes,
    /// Per-topic statistics.
    pub topic_stats: HashMap<String, TopicStats>,
}

/// Full player state emitted to the pipeline.
#[derive(Debug, Clone)]
pub struct PlayerState {
    /// Current presence.
    pub presence: PlayerPresence,
    /// Loading progress.
    pub progress: Progress,
    /// Player capabilities.
    pub capabilities: Vec<PlayerCapability>,
    /// Semantic profile (ros1, ros2, foxglove-websocket, mcap, etc.).
    pub profile: Option<String>,
    /// Unique session ID.
    pub player_id: String,
    /// Active alerts.
    pub alerts: Vec<PlayerAlert>,
    /// Active data (None if not yet loaded).
    pub active_data: Option<ActiveData>,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            presence: PlayerPresence::NotPresent,
            progress: Progress::default(),
            capabilities: Vec::new(),
            profile: None,
            player_id: String::new(),
            alerts: Vec::new(),
            active_data: None,
        }
    }
}

/// Player state machine states (internal).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerInternalState {
    Preinit,
    Initialize,
    StartPlay,
    Idle,
    Play,
    SeekBackfill,
    ResetPlaybackIterator,
    Close,
}

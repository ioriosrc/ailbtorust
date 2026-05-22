// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use lichtblick_core::error::LichtblickError;
use lichtblick_core::player::*;
use lichtblick_core::time::Time;
use lichtblick_core::types::*;
use lichtblick_mcap::source::{McapIterableSource, SourceInitialization};

use crate::traits::{Player, PlayerListener};

/// Configuration for the IterablePlayer.
pub struct IterablePlayerConfig {
    /// Display name.
    pub name: String,
    /// How far ahead to buffer (in nanoseconds).
    pub read_ahead_duration_ns: u64,
}

impl Default for IterablePlayerConfig {
    fn default() -> Self {
        Self {
            name: "MCAP Player".to_string(),
            read_ahead_duration_ns: 120_000_000_000, // 120 seconds
        }
    }
}

/// Player for iterating over pre-recorded data (MCAP files, bag files, etc.).
///
/// State machine: preinit → initialize → start-play → idle ↔ play → close
pub struct IterablePlayer {
    config: IterablePlayerConfig,
    source: McapIterableSource,
    state: PlayerInternalState,
    listener: Option<PlayerListener>,
    player_id: String,

    // Playback state
    subscriptions: Vec<SubscribePayload>,
    is_playing: bool,
    speed: f64,
    current_time: Time,
    start_time: Time,
    end_time: Time,

    // Metadata
    topics: Vec<Topic>,
    datatypes: RosDatatypes,
    topic_stats: std::collections::HashMap<String, TopicStats>,
    profile: Option<String>,
}

impl IterablePlayer {
    pub fn new(source: McapIterableSource, config: IterablePlayerConfig) -> Self {
        Self {
            config,
            source,
            state: PlayerInternalState::Preinit,
            listener: None,
            player_id: uuid::Uuid::new_v4().to_string(),
            subscriptions: Vec::new(),
            is_playing: false,
            speed: 1.0,
            current_time: Time::ZERO,
            start_time: Time::ZERO,
            end_time: Time::ZERO,
            topics: Vec::new(),
            datatypes: RosDatatypes::new(),
            topic_stats: std::collections::HashMap::new(),
            profile: None,
        }
    }

    /// Initialize the player by reading source metadata.
    pub fn initialize(&mut self) -> Result<(), LichtblickError> {
        self.state = PlayerInternalState::Initialize;
        self.emit_state();

        let init = self.source.initialize()?;
        self.start_time = init.start_time;
        self.end_time = init.end_time;
        self.current_time = init.start_time;
        self.topics = init.topics;
        self.datatypes = init.datatypes;
        self.topic_stats = init.topic_stats;
        self.profile = Some(init.profile);

        self.state = PlayerInternalState::Idle;
        self.emit_state();
        Ok(())
    }

    /// Perform a single playback tick - read messages and advance time.
    pub fn tick(&mut self) -> Result<Vec<MessageEvent>, LichtblickError> {
        if !self.is_playing || self.state != PlayerInternalState::Play {
            return Ok(Vec::new());
        }

        // Calculate tick duration (~16ms at 60fps, scaled by speed)
        let tick_duration_ns = (16_000_000.0 * self.speed) as u64;
        let max_tick_ns = 300_000_000u64; // Cap at 300ms
        let duration = tick_duration_ns.min(max_tick_ns);

        let target_time = Time::from_nanos(
            (self.current_time.to_nanos() + duration).min(self.end_time.to_nanos()),
        );

        // Get subscribed topics
        let subscribed_topics: Vec<String> = self
            .subscriptions
            .iter()
            .map(|s| s.topic.clone())
            .collect();

        let results = self
            .source
            .message_iterator(&subscribed_topics, self.current_time, target_time)?;

        let messages: Vec<MessageEvent> = results
            .into_iter()
            .filter_map(|r| match r {
                lichtblick_mcap::source::IteratorResult::MessageEvent(msg) => Some(msg),
                _ => None,
            })
            .collect();

        self.current_time = target_time;

        // Check if we've reached the end
        if self.current_time >= self.end_time {
            self.is_playing = false;
            self.state = PlayerInternalState::Idle;
        }

        self.emit_state_with_messages(&messages);
        Ok(messages)
    }

    /// Emit current player state to the listener.
    fn emit_state(&self) {
        self.emit_state_with_messages(&[]);
    }

    fn emit_state_with_messages(&self, messages: &[MessageEvent]) {
        if let Some(listener) = &self.listener {
            let presence = match self.state {
                PlayerInternalState::Preinit => PlayerPresence::NotPresent,
                PlayerInternalState::Initialize => PlayerPresence::Initializing,
                _ => PlayerPresence::Present,
            };

            let active_data = if self.state != PlayerInternalState::Preinit
                && self.state != PlayerInternalState::Initialize
            {
                Some(ActiveData {
                    messages: messages.to_vec(),
                    current_time: self.current_time,
                    start_time: self.start_time,
                    end_time: self.end_time,
                    is_playing: self.is_playing,
                    speed: self.speed,
                    topics: self.topics.clone(),
                    datatypes: self.datatypes.clone(),
                    topic_stats: self.topic_stats.clone(),
                })
            } else {
                None
            };

            let state = PlayerState {
                presence,
                progress: Progress::default(),
                capabilities: vec![
                    PlayerCapability::PlaybackControl,
                    PlayerCapability::SetSpeed,
                ],
                profile: self.profile.clone(),
                player_id: self.player_id.clone(),
                alerts: Vec::new(),
                active_data,
            };

            listener(state);
        }
    }
}

impl Player for IterablePlayer {
    fn set_listener(&mut self, listener: PlayerListener) {
        self.listener = Some(listener);
        self.emit_state();
    }

    fn set_subscriptions(&mut self, subscriptions: Vec<SubscribePayload>) {
        self.subscriptions = subscriptions;
    }

    fn start_playback(&mut self) {
        if self.state == PlayerInternalState::Idle {
            self.is_playing = true;
            self.state = PlayerInternalState::Play;
            self.emit_state();
        }
    }

    fn pause_playback(&mut self) {
        if self.is_playing {
            self.is_playing = false;
            self.state = PlayerInternalState::Idle;
            self.emit_state();
        }
    }

    fn seek_playback(&mut self, time: Time) {
        let clamped = Time::from_nanos(
            time.to_nanos()
                .max(self.start_time.to_nanos())
                .min(self.end_time.to_nanos()),
        );
        self.current_time = clamped;
        self.state = PlayerInternalState::SeekBackfill;

        // Get backfill messages
        let subscribed_topics: Vec<String> = self
            .subscriptions
            .iter()
            .map(|s| s.topic.clone())
            .collect();

        if let Ok(backfill) = self.source.get_backfill_messages(&subscribed_topics, clamped) {
            self.emit_state_with_messages(&backfill);
        }

        self.state = if self.is_playing {
            PlayerInternalState::Play
        } else {
            PlayerInternalState::Idle
        };
    }

    fn set_playback_speed(&mut self, speed: f64) {
        self.speed = speed.max(0.01).min(100.0);
        self.emit_state();
    }

    fn close(&mut self) {
        self.state = PlayerInternalState::Close;
        self.is_playing = false;
    }

    fn name(&self) -> &str {
        &self.config.name
    }
}

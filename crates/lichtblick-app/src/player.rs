// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

//! MCAP file player with efficient zero-copy message storage.
//! Key optimizations:
//! - Messages store data as Rc<Vec<u8>> (shared, no clone on read)
//! - Skip bad messages instead of aborting
//! - Only lightweight index per message (16 bytes + topic_idx)
//! - Binary search for time-based lookups

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::state::app_state::AppState;

/// A single stored message - data is shared (Rc) to avoid cloning large buffers.
#[derive(Clone, Debug)]
pub struct StoredMessage {
    pub log_time_ns: u64,
    pub topic: String,
    pub data: Rc<Vec<u8>>,
    pub schema_name: String,
    pub encoding: String,
}

/// Per-topic sorted message indices for O(log n) time lookups.
#[derive(Clone, Debug)]
pub struct TopicIndex {
    pub message_indices: Vec<usize>,
}

/// Loaded MCAP data with efficient storage.
#[derive(Clone, Debug)]
pub struct McapData {
    pub messages: Vec<StoredMessage>,
    pub start_time_ns: u64,
    pub end_time_ns: u64,
    pub topics: Vec<TopicInfo>,
    pub topic_indices: HashMap<String, TopicIndex>,
    pub total_bytes: usize,
}

#[derive(Clone, Debug)]
pub struct TopicInfo {
    pub name: String,
    pub schema_name: String,
    pub encoding: String,
    pub message_count: usize,
}

/// Parse MCAP file bytes into McapData.
/// Skips bad messages instead of aborting. Uses Rc for data to avoid copies.
pub fn parse_mcap_file(data: &[u8]) -> Result<McapData, String> {
    let total_bytes = data.len();
    log::info!("Parsing MCAP file: {:.1} MB", total_bytes as f64 / 1_048_576.0);

    let mut messages: Vec<StoredMessage> = Vec::new();
    let mut topic_counts: HashMap<String, (String, String, usize)> = HashMap::new();
    let mut skipped = 0usize;

    let stream =
        mcap::MessageStream::new(data).map_err(|e| format!("Failed to open MCAP: {}", e))?;

    for msg_result in stream {
        // Skip bad messages instead of aborting
        let msg = match msg_result {
            Ok(m) => m,
            Err(e) => {
                skipped += 1;
                if skipped <= 5 {
                    log::warn!("Skipping bad message: {}", e);
                }
                continue;
            }
        };

        let topic = msg.channel.topic.clone();
        let schema_name = msg
            .channel
            .schema
            .as_ref()
            .map(|s| s.name.clone())
            .unwrap_or_default();
        let encoding = msg.channel.message_encoding.clone();

        let entry = topic_counts
            .entry(topic.clone())
            .or_insert_with(|| (schema_name.clone(), encoding.clone(), 0));
        entry.2 += 1;

        // Use Rc to share data without cloning on read
        messages.push(StoredMessage {
            log_time_ns: msg.log_time,
            topic,
            data: Rc::new(msg.data.to_vec()),
            schema_name,
            encoding,
        });
    }

    if messages.is_empty() {
        return Err("MCAP file contains no messages".to_string());
    }

    if skipped > 0 {
        log::warn!("Skipped {} bad messages total", skipped);
    }

    messages.sort_by_key(|m| m.log_time_ns);

    // Build per-topic indices
    let mut topic_indices: HashMap<String, TopicIndex> = HashMap::new();
    for (idx, msg) in messages.iter().enumerate() {
        topic_indices
            .entry(msg.topic.clone())
            .or_insert_with(|| TopicIndex {
                message_indices: Vec::new(),
            })
            .message_indices
            .push(idx);
    }

    let start_time_ns = messages.first().unwrap().log_time_ns;
    let end_time_ns = messages.last().unwrap().log_time_ns;

    let mut topics: Vec<TopicInfo> = topic_counts
        .into_iter()
        .map(|(name, (schema_name, encoding, message_count))| TopicInfo {
            name,
            schema_name,
            encoding,
            message_count,
        })
        .collect();
    topics.sort_by(|a, b| a.name.cmp(&b.name));

    log::info!(
        "MCAP loaded: {} messages, {} topics, {:.3}s duration",
        messages.len(),
        topics.len(),
        (end_time_ns - start_time_ns) as f64 / 1_000_000_000.0
    );

    Ok(McapData {
        messages,
        start_time_ns,
        end_time_ns,
        topics,
        topic_indices,
        total_bytes,
    })
}

impl McapData {
    /// Get the latest message for a topic at or before the given time.
    pub fn get_latest_message(&self, topic: &str, time_ns: u64) -> Option<&StoredMessage> {
        let index = self.topic_indices.get(topic)?;
        let indices = &index.message_indices;
        let pos = indices.partition_point(|&idx| self.messages[idx].log_time_ns <= time_ns);
        if pos == 0 {
            return None;
        }
        Some(&self.messages[indices[pos - 1]])
    }

    /// Get messages for a topic in a time range (for state transitions etc.)
    pub fn get_messages_in_range(&self, topic: &str, start_ns: u64, end_ns: u64) -> Vec<&StoredMessage> {
        let Some(index) = self.topic_indices.get(topic) else {
            return Vec::new();
        };
        let indices = &index.message_indices;
        let start_pos = indices.partition_point(|&idx| self.messages[idx].log_time_ns < start_ns);
        let end_pos = indices.partition_point(|&idx| self.messages[idx].log_time_ns <= end_ns);
        indices[start_pos..end_pos]
            .iter()
            .map(|&idx| &self.messages[idx])
            .collect()
    }
}

/// Player internal state.
struct PlaybackState {
    mcap_data: McapData,
    current_time_ns: u64,
    is_playing: bool,
    speed: f64,
    last_wall_time_ms: f64,
    current_msg_index: usize,
    animation_frame_id: Option<i32>,
    frame_counter: u64,
}

/// Shared player handle.
pub struct McapPlayer {
    inner: Rc<RefCell<PlaybackState>>,
    app_state: AppState,
    _closure: Rc<RefCell<Option<Closure<dyn FnMut()>>>>,
}

impl McapPlayer {
    pub fn new(mcap_data: McapData, app_state: AppState) -> Self {
        let start_ns = mcap_data.start_time_ns;

        let state = PlaybackState {
            mcap_data,
            current_time_ns: start_ns,
            is_playing: false,
            speed: 1.0,
            last_wall_time_ms: 0.0,
            current_msg_index: 0,
            animation_frame_id: None,
            frame_counter: 0,
        };

        let inner = Rc::new(RefCell::new(state));
        let closure: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));

        let player = McapPlayer {
            inner,
            app_state,
            _closure: closure,
        };

        player.update_time_display();
        app_state.has_active_layout.set(true);

        log::info!(
            "Player ready: {} topics, {} messages",
            player.inner.borrow().mcap_data.topics.len(),
            player.inner.borrow().mcap_data.messages.len()
        );

        player
    }

    /// Get current playback time in nanoseconds.
    pub fn current_time_ns(&self) -> u64 {
        self.inner.borrow().current_time_ns
    }

    /// Get the latest message for a given topic at the current time.
    /// Returns a clone with Rc<Vec<u8>> data (cheap - just increments refcount).
    pub fn get_current_message(&self, topic: &str) -> Option<StoredMessage> {
        let state = self.inner.borrow();
        state
            .mcap_data
            .get_latest_message(topic, state.current_time_ns)
            .cloned()
    }

    /// Get topic list.
    pub fn topics(&self) -> Vec<TopicInfo> {
        self.inner.borrow().mcap_data.topics.clone()
    }

    /// Get messages for a topic within a time range (for panels that need history).
    pub fn get_messages_in_range(&self, topic: &str, start_ns: u64, end_ns: u64) -> Vec<StoredMessage> {
        let state = self.inner.borrow();
        state
            .mcap_data
            .get_messages_in_range(topic, start_ns, end_ns)
            .into_iter()
            .cloned()
            .collect()
    }

    /// Get all messages for a topic up to current time (for state transitions).
    pub fn get_topic_messages_until_now(&self, topic: &str) -> Vec<StoredMessage> {
        let state = self.inner.borrow();
        let current = state.current_time_ns;
        state
            .mcap_data
            .get_messages_in_range(topic, state.mcap_data.start_time_ns, current)
            .into_iter()
            .cloned()
            .collect()
    }

    /// Get time range info.
    pub fn time_range(&self) -> (u64, u64) {
        let state = self.inner.borrow();
        (state.mcap_data.start_time_ns, state.mcap_data.end_time_ns)
    }

    /// Get frame counter (increments every animation frame).
    pub fn frame_counter(&self) -> u64 {
        self.inner.borrow().frame_counter
    }

    pub fn play(&self) {
        let window = web_sys::window().unwrap();
        let now = window.performance().unwrap().now();

        {
            let mut state = self.inner.borrow_mut();
            state.is_playing = true;
            state.last_wall_time_ms = now;
        }

        self.app_state.is_playing.set(true);
        self.schedule_tick();
    }

    pub fn pause(&self) {
        {
            let mut state = self.inner.borrow_mut();
            state.is_playing = false;
            if let Some(id) = state.animation_frame_id.take() {
                web_sys::window().unwrap().cancel_animation_frame(id).ok();
            }
        }
        self.app_state.is_playing.set(false);
    }

    pub fn seek(&self, fraction: f64) {
        let mut state = self.inner.borrow_mut();
        let start = state.mcap_data.start_time_ns;
        let end = state.mcap_data.end_time_ns;
        let duration = end - start;

        let target_ns = start + (fraction * duration as f64) as u64;
        state.current_time_ns = target_ns.clamp(start, end);
        state.current_msg_index = state
            .mcap_data
            .messages
            .partition_point(|m| m.log_time_ns < target_ns);

        let window = web_sys::window().unwrap();
        state.last_wall_time_ms = window.performance().unwrap().now();
        state.frame_counter += 1;
        let frame = state.frame_counter;

        drop(state);
        self.update_time_display();
        self.update_progress();
        self.app_state.frame_tick.set(frame);
    }

    pub fn set_speed(&self, speed: f64) {
        self.inner.borrow_mut().speed = speed;
        self.app_state.playback_speed.set(speed);
    }

    fn schedule_tick(&self) {
        let inner = Rc::clone(&self.inner);
        let app_state = self.app_state;
        let closure_holder = Rc::clone(&self._closure);

        let closure = Closure::once(move || {
            tick_and_schedule(inner, app_state, closure_holder);
        });

        let window = web_sys::window().unwrap();
        let id = window
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .unwrap();
        self.inner.borrow_mut().animation_frame_id = Some(id);
        closure.forget();
    }

    fn update_time_display(&self) {
        let state = self.inner.borrow();
        let elapsed_ns = state.current_time_ns - state.mcap_data.start_time_ns;
        let elapsed_secs = elapsed_ns as f64 / 1_000_000_000.0;
        let mins = (elapsed_secs / 60.0).floor() as u32;
        let secs = elapsed_secs % 60.0;
        self.app_state
            .current_time_display
            .set(format!("{}:{:05.3}", mins, secs));
    }

    fn update_progress(&self) {
        let state = self.inner.borrow();
        let start = state.mcap_data.start_time_ns;
        let end = state.mcap_data.end_time_ns;
        let current = state.current_time_ns;
        let duration = end - start;
        let progress = if duration > 0 {
            (current - start) as f64 / duration as f64
        } else {
            0.0
        };
        self.app_state.playback_progress.set(progress * 100.0);
    }
}

/// Combined tick + schedule-next function to avoid deep recursion with Closure::once.
fn tick_and_schedule(
    inner: Rc<RefCell<PlaybackState>>,
    app_state: AppState,
    closure_holder: Rc<RefCell<Option<Closure<dyn FnMut()>>>>,
) {
    let should_continue = {
        let mut state = inner.borrow_mut();
        if !state.is_playing {
            return;
        }

        let window = web_sys::window().unwrap();
        let now = window.performance().unwrap().now();
        let wall_elapsed_ms = now - state.last_wall_time_ms;
        state.last_wall_time_ms = now;

        let capped_ms = wall_elapsed_ms.min(300.0);
        let advance_ns = (capped_ms * 1_000_000.0 * state.speed) as u64;
        state.current_time_ns += advance_ns;

        let end = state.mcap_data.end_time_ns;
        if state.current_time_ns >= end {
            state.current_time_ns = end;
            state.is_playing = false;
            app_state.is_playing.set(false);
            state.frame_counter += 1;
            false
        } else {
            state.frame_counter += 1;
            true
        }
    };

    // Update UI signals
    {
        let state = inner.borrow();
        let start = state.mcap_data.start_time_ns;
        let end = state.mcap_data.end_time_ns;
        let current = state.current_time_ns;
        let duration_ns = end - start;
        let progress = if duration_ns > 0 {
            (current - start) as f64 / duration_ns as f64
        } else {
            0.0
        };
        let elapsed_secs = (current - start) as f64 / 1_000_000_000.0;
        let frame = state.frame_counter;

        app_state.playback_progress.set(progress * 100.0);
        let mins = (elapsed_secs / 60.0).floor() as u32;
        let secs = elapsed_secs % 60.0;
        app_state
            .current_time_display
            .set(format!("{}:{:05.3}", mins, secs));
        app_state.frame_tick.set(frame);
    }

    if should_continue {
        // Schedule next frame
        let inner2 = Rc::clone(&inner);
        let closure_holder2 = Rc::clone(&closure_holder);

        let closure = Closure::once(move || {
            tick_and_schedule(inner2, app_state, closure_holder2);
        });

        let window = web_sys::window().unwrap();
        let id = window
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .unwrap();
        inner.borrow_mut().animation_frame_id = Some(id);
        *closure_holder.borrow_mut() = Some(closure);
    }
}

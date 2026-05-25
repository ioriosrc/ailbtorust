// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

//! MCAP file player with lazy chunk-based loading.
//!
//! Architecture (matches how original Lichtblick/Foxglove works):
//! 1. On file open: read only the MCAP footer/summary (a few KB) → instant open
//! 2. Summary gives: topics, schemas, time range, chunk index (byte offsets)
//! 3. During playback: load only the chunk(s) covering the current time window
//! 4. LRU cache of decoded chunks keeps memory bounded
//!
//! This handles 1.5GB+ files without OOM because only ~50-100MB is in memory at once.

use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::rc::Rc;

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::mcap_reader::{
    self, ChunkIndexEntry, McapSummary, DecodedMessage, ChannelInfo,
};
use crate::state::app_state::{AppState, TimeFormat};

/// A single stored message - data is shared (Rc) to avoid cloning large buffers.
#[derive(Clone, Debug)]
pub struct StoredMessage {
    pub log_time_ns: u64,
    pub topic: String,
    pub data: Rc<Vec<u8>>,
    pub schema_name: String,
    pub encoding: String,
}

#[derive(Clone, Debug)]
pub struct TopicInfo {
    pub name: String,
    pub schema_name: String,
    pub encoding: String,
    pub message_count: usize,
}

/// A decoded and cached chunk of messages.
struct CachedChunk {
    chunk_idx: usize,
    messages: Vec<StoredMessage>,
    /// Approximate memory usage of this chunk's message data.
    mem_bytes: usize,
}

/// Maximum total cached chunk data (~100MB - keeps tick scanning fast).
const MAX_CACHE_BYTES: usize = 100 * 1024 * 1024;

/// Player internal state for lazy-loading architecture.
struct PlaybackState {
    /// The JS File object - kept for on-demand chunk reads via File.slice().
    file: web_sys::File,
    /// Parsed MCAP summary (topics, schemas, chunk index).
    summary: McapSummary,
    /// Topic info derived from summary.
    topics: Vec<TopicInfo>,
    /// Channel ID → topic name + schema for decoding.
    channel_lookup: HashMap<u16, (String, String, String)>, // (topic, schema_name, encoding)
    /// Chunk index sorted by start time.
    chunk_indices: Vec<ChunkIndexEntry>,
    /// LRU cache of decoded chunks. Key = chunk_idx.
    chunk_cache: VecDeque<CachedChunk>,
    /// Total bytes in cache.
    cache_bytes: usize,
    /// Per-topic latest message cache (for panels that just need "current" message).
    latest_messages: HashMap<String, StoredMessage>,
    /// Playback timing.
    current_time_ns: u64,
    start_time_ns: u64,
    end_time_ns: u64,
    is_playing: bool,
    speed: f64,
    last_wall_time_ms: f64,
    animation_frame_id: Option<i32>,
    frame_counter: u64,
    /// Which chunk indices are currently loaded.
    loaded_chunk_indices: Vec<usize>,
    /// If a chunk load is in progress (async), don't start another.
    loading_in_progress: bool,
    /// Generation counter - incremented on seek to invalidate stale chunk loads.
    load_generation: u64,
}

/// Shared player handle.
pub struct McapPlayer {
    inner: Rc<RefCell<PlaybackState>>,
    app_state: AppState,
    _closure: Rc<RefCell<Option<Closure<dyn FnMut()>>>>,
}

impl McapPlayer {
    /// Create a new player from MCAP summary + File reference.
    /// This is INSTANT - no message data is loaded yet.
    pub fn new_lazy(file: web_sys::File, summary: McapSummary, app_state: AppState) -> Self {
        let start_time_ns = summary.statistics.message_start_time;
        let end_time_ns = summary.statistics.message_end_time;

        // Build topic info from channels + schemas
        let mut topics: Vec<TopicInfo> = summary
            .channels
            .values()
            .map(|ch| {
                let schema_name = summary
                    .schemas
                    .get(&ch.schema_id)
                    .map(|s| s.name.clone())
                    .unwrap_or_default();
                let message_count = summary.statistics.channel_message_counts
                    .get(&ch.id)
                    .copied()
                    .unwrap_or(0) as usize;
                TopicInfo {
                    name: ch.topic.clone(),
                    schema_name,
                    encoding: ch.message_encoding.clone(),
                    message_count,
                }
            })
            .collect();
        topics.sort_by(|a, b| a.name.cmp(&b.name));
        // Deduplicate topics (same topic can appear in multiple channels)
        topics.dedup_by(|a, b| a.name == b.name);

        // Build channel lookup
        let mut channel_lookup: HashMap<u16, (String, String, String)> = HashMap::new();
        for ch in summary.channels.values() {
            let schema_name = summary
                .schemas
                .get(&ch.schema_id)
                .map(|s| s.name.clone())
                .unwrap_or_default();
            channel_lookup.insert(
                ch.id,
                (ch.topic.clone(), schema_name, ch.message_encoding.clone()),
            );
        }

        let chunk_indices = summary.chunk_indices.clone();

        let state = PlaybackState {
            file,
            summary,
            topics,
            channel_lookup,
            chunk_indices,
            chunk_cache: VecDeque::new(),
            cache_bytes: 0,
            latest_messages: HashMap::new(),
            current_time_ns: start_time_ns,
            start_time_ns,
            end_time_ns,
            is_playing: false,
            speed: 1.0,
            last_wall_time_ms: 0.0,
            animation_frame_id: None,
            frame_counter: 0,
            loaded_chunk_indices: Vec::new(),
            loading_in_progress: false,
            load_generation: 0,
        };

        let inner = Rc::new(RefCell::new(state));
        let closure: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));

        let player = McapPlayer {
            inner: Rc::clone(&inner),
            app_state,
            _closure: closure,
        };

        player.update_time_display();
        app_state.has_active_layout.set(true);

        let topic_count = inner.borrow().topics.len();
        let chunk_count = inner.borrow().chunk_indices.len();
        let duration_secs = (end_time_ns - start_time_ns) as f64 / 1_000_000_000.0;
        log::info!(
            "Player ready (lazy): {} topics, {} chunks, {:.2}s duration",
            topic_count, chunk_count, duration_secs
        );

        // For small files (< 100MB), preload ALL chunks immediately to avoid
        // any chunk-boundary stutter during playback.
        let file_size = inner.borrow().file.size() as usize;
        if file_size < 100 * 1024 * 1024 {
            // Load all chunks - use the entire time range
            player.request_chunks_for_time(start_time_ns);
        } else {
            // Large file: only load chunks near the start
            player.request_chunks_for_time(start_time_ns);
        }

        player
    }

    /// Get current playback time in nanoseconds.
    pub fn current_time_ns(&self) -> u64 {
        self.inner.borrow().current_time_ns
    }

    pub fn start_time_ns(&self) -> u64 {
        self.inner.borrow().start_time_ns
    }

    pub fn end_time_ns(&self) -> u64 {
        self.inner.borrow().end_time_ns
    }

    /// Seek forward or backward by a given number of milliseconds.
    pub fn seek_by_ms(&self, delta_ms: i64) {
        let current = self.inner.borrow().current_time_ns;
        let start = self.inner.borrow().start_time_ns;
        let end = self.inner.borrow().end_time_ns;
        let delta_ns = delta_ms * 1_000_000;
        let target = if delta_ns >= 0 {
            current.saturating_add(delta_ns as u64).min(end)
        } else {
            current.saturating_sub((-delta_ns) as u64).max(start)
        };
        self.seek_to_ns(target);
    }

    /// Get the latest message for a given topic at the current time.
    /// Uses binary search through cached chunks for efficiency.
    pub fn get_current_message(&self, topic: &str) -> Option<StoredMessage> {
        let state = self.inner.borrow();
        state.latest_messages.get(topic).cloned()
    }

    /// Get topic list.
    pub fn topics(&self) -> Vec<TopicInfo> {
        self.inner.borrow().topics.clone()
    }

    /// Get per-topic statistics: message count and frequency (Hz).
    /// Calculated from loaded chunks in cache.
    pub fn topic_stats(&self) -> HashMap<String, (u64, f64)> {
        let state = self.inner.borrow();
        let duration_ns = state.end_time_ns.saturating_sub(state.start_time_ns);
        let duration_secs = duration_ns as f64 / 1_000_000_000.0;

        let mut result = HashMap::new();

        if !state.summary.statistics.channel_message_counts.is_empty() {
            // Use summary channel_message_counts for stable values (doesn't change during playback)
            for (&channel_id, &count) in &state.summary.statistics.channel_message_counts {
                if let Some((topic, _, _)) = state.channel_lookup.get(&channel_id) {
                    let hz = if duration_secs > 0.0 && count > 1 {
                        (count as f64 - 1.0) / duration_secs
                    } else {
                        0.0
                    };
                    // Accumulate if multiple channels map to same topic
                    let entry = result.entry(topic.clone()).or_insert((0u64, 0.0f64));
                    entry.0 += count;
                    entry.1 += hz;
                }
            }
        } else {
            // Fallback: compute from total messages per topic in chunk index.
            // Use TopicInfo.message_count if available.
            for topic_info in &state.topics {
                let count = topic_info.message_count as u64;
                let hz = if duration_secs > 0.0 && count > 1 {
                    (count as f64 - 1.0) / duration_secs
                } else {
                    0.0
                };
                result.insert(topic_info.name.clone(), (count, hz));
            }
        }
        result
    }

    /// Get messages for a topic within a time range (from loaded chunks only).
    pub fn get_messages_in_range(&self, topic: &str, start_ns: u64, end_ns: u64) -> Vec<StoredMessage> {
        let state = self.inner.borrow();
        let mut result = Vec::new();
        for chunk in &state.chunk_cache {
            for msg in &chunk.messages {
                if msg.topic == topic && msg.log_time_ns >= start_ns && msg.log_time_ns <= end_ns {
                    result.push(msg.clone());
                }
            }
        }
        result.sort_by_key(|m| m.log_time_ns);
        result
    }

    /// Get all messages for a topic up to current time (from loaded chunks).
    pub fn get_topic_messages_until_now(&self, topic: &str) -> Vec<StoredMessage> {
        let state = self.inner.borrow();
        let current = state.current_time_ns;
        let start = state.start_time_ns;
        drop(state);
        self.get_messages_in_range(topic, start, current)
    }

    /// Get time range info.
    pub fn time_range(&self) -> (u64, u64) {
        let state = self.inner.borrow();
        (state.start_time_ns, state.end_time_ns)
    }

    /// Get frame counter.
    pub fn frame_counter(&self) -> u64 {
        self.inner.borrow().frame_counter
    }

    pub fn play(&self) {
        {
            let mut state = self.inner.borrow_mut();
            state.is_playing = true;
            // Reset wall time so first frame uses 0ms advance (no jump)
            state.last_wall_time_ms = 0.0;
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
        let target_ns;
        {
            let mut state = self.inner.borrow_mut();
            let start = state.start_time_ns;
            let end = state.end_time_ns;
            let duration = end - start;
            target_ns = start + (fraction * duration as f64) as u64;
            state.current_time_ns = target_ns.clamp(start, end);
            let window = web_sys::window().unwrap();
            state.last_wall_time_ms = window.performance().unwrap().now();
            state.frame_counter += 1;
            // Invalidate any in-flight chunk loads from before this seek
            state.load_generation += 1;
            state.loading_in_progress = false;

            // Rebuild latest_messages for the new time position.
            // For each topic, find the latest message <= current_time across all cached chunks.
            let current_ns = state.current_time_ns;
            state.latest_messages.clear();
            // Collect one best message per topic per chunk, then merge
            let mut updates: Vec<(String, StoredMessage)> = Vec::new();
            for chunk in &state.chunk_cache {
                let msgs = &chunk.messages;
                if msgs.is_empty() {
                    continue;
                }
                if msgs.first().unwrap().log_time_ns > current_ns {
                    continue;
                }
                let pos = msgs.partition_point(|m| m.log_time_ns <= current_ns);
                let mut seen_in_chunk = std::collections::HashSet::new();
                for i in (0..pos).rev() {
                    let msg = &msgs[i];
                    if seen_in_chunk.contains(&msg.topic) {
                        continue;
                    }
                    seen_in_chunk.insert(msg.topic.clone());
                    updates.push((msg.topic.clone(), msg.clone()));
                }
            }
            for (topic, msg) in updates {
                match state.latest_messages.get(&topic) {
                    Some(existing) if existing.log_time_ns >= msg.log_time_ns => {},
                    _ => { state.latest_messages.insert(topic, msg); }
                }
            }
        }

        self.update_time_display();
        self.update_progress();
        self.app_state.frame_tick.set(self.inner.borrow().frame_counter);

        // Update URL time parameter
        update_url_time(self.inner.borrow().current_time_ns);

        // Load chunks for the new time position
        self.request_chunks_for_time(target_ns);
    }

    /// Seek to an absolute nanosecond timestamp.
    pub fn seek_to_ns(&self, time_ns: u64) {
        {
            let mut state = self.inner.borrow_mut();
            let start = state.start_time_ns;
            let end = state.end_time_ns;
            state.current_time_ns = time_ns.clamp(start, end);
            let window = web_sys::window().unwrap();
            state.last_wall_time_ms = window.performance().unwrap().now();
            state.frame_counter += 1;
            // Invalidate any in-flight chunk loads from before this seek
            state.load_generation += 1;
            state.loading_in_progress = false;

            // Rebuild latest_messages for the new time position
            let current_ns = state.current_time_ns;
            state.latest_messages.clear();
            let mut updates: Vec<(String, StoredMessage)> = Vec::new();
            for chunk in &state.chunk_cache {
                let msgs = &chunk.messages;
                if msgs.is_empty() {
                    continue;
                }
                if msgs.first().unwrap().log_time_ns > current_ns {
                    continue;
                }
                let pos = msgs.partition_point(|m| m.log_time_ns <= current_ns);
                let mut seen_in_chunk = std::collections::HashSet::new();
                for i in (0..pos).rev() {
                    let msg = &msgs[i];
                    if seen_in_chunk.contains(&msg.topic) {
                        continue;
                    }
                    seen_in_chunk.insert(msg.topic.clone());
                    updates.push((msg.topic.clone(), msg.clone()));
                }
            }
            for (topic, msg) in updates {
                match state.latest_messages.get(&topic) {
                    Some(existing) if existing.log_time_ns >= msg.log_time_ns => {},
                    _ => { state.latest_messages.insert(topic, msg); }
                }
            }
        }

        self.update_time_display();
        self.update_progress();
        self.app_state.frame_tick.set(self.inner.borrow().frame_counter);

        update_url_time(self.inner.borrow().current_time_ns);

        self.request_chunks_for_time(time_ns);
    }

    pub fn set_speed(&self, speed: f64) {
        self.inner.borrow_mut().speed = speed;
        self.app_state.playback_speed.set(speed);
    }

    /// Request loading chunks that cover the given time.
    /// For small files (< 100MB), loads ALL remaining chunks.
    /// For large files, prefetches 10s ahead.
    fn request_chunks_for_time(&self, time_ns: u64) {
        let inner = Rc::clone(&self.inner);

        let chunks_needed = {
            let state = inner.borrow();
            if state.loading_in_progress {
                return;
            }

            let file_size = state.file.size() as usize;
            let needed: Vec<usize> = if file_size < 100 * 1024 * 1024 {
                // Small file: load ALL chunks for seamless playback
                (0..state.chunk_indices.len())
                    .filter(|idx| !state.loaded_chunk_indices.contains(idx))
                    .collect()
            } else {
                // Large file: load 10s ahead
                let behind_ns = 1_000_000_000u64;
                let ahead_ns = 10_000_000_000u64;
                let start = time_ns.saturating_sub(behind_ns);
                let end = time_ns.saturating_add(ahead_ns).min(state.end_time_ns);

                mcap_reader::find_chunks_for_time(&state.chunk_indices, start, end)
                    .into_iter()
                    .filter(|idx| !state.loaded_chunk_indices.contains(idx))
                    .take(10)
                    .collect()
            };
            needed
        };

        if chunks_needed.is_empty() {
            return;
        }

        inner.borrow_mut().loading_in_progress = true;

        // Load chunks one at a time using File.slice() + FileReader
        let first_chunk_idx = chunks_needed[0];
        self.load_chunk_async(first_chunk_idx, chunks_needed[1..].to_vec());
    }

    /// Asynchronously load a single chunk from the file using File.slice().
    fn load_chunk_async(&self, chunk_idx: usize, remaining: Vec<usize>) {
        let inner = Rc::clone(&self.inner);
        let app_state = self.app_state;

        let (blob, channel_lookup, generation) = {
            let state = inner.borrow();
            let ci = &state.chunk_indices[chunk_idx];
            let start = ci.chunk_offset as f64;
            let end = (ci.chunk_offset + ci.chunk_length) as f64;
            let blob = state.file.slice_with_f64_and_f64(start, end)
                .unwrap_or_else(|_| {
                    state.file.slice_with_f64_and_f64(start, end + 9.0).unwrap()
                });
            (blob, state.channel_lookup.clone(), state.load_generation)
        };

        let reader = web_sys::FileReader::new().unwrap();
        let reader_clone = reader.clone();
        let inner_clone = Rc::clone(&inner);

        let onload = Closure::once(move |_: web_sys::Event| {
            let array_buffer = reader_clone.result().unwrap();
            let uint8_array = js_sys::Uint8Array::new(&array_buffer);
            let chunk_bytes = uint8_array.to_vec();

            // If a seek happened since this load started, discard and stop chain
            if inner_clone.borrow().load_generation != generation {
                inner_clone.borrow_mut().loading_in_progress = false;
                return;
            }

            if chunk_bytes.len() < 9 {
                log::warn!("Chunk {} too small: {} bytes", chunk_idx, chunk_bytes.len());
                inner_clone.borrow_mut().loading_in_progress = false;
                return;
            }

            let _opcode = chunk_bytes[0];
            let record_data = &chunk_bytes[9..];

            match mcap_reader::parse_chunk_messages(record_data, &channel_lookup_to_info(&channel_lookup)) {
                Ok(decoded_msgs) => {
                    // Convert DecodedMessage → StoredMessage
                    let mut messages: Vec<StoredMessage> = decoded_msgs
                        .into_iter()
                        .filter_map(|dm| {
                            let (topic, schema_name, encoding) = channel_lookup.get(&dm.channel_id)?;
                            Some(StoredMessage {
                                log_time_ns: dm.log_time,
                                topic: topic.clone(),
                                data: dm.data,
                                schema_name: schema_name.clone(),
                                encoding: encoding.clone(),
                            })
                        })
                        .collect();
                    messages.sort_by_key(|m| m.log_time_ns);

                    let mem_bytes: usize = messages.iter().map(|m| m.data.len() + 64).sum();

                    {
                        let mut state = inner_clone.borrow_mut();
                        let current_time = state.current_time_ns;

                        // Evict old chunks if cache is too large
                        while state.cache_bytes + mem_bytes > MAX_CACHE_BYTES
                            && !state.chunk_cache.is_empty()
                        {
                            let evicted = state.chunk_cache.pop_front().unwrap();
                            state.cache_bytes -= evicted.mem_bytes;
                            state.loaded_chunk_indices.retain(|&i| i != evicted.chunk_idx);
                        }

                        // Update latest_messages cache for newly loaded messages
                        for msg in &messages {
                            if msg.log_time_ns <= current_time {
                                match state.latest_messages.get(&msg.topic) {
                                    Some(existing) if existing.log_time_ns >= msg.log_time_ns => {},
                                    _ => {
                                        state.latest_messages.insert(msg.topic.clone(), msg.clone());
                                    }
                                }
                            }
                        }

                        state.chunk_cache.push_back(CachedChunk {
                            chunk_idx,
                            messages,
                            mem_bytes,
                        });
                        state.cache_bytes += mem_bytes;
                        state.loaded_chunk_indices.push(chunk_idx);
                    }
                }
                Err(e) => {
                    log::warn!("Failed to parse chunk {}: {}", chunk_idx, e);
                }
            }

            // Load next chunk if any
            inner_clone.borrow_mut().loading_in_progress = false;
            if !remaining.is_empty() {
                let next_idx = remaining[0];
                let rest = remaining[1..].to_vec();
                let inner2 = Rc::clone(&inner_clone);
                let load_next = Closure::once(move || {
                    // Check generation before continuing chain
                    if inner2.borrow().load_generation != generation {
                        return;
                    }
                    inner2.borrow_mut().loading_in_progress = true;
                    let state = inner2.borrow();
                    let ci = &state.chunk_indices[next_idx];
                    let start = ci.chunk_offset as f64;
                    let end = (ci.chunk_offset + ci.chunk_length) as f64;
                    let blob = state.file.slice_with_f64_and_f64(start, end).unwrap();
                    let channel_lookup = state.channel_lookup.clone();
                    drop(state);

                    load_chunk_from_blob(inner2, blob, next_idx, rest, channel_lookup, app_state, generation);
                });
                web_sys::window()
                    .unwrap()
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        load_next.as_ref().unchecked_ref(),
                        0,
                    )
                    .ok();
                load_next.forget();
            }
        });

        reader.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget();
        reader.read_as_array_buffer(&blob).unwrap();
    }

    fn schedule_tick(&self) {
        let inner = Rc::clone(&self.inner);
        let app_state = self.app_state;
        let closure_holder = Rc::clone(&self._closure);

        // Create a persistent FnMut closure that reuses itself via closure_holder.
        // This avoids allocating a new Closure per frame (unlike Closure::once).
        let c = Closure::wrap(Box::new(move || {
            tick_and_reschedule(&inner, app_state, &closure_holder);
        }) as Box<dyn FnMut()>);

        let window = web_sys::window().unwrap();
        let id = window
            .request_animation_frame(c.as_ref().unchecked_ref())
            .unwrap();
        self.inner.borrow_mut().animation_frame_id = Some(id);
        // Store the closure so it stays alive and can be re-registered each frame
        *self._closure.borrow_mut() = Some(c);
    }

    fn update_time_display(&self) {
        let state = self.inner.borrow();
        let current_ns = state.current_time_ns;
        let formatted = match self.app_state.time_format.get_untracked() {
            TimeFormat::TOD => format_timestamp_full(current_ns),
            TimeFormat::SEC => format_timestamp_secs(current_ns),
        };
        self.app_state.current_time_display.set(formatted);
    }

    fn update_progress(&self) {
        let state = self.inner.borrow();
        let start = state.start_time_ns;
        let end = state.end_time_ns;
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

/// Helper: load a chunk from a Blob (for chained async loads).
fn load_chunk_from_blob(
    inner: Rc<RefCell<PlaybackState>>,
    blob: web_sys::Blob,
    chunk_idx: usize,
    remaining: Vec<usize>,
    channel_lookup: HashMap<u16, (String, String, String)>,
    app_state: AppState,
    generation: u64,
) {
    let reader = web_sys::FileReader::new().unwrap();
    let reader_clone = reader.clone();
    let inner_clone = Rc::clone(&inner);

    let onload = Closure::once(move |_: web_sys::Event| {
        let array_buffer = reader_clone.result().unwrap();
        let uint8_array = js_sys::Uint8Array::new(&array_buffer);
        let chunk_bytes = uint8_array.to_vec();

        // If a seek happened since this load started, discard and stop chain
        if inner_clone.borrow().load_generation != generation {
            inner_clone.borrow_mut().loading_in_progress = false;
            return;
        }

        if chunk_bytes.len() < 9 {
            inner_clone.borrow_mut().loading_in_progress = false;
            return;
        }

        let record_data = &chunk_bytes[9..];

        match mcap_reader::parse_chunk_messages(record_data, &channel_lookup_to_info(&channel_lookup)) {
            Ok(decoded_msgs) => {
                let mut messages: Vec<StoredMessage> = decoded_msgs
                    .into_iter()
                    .filter_map(|dm| {
                        let (topic, schema_name, encoding) = channel_lookup.get(&dm.channel_id)?;
                        Some(StoredMessage {
                            log_time_ns: dm.log_time,
                            topic: topic.clone(),
                            data: dm.data,
                            schema_name: schema_name.clone(),
                            encoding: encoding.clone(),
                        })
                    })
                    .collect();
                messages.sort_by_key(|m| m.log_time_ns);

                let mem_bytes: usize = messages.iter().map(|m| m.data.len() + 64).sum();

                {
                    let mut state = inner_clone.borrow_mut();
                    let current_time = state.current_time_ns;
                    for msg in &messages {
                        if msg.log_time_ns > current_time {
                            continue;
                        }
                        match state.latest_messages.get(&msg.topic) {
                            Some(existing) if existing.log_time_ns >= msg.log_time_ns => {},
                            _ => {
                                state.latest_messages.insert(msg.topic.clone(), msg.clone());
                            }
                        }
                    }

                    while state.cache_bytes + mem_bytes > MAX_CACHE_BYTES
                        && !state.chunk_cache.is_empty()
                    {
                        let evicted = state.chunk_cache.pop_front().unwrap();
                        state.cache_bytes -= evicted.mem_bytes;
                        state.loaded_chunk_indices.retain(|&i| i != evicted.chunk_idx);
                    }

                    state.chunk_cache.push_back(CachedChunk {
                        chunk_idx,
                        messages,
                        mem_bytes,
                    });
                    state.cache_bytes += mem_bytes;
                    state.loaded_chunk_indices.push(chunk_idx);
                    // Do NOT bump frame_counter here - let the playback loop handle rendering
                }

                // No frame_tick signal - chunk loading is invisible to rendering
            }
            Err(e) => {
                log::warn!("Failed to parse chunk {}: {}", chunk_idx, e);
            }
        }

        inner_clone.borrow_mut().loading_in_progress = false;

        // Continue with remaining chunks
        if !remaining.is_empty() {
            let next_idx = remaining[0];
            let rest = remaining[1..].to_vec();
            let inner2 = Rc::clone(&inner_clone);
            let load_next = Closure::once(move || {
                // Check generation before continuing chain
                if inner2.borrow().load_generation != generation {
                    return;
                }
                inner2.borrow_mut().loading_in_progress = true;
                let state = inner2.borrow();
                let ci = &state.chunk_indices[next_idx];
                let start = ci.chunk_offset as f64;
                let end = (ci.chunk_offset + ci.chunk_length) as f64;
                let blob = state.file.slice_with_f64_and_f64(start, end).unwrap();
                let ch_lookup = state.channel_lookup.clone();
                drop(state);
                load_chunk_from_blob(inner2, blob, next_idx, rest, ch_lookup, app_state, generation);
            });
            web_sys::window()
                .unwrap()
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    load_next.as_ref().unchecked_ref(),
                    0,
                )
                .ok();
            load_next.forget();
        }
    });

    reader.set_onload(Some(onload.as_ref().unchecked_ref()));
    onload.forget();
    reader.read_as_array_buffer(&blob).unwrap();
}

/// Convert our channel_lookup to the format mcap_reader expects.
fn channel_lookup_to_info(lookup: &HashMap<u16, (String, String, String)>) -> HashMap<u16, ChannelInfo> {
    lookup
        .iter()
        .map(|(&id, (topic, _schema, encoding))| {
            (
                id,
                ChannelInfo {
                    id,
                    schema_id: 0,
                    topic: topic.clone(),
                    message_encoding: encoding.clone(),
                },
            )
        })
        .collect()
}

// ============================================================================
// URL Time Synchronization
// ============================================================================

thread_local! {
    static LAST_URL_UPDATE_MS: RefCell<f64> = RefCell::new(0.0);
}

/// Convert nanoseconds timestamp to RFC3339 string with nanosecond precision.
/// e.g. "1970-10-01T18:14:06.666040305Z"
fn ns_to_rfc3339(time_ns: u64) -> String {
    let secs = time_ns / 1_000_000_000;
    let nanos = (time_ns % 1_000_000_000) as u32;

    // Break into date/time components
    let mut remaining_secs = secs;

    // Days since epoch
    let days = remaining_secs / 86400;
    remaining_secs %= 86400;

    let hours = remaining_secs / 3600;
    remaining_secs %= 3600;
    let minutes = remaining_secs / 60;
    let seconds = remaining_secs % 60;

    // Convert days to year/month/day (simplified calendar)
    let (year, month, day) = days_to_date(days);

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:09}Z",
        year, month, day, hours, minutes, seconds, nanos
    )
}

/// Convert days since Unix epoch to (year, month, day).
fn days_to_date(days: u64) -> (i32, u32, u32) {
    // Algorithm from http://howardhinnant.github.io/date_algorithms.html
    let z = days as i64 + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64; // day of era [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365; // year of era [0, 399]
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // day of year [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = doy - (153 * mp + 2) / 5 + 1; // [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 }; // [1, 12]
    let y = if m <= 2 { y + 1 } else { y };

    (y as i32, m as u32, d as u32)
}

/// Parse an RFC3339 time string back to nanoseconds.
/// Supports: "2025-07-01T14:05:09.331293771Z"
pub fn parse_rfc3339_to_ns(s: &str) -> Option<u64> {
    let s = s.trim_end_matches('Z').trim_end_matches('z');

    // Split at 'T'
    let parts: Vec<&str> = s.split('T').collect();
    if parts.len() != 2 {
        return None;
    }

    // Parse date
    let date_parts: Vec<&str> = parts[0].split('-').collect();
    if date_parts.len() != 3 {
        return None;
    }
    let year: i32 = date_parts[0].parse().ok()?;
    let month: u32 = date_parts[1].parse().ok()?;
    let day: u32 = date_parts[2].parse().ok()?;

    // Parse time
    let time_str = parts[1];
    let (time_hms, frac) = if let Some(dot_pos) = time_str.find('.') {
        (&time_str[..dot_pos], &time_str[dot_pos + 1..])
    } else {
        (time_str, "")
    };

    let time_parts: Vec<&str> = time_hms.split(':').collect();
    if time_parts.len() != 3 {
        return None;
    }
    let hours: u64 = time_parts[0].parse().ok()?;
    let minutes: u64 = time_parts[1].parse().ok()?;
    let seconds: u64 = time_parts[2].parse().ok()?;

    // Parse fractional seconds as nanoseconds
    let nanos: u64 = if frac.is_empty() {
        0
    } else {
        let padded = format!("{:0<9}", &frac[..frac.len().min(9)]);
        padded.parse().unwrap_or(0)
    };

    // Convert date to days since epoch
    let days = date_to_days(year, month, day);

    let total_secs = days * 86400 + hours * 3600 + minutes * 60 + seconds;
    Some(total_secs * 1_000_000_000 + nanos)
}

/// Convert (year, month, day) to days since Unix epoch.
fn date_to_days(year: i32, month: u32, day: u32) -> u64 {
    let y = if month <= 2 { year as i64 - 1 } else { year as i64 };
    let m = if month <= 2 { month as i64 + 9 } else { month as i64 - 3 };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = (y - era * 400) as u64;
    let doy = (153 * m as u64 + 2) / 5 + day as u64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era as i64 * 146097 + doe as i64 - 719468;
    days as u64
}

/// Update the URL `time=` query parameter using replaceState (no navigation).
/// Throttled to max once per 500ms.
pub fn update_url_time(time_ns: u64) {
    let window = web_sys::window().unwrap();
    let now = window.performance().unwrap().now();

    let should_update = LAST_URL_UPDATE_MS.with(|last| {
        let prev = *last.borrow();
        if now - prev >= 500.0 {
            *last.borrow_mut() = now;
            true
        } else {
            false
        }
    });

    if !should_update {
        return;
    }

    let time_str = ns_to_rfc3339(time_ns);

    let location = window.location();
    let origin = location.origin().unwrap_or_default();
    let pathname = location.pathname().unwrap_or_default();

    // Build URL manually to avoid UrlSearchParams encoding colons as %3A
    let new_url = format!("{}{}?time={}", origin, pathname, time_str);

    let history = window.history().unwrap();
    history
        .replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&new_url))
        .ok();
}

/// Read the `time=` query parameter from the current URL.
/// Returns the parsed nanosecond timestamp if present and valid.
pub fn get_url_time_ns() -> Option<u64> {
    let window = web_sys::window()?;
    let location = window.location();
    let search = location.search().ok()?;

    let params = web_sys::UrlSearchParams::new_with_str(&search).ok()?;
    let time_str = params.get("time")?;

    parse_rfc3339_to_ns(&time_str)
}

/// Tick function: advance playback time with smoothing, update UI, and reschedule.
/// Uses exponential moving average (like original Lichtblick) to eliminate jitter.
fn tick_and_reschedule(
    inner: &Rc<RefCell<PlaybackState>>,
    app_state: AppState,
    closure_holder: &Rc<RefCell<Option<Closure<dyn FnMut()>>>>,
) {
    let (should_continue, current_time, prev_time) = {
        let mut state = inner.borrow_mut();
        if !state.is_playing {
            return;
        }

        let window = web_sys::window().unwrap();
        let now = window.performance().unwrap().now();

        // Calculate raw elapsed time since last frame.
        // Use direct wall-clock delta for accurate real-time playback.
        let raw_elapsed_ms = if state.last_wall_time_ms > 0.0 {
            now - state.last_wall_time_ms
        } else {
            0.0 // First frame: don't advance (just establish baseline)
        };
        state.last_wall_time_ms = now;

        // Cap to prevent huge jumps after tab-switch, GC pause, or slow frames.
        // Use a tight cap (50ms = 20fps minimum) to prevent visible jumps.
        let capped_ms = raw_elapsed_ms.clamp(0.0, 50.0);

        // Apply playback speed and convert directly to nanoseconds.
        let advance_ns = (capped_ms * state.speed * 1_000_000.0) as u64;
        let prev_time = state.current_time_ns;
        state.current_time_ns += advance_ns;

        let end = state.end_time_ns;
        if state.current_time_ns >= end {
            // Check if loop is enabled
            if app_state.loop_playback.get_untracked() {
                state.current_time_ns = state.start_time_ns;
                state.latest_messages.clear();
                state.frame_counter += 1;
                (true, state.current_time_ns, state.start_time_ns)
            } else {
                state.current_time_ns = end;
                state.is_playing = false;
                app_state.is_playing.set(false);
                state.frame_counter += 1;
                (false, state.current_time_ns, prev_time)
            }
        } else {
            state.frame_counter += 1;
            (true, state.current_time_ns, prev_time)
        }
    };

    // Update latest_messages incrementally: only scan chunks whose time range
    // overlaps the narrow window (prev_time..current_time].
    // Skip chunks entirely outside this window for O(relevant_chunks) instead of O(all_chunks).
    {
        let mut state = inner.borrow_mut();
        let current_ns = current_time;
        let prev_ns = prev_time;
        // Collect updates from chunks that overlap (prev_ns, current_ns]
        let mut updates: Vec<(String, StoredMessage)> = Vec::new();
        for chunk in &state.chunk_cache {
            let msgs = &chunk.messages;
            if msgs.is_empty() {
                continue;
            }
            // Skip chunk entirely if its time range doesn't overlap (prev_ns, current_ns]
            let chunk_start = msgs.first().unwrap().log_time_ns;
            let chunk_end = msgs.last().unwrap().log_time_ns;
            if chunk_end <= prev_ns || chunk_start > current_ns {
                continue;
            }
            // Find messages in range (prev_ns, current_ns]
            let start_pos = msgs.partition_point(|m| m.log_time_ns <= prev_ns);
            let end_pos = msgs.partition_point(|m| m.log_time_ns <= current_ns);
            for i in start_pos..end_pos {
                let msg = &msgs[i];
                updates.push((msg.topic.clone(), msg.clone()));
            }
        }
        // Apply updates
        for (topic, msg) in updates {
            match state.latest_messages.get(&topic) {
                Some(existing) if existing.log_time_ns >= msg.log_time_ns => {},
                _ => { state.latest_messages.insert(topic, msg); }
            }
        }
    }

    // Update UI signals (single borrow, compute everything at once)
    {
        let state = inner.borrow();
        let start = state.start_time_ns;
        let end = state.end_time_ns;
        let current = state.current_time_ns;
        let duration_ns = end - start;
        let progress = if duration_ns > 0 {
            (current - start) as f64 / duration_ns as f64
        } else {
            0.0
        };
        let frame = state.frame_counter;

        app_state.playback_progress.set(progress * 100.0);
        let formatted = match app_state.time_format.get_untracked() {
            TimeFormat::TOD => format_timestamp_full(current),
            TimeFormat::SEC => format_timestamp_secs(current),
        };
        app_state.current_time_display.set(formatted);
        // Throttle frame_tick to every other frame (~30fps panel updates)
        // This halves the reactive work while keeping time/progress smooth at 60fps
        if frame % 2 == 0 {
            app_state.frame_tick.set(frame);
        }
    }

    // Update URL time parameter outside the render-critical path.
    // Throttled internally to every 500ms, but still avoid calling
    // performance.now() on every frame - only check every 30 frames.
    {
        let frame_counter = inner.borrow().frame_counter;
        if frame_counter % 30 == 0 {
            update_url_time(current_time);
        }
    }

    // Check if we need to load new chunks for current time (prefetch ahead)
    {
        let state = inner.borrow();
        let file_size = state.file.size() as usize;
        let (needed, any_missing) = if file_size < 100 * 1024 * 1024 {
            // Small file: ensure all chunks are loaded
            let needed: Vec<usize> = (0..state.chunk_indices.len())
                .filter(|idx| !state.loaded_chunk_indices.contains(idx))
                .collect();
            let any = !needed.is_empty();
            (needed, any)
        } else {
            // Large file: prefetch 3s ahead to avoid blocking too much main thread
            let behind_ns = 500_000_000u64; // 0.5s behind
            let ahead_ns = 3_000_000_000u64; // 3s ahead
            let start = current_time.saturating_sub(behind_ns);
            let end = current_time.saturating_add(ahead_ns).min(state.end_time_ns);
            let needed = mcap_reader::find_chunks_for_time(&state.chunk_indices, start, end);
            let any = needed.iter().any(|idx| !state.loaded_chunk_indices.contains(idx));
            let needed: Vec<usize> = needed
                .into_iter()
                .filter(|idx| !state.loaded_chunk_indices.contains(idx))
                .collect();
            (needed, any)
        };

        if any_missing && !state.loading_in_progress {
            // Load only 2 chunks at a time to keep main thread responsive
            let missing: Vec<usize> = needed.into_iter().take(2).collect();
            let gen = state.load_generation;
            drop(state);

            if !missing.is_empty() {
                let mut state = inner.borrow_mut();
                state.loading_in_progress = true;
                let ci = &state.chunk_indices[missing[0]];
                let file_start = ci.chunk_offset as f64;
                let file_end = (ci.chunk_offset + ci.chunk_length) as f64;
                let blob = state.file.slice_with_f64_and_f64(file_start, file_end).unwrap();
                let channel_lookup = state.channel_lookup.clone();
                drop(state);

                let remaining = if missing.len() > 1 { missing[1..].to_vec() } else { Vec::new() };
                load_chunk_from_blob(Rc::clone(inner), blob, missing[0], remaining, channel_lookup, app_state, gen);
            }
        }
    }

    if should_continue {
        // Reschedule using the same persistent closure (no allocation per frame)
        let window = web_sys::window().unwrap();
        if let Some(ref closure) = *closure_holder.borrow() {
            let id = window
                .request_animation_frame(closure.as_ref().unchecked_ref())
                .unwrap();
            inner.borrow_mut().animation_frame_id = Some(id);
        }
    }
}

// ============================================================================
// Legacy API: parse_mcap_file for small files (< 500MB)
// ============================================================================

/// Parse MCAP file fully into memory (for small files only).
/// For large files, use McapPlayer::new_lazy() instead.
pub fn parse_mcap_file(data: &[u8]) -> Result<(McapSummary, Vec<StoredMessage>), String> {
    let total_bytes = data.len();
    log::info!("Parsing MCAP file (full): {:.1} MB", total_bytes as f64 / 1_048_576.0);

    // First try to read summary from the file (for full file, footer is at the end)
    let tail_size = 64.min(data.len());
    let footer_data = &data[data.len() - tail_size..];
    let summary_start = mcap_reader::get_summary_start_from_footer(footer_data)?;
    let summary_section = &data[summary_start as usize..];
    let summary = mcap_reader::parse_summary_section(summary_section)?;

    // Then parse all messages using the mcap crate (for small files this is fine)
    let mut messages: Vec<StoredMessage> = Vec::new();
    let mut skipped = 0usize;

    let stream = mcap::MessageStream::new(data)
        .map_err(|e| format!("Failed to open MCAP: {}", e))?;

    for msg_result in stream {
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
        let schema_name = msg.channel.schema.as_ref()
            .map(|s| s.name.clone())
            .unwrap_or_default();
        let encoding = msg.channel.message_encoding.clone();

        messages.push(StoredMessage {
            log_time_ns: msg.log_time,
            topic,
            data: Rc::new(msg.data.to_vec()),
            schema_name,
            encoding,
        });
    }

    if skipped > 0 {
        log::warn!("Skipped {} bad messages", skipped);
    }

    messages.sort_by_key(|m| m.log_time_ns);
    Ok((summary, messages))
}

// ============================================================================
// Time Formatting Utilities
// ============================================================================

/// Format a nanosecond timestamp as date only "YYYY-MM-DD"
pub fn format_date(ns: u64) -> String {
    let total_secs = ns / 1_000_000_000;
    let secs_in_day = 86400u64;
    let days = total_secs / secs_in_day;
    let (year, month, day) = days_to_date(days);
    format!("{:04}-{:02}-{:02}", year, month, day)
}

/// Format a nanosecond timestamp as time only "h:mm:ss.SSS AM/PM"
pub fn format_time_only(ns: u64) -> String {
    let total_secs = ns / 1_000_000_000;
    let millis = (ns % 1_000_000_000) / 1_000_000;
    let time_of_day = total_secs % 86400;

    let hours24 = (time_of_day / 3600) as u32;
    let minutes = ((time_of_day % 3600) / 60) as u32;
    let seconds = (time_of_day % 60) as u32;

    let (hours12, ampm) = if hours24 == 0 {
        (12, "AM")
    } else if hours24 < 12 {
        (hours24, "AM")
    } else if hours24 == 12 {
        (12, "PM")
    } else {
        (hours24 - 12, "PM")
    };

    format!("{}:{:02}:{:02}.{:03} {}", hours12, minutes, seconds, millis, ampm)
}

/// Format a nanosecond timestamp as "YYYY-MM-DD h:mm:ss.SSS AM/PM"
pub fn format_timestamp_full(ns: u64) -> String {
    let total_secs = ns / 1_000_000_000;
    let millis = (ns % 1_000_000_000) / 1_000_000;

    // Convert to date/time components using seconds since epoch
    let secs_in_day = 86400u64;
    let mut days = total_secs / secs_in_day;
    let time_of_day = total_secs % secs_in_day;

    let hours24 = (time_of_day / 3600) as u32;
    let minutes = ((time_of_day % 3600) / 60) as u32;
    let seconds = (time_of_day % 60) as u32;

    let (hours12, ampm) = if hours24 == 0 {
        (12, "AM")
    } else if hours24 < 12 {
        (hours24, "AM")
    } else if hours24 == 12 {
        (12, "PM")
    } else {
        (hours24 - 12, "PM")
    };

    // Calculate year/month/day from days since epoch (1970-01-01)
    let (year, month, day) = days_to_date(days);

    format!(
        "{:04}-{:02}-{:02} {}:{:02}:{:02}.{:03} {}",
        year, month, day, hours12, minutes, seconds, millis, ampm
    )
}

/// Format nanoseconds as duration string "H:MM:SS.mmm"
pub fn format_duration_ns(ns: u64) -> String {
    let total_secs = ns / 1_000_000_000;
    let millis = (ns % 1_000_000_000) / 1_000_000;
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;
    format!("{}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis)
}

/// Format nanoseconds as elapsed seconds "123.456789012 sec"
pub fn format_elapsed_secs(ns: u64) -> String {
    let secs = ns / 1_000_000_000;
    let frac = ns % 1_000_000_000;
    format!("{}.{:09} sec", secs, frac)
}

/// Format nanosecond timestamp as seconds with decimal (e.g. "23652843.985514904")
pub fn format_timestamp_secs(ns: u64) -> String {
    let secs = ns / 1_000_000_000;
    let frac = ns % 1_000_000_000;
    format!("{}.{:09}", secs, frac)
}

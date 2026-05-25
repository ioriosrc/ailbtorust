# Agents - Lichtblick Rust/WASM

## Project Context

This is a Rust/WASM port of [Lichtblick](https://github.com/lichtblick-suite/lichtblick) — a robotics data visualization web app. It uses **Leptos 0.7.8** for reactive UI, compiled to `wasm32-unknown-unknown`, served via **Trunk**.

The reference implementation is the original Lichtblick TypeScript/React app. When in doubt about behavior, match the original.

---

## Agent Roles

### Default Agent
General-purpose development. Handles feature implementation, bug fixes, refactoring.

**Key knowledge:**
- Build: `cargo build --target wasm32-unknown-unknown`
- Dev server: `trunk serve --port 8081` (from project root)
- Main app crate: `crates/lichtblick-app/`
- CSS: `web/style.css` (no CSS framework, manual styles)

### Performance Agent
For diagnosing and fixing playback stuttering, frame drops, memory issues.

**Focus areas:**
- `player.rs` → `tick_and_reschedule()` - the 60fps playback loop
- Chunk loading pipeline (File.slice → FileReader → parse → cache)
- Signal cascading from `frame_tick` → all panel Effects
- Memory: chunk_cache eviction, Rc<Vec<u8>> data sharing

**Proven fixes:**
- Time-range early-out before binary search in chunk scan
- Throttle `frame_tick` to every 2nd frame
- Reduce prefetch batch (2 chunks, not 5)
- Reduce ahead window (3s, not 10s)
- Generation counter to discard stale async loads

### MCAP Agent
For MCAP file format work: parsing, seeking, chunk management.

**Key files:**
- `src/mcap_reader.rs` - Summary parser, chunk decoder, LZ4/zstd
- `src/player.rs` - Lazy loading orchestration

**Format knowledge:**
- Statistics record has `channel_message_counts` map at offset 42+
- Entries: `channel_id(u16) + count(u64)`, length-prefixed with u32
- ChunkIndex gives time range + file offset per chunk
- Chunks contain compressed records (Message, Schema, Channel)

### UI/Sidebar Agent
For sidebar tabs, panel settings, layout management.

**Key files:**
- `src/components/sidebar.rs` - 4-tab interface (Panel/Topics/Alerts/Layouts)
- `src/components/topic_list.rs` - Filterable topic list with Hz/count
- `src/state/app_state.rs` - AppState, LayoutState, signals

**Sidebar architecture:**
- Tab 0 (Panel): Shows settings for `active_settings_panel` via gear icon
- Tab 1 (Topics): `TopicList` component with search filter, Hz from summary stats
- Tab 2 (Alerts): Checks for >60Hz topics, shows warning matching original Lichtblick
- Tab 3 (Layouts): Create/import/export/load from localStorage

### Panel Agent
For creating or modifying visualization panels.

**Pattern:**
1. Panel component in `src/panels/{name}_panel.rs`
2. Uses `frame_tick.get()` to subscribe to playback updates
3. Gets data via `player.get_current_message(&topic)` 
4. Only processes when timestamp changes (dedup check)
5. Registered in `PanelType` enum in `app_state.rs`

**Existing panels:** Image, ThreeDee, RawMessages, Log, Plot, DataSourceInfo, Diagnostics, StateTransitions, Teleop

---

## Critical Rules

1. **Topic stats must be stable** - use `channel_message_counts` from summary, never count from chunk_cache
2. **Seek must invalidate stale loads** - always check `load_generation` in async callbacks
3. **Never scan all cached chunks unconditionally** - use time-range checks first
4. **Views need owned Strings** - `t.name.clone()` not `&t.name`
5. **RefCell borrow conflicts** - collect updates into Vec, then apply separately
6. **Port 8081 for dev** - always kill existing process first
7. **Match Lichtblick original behavior** - when the TypeScript version does X, we do X

---

## Test MCAP Characteristics
- Size: 132MB, 880 chunks, ~20s duration, 219 topics
- Image topics at 12.5fps (12.50 Hz, 250 messages)
- Multiple topics at various frequencies (6.5-12.5 Hz range visible)
- Has topics >60Hz that trigger the performance alert
- Compression: LZ4 (most chunks)

---

## File Quick Reference

| Path | Purpose |
|------|---------|
| `Trunk.toml` | Build config (target=web/index.html, watch=crates+web) |
| `web/index.html` | Entry HTML |
| `web/style.css` | All CSS styles |
| `crates/lichtblick-app/src/app.rs` | Root Leptos component |
| `crates/lichtblick-app/src/player.rs` | MCAP player (most complex file) |
| `crates/lichtblick-app/src/mcap_reader.rs` | MCAP format parser |
| `crates/lichtblick-app/src/decoder.rs` | CDR/ROS1 message decoders |
| `crates/lichtblick-app/src/components/` | UI components (sidebar, toolbar, layout) |
| `crates/lichtblick-app/src/panels/` | Panel implementations |
| `crates/lichtblick-app/src/state/` | Global state (AppState, LayoutState) |

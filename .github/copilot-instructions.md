# Copilot Agent Instructions for Lichtblick (Rust/WASM)

## Project Overview
A complete port of [Lichtblick](https://github.com/lichtblick-suite/lichtblick) web application to Rust/WebAssembly. Robotics data visualization tool supporting MCAP files with lazy chunk loading, real-time playback, and panel-based layout system.

## Tech Stack
- **Language**: Rust 1.95+ → `wasm32-unknown-unknown`
- **UI Framework**: Leptos 0.7.8 (CSR mode, `csr` feature)
- **Build Tool**: Trunk 0.21.14
- **Core Domain**: MCAP file playback, ROS/CDR message decoding, WebGL 3D rendering

## Crate Architecture
```
crates/
├── lichtblick-app         # Web UI (Leptos components, player, state management)
│   ├── src/player.rs      # MCAP lazy player (chunk loading, playback tick, seek)
│   ├── src/mcap_reader.rs # MCAP format parser (summary, chunks, LZ4/zstd)
│   ├── src/decoder.rs     # CDR/ROS1 message decoders (image, pointcloud, etc.)
│   ├── src/components/    # Sidebar tabs, topic list, panel layout, toolbar
│   ├── src/panels/        # Image, 3D, RawMessages, Plot, Log, Diagnostics, etc.
│   └── src/state/         # AppState, LayoutState (reactive signals)
├── lichtblick-core        # Types: Time, Topic, MessageEvent, PlayerState
├── lichtblick-messages    # Message path parsing/evaluation
├── lichtblick-mcap        # MCAP reading (schema parsing, source interface)
├── lichtblick-players     # Player traits (Iterable, WebSocket)
├── lichtblick-datasources # Data source factories
├── lichtblick-panels      # Panel config types
└── lichtblick-theme       # Theme system (dark/light)
```

## Development Commands
```bash
# Build check (fast)
cargo build --target wasm32-unknown-unknown

# Dev server (must run from project root!)
cd /path/to/ailbtorust && trunk serve --port 8081

# Tests
cargo test

# Lint
cargo clippy --target wasm32-unknown-unknown -- -D warnings
```

## MCAP Player Architecture (Critical Knowledge)

### Lazy Loading
- Only the MCAP summary (footer) is read on file open → instant start
- Chunks are loaded on-demand via `File.slice()` + `FileReader.readAsArrayBuffer()`
- Each chunk is decompressed (LZ4/zstd) and messages are stored as `StoredMessage`

### Playback Loop (`tick_and_reschedule`)
1. Advances `current_time_ns` by wall-clock delta × speed
2. Scans chunk_cache for messages in `(prev_time, current_time]` window
3. Updates `latest_messages` HashMap (per-topic most recent message)
4. Fires `frame_tick` signal every 2nd frame (~30fps for panels, 60fps for progress)
5. Prefetches chunks 3s ahead (large files) or all chunks (small <100MB files)

### Seek Safety
- `load_generation: u64` counter incremented on every seek
- All chunk load callbacks check generation before applying results
- Stale loads (from before seek) are silently discarded

### Performance Rules
- **Never scan all chunks every frame** - use time-range early-out
- **Collect-then-apply pattern** for RefCell borrow conflicts
- **Throttle frame_tick** to reduce reactive cascade
- **Batch chunk loads** at max 2 per tick to keep main thread responsive
- **100MB cache cap** - evict oldest chunks to limit scan work

### Topic Stats (must be stable!)
- Computed from `McapStatistics.channel_message_counts` (parsed from MCAP footer)
- Formula: `hz = (count - 1) / duration_secs`
- These values NEVER change during playback (unlike chunk-cache counting)

## Leptos Patterns

### Signals & Reactivity
```rust
let state = use_app_state();       // AppState from context
let layout = use_layout_state();   // LayoutState from context
let frame_tick = state.frame_tick; // RwSignal<u64> - triggers panel re-renders
```

### View Requirements
- Views MUST use owned types: `String`, not `&str` or `&String`
- Use `.into_any()` when returning different view types from match/if
- Use `collect_view()` to render iterators in templates
- `class:active=move || bool_expr` for conditional CSS classes

### Common Borrow Issues
```rust
// WRONG: can't iterate chunk_cache while mutating latest_messages (same RefCell)
for chunk in &state.chunk_cache {
    state.latest_messages.insert(...); // ERROR!
}

// CORRECT: collect first, then apply
let updates: Vec<_> = state.chunk_cache.iter()...collect();
for (k, v) in updates { state.latest_messages.insert(k, v); }
```

### Closures & Ownership
```rust
// Clone BEFORE moving into closure if you need the value after
let input_clone = input.clone();
let closure = Closure::once(move |_| { input_clone.files()... });
input.set_onchange(...); // Still works - we have the original
input.click();
```

## Alerts System
- Fires when any topic has Hz > 60 (excluding log schemas)
- Log schemas excluded: `rosgraph_msgs/Log`, `rcl_interfaces/msg/Log`, `foxglove.Log`
- Message matches Lichtblick original exactly

## Layout System
- `LayoutNode` enum: `Panel { id, panel_type, topic }` | `Split { id, direction, ratio, first, second }`
- JSON format matches Lichtblick: `{ configById, layout, playbackConfig, globalVariables }`
- Saved/loaded from localStorage with `layout:` prefix keys

## When Creating/Modifying Panels
1. Panel component in `src/panels/` - uses `frame_tick.get()` for reactivity
2. Gets latest message via `player.get_current_message(&topic)`
3. Only decodes when timestamp changes (skip redundant frames)
4. Register in `PanelType` enum and panel factory

## When Debugging Playback Issues
1. Check `load_generation` - are stale loads being discarded?
2. Check `latest_messages` update logic - correct time window?
3. Check chunk_cache scan - are irrelevant chunks being skipped?
4. Check image panel - is it blocking on decode? (should use Blob URLs)
5. Use browser DevTools Performance tab to find frame drops

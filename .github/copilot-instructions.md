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

## Extension System Architecture

### Overview
Extensions are `.foxe` ZIP archives containing JavaScript converter code. They are stored in IndexedDB and activated at startup. The primary use case is message converters that transform protobuf messages into FrameTransforms for the 3D panel.

### Key Files
- `src/extensions/manager.rs` — JS bridge (inline_js block), converter registry, protobufjs init
- `src/extensions/storage.rs` — IndexedDB CRUD operations
- `src/extensions/loader.rs` — ZIP parser (Store + Deflate methods)
- `src/extensions/types.rs` — ExtensionInfo, StoredExtension, ContributionPoints

### JavaScript Global State (in manager.rs inline_js)
```javascript
globalThis.__extensionConverters  // { fromSchemaName: [converterFn, ...] }
globalThis.__protoDeserializers   // { schemaName: deserializeFn }
globalThis.__protoDescriptorRoot  // protobufjs Root from descriptor.json
globalThis.__protobufjs           // protobufjs library reference
```

### Critical Functions (extern "C" in manager.rs)
- `js_init_protobuf()` — ASYNC. Loads protobuf CDN + fetches descriptor.json
- `js_is_protobuf_ready()` — Returns true when both __protobufjs and __protoDescriptorRoot exist
- `js_execute_extension(source, id, name)` — Activates extension JS, stores converters
- `js_register_proto_schema(name, data)` — Decodes FileDescriptorSet, caches deserializer
- `js_convert_message_to_frames(schema, msgData)` — Full pipeline: deserialize → convert → extract
- `js_has_converters(fromSchemaName)` — Checks if converters exist for schema

### Protobufjs Browser Constraints (IMPORTANT)
- protobufjs CDN bundle does NOT include `ext/descriptor` (it uses CommonJS `require()`)
- Solution: fetch `descriptor.json` directly → `Root.fromJSON(json)` → store globally
- This is an ASYNC operation — schema registration fails silently until ready
- Never mark a schema as "failed" if protobuf simply hasn't loaded yet (race condition)

### Converter Pipeline (in three_dee_panel.rs frame tick Effect)
1. Check if `js_has_converters(schema_name)` → converters exist for this topic
2. Check if schema is in REGISTERED_SCHEMAS → already decoded
3. If not registered: check `js_is_protobuf_ready()` → skip if not ready (DON'T fail)
4. Register schema: `js_register_proto_schema(name, schema_data)` → decode FileDescriptorSet
5. Check timestamp dedup: skip if same `log_time_ns` as last processed
6. Call `js_convert_message_to_frames(schema, msgData)` → returns JsValue array
7. Parse each frame transform → insert into TfTree

### TF Tree System
- **File**: `src/panels/tf_tree.rs`
- `TfTree::insert(StampedTransform)` — adds/updates transform in tree
- `TfTree::frames()` → Vec<String> of all known frame names
- `TfTree::lookup(target, source, time)` → chain transform with SLERP interpolation
- Display Frame dropdown reads `state.tf_frames` signal (populated from TfTree::frames)
- Auto-select from preferred: ["map", "odom", "world", "earth", "base_link", "Global"]

## When Working on Extensions
1. The inline_js block in manager.rs is the JavaScript bridge — ALL JS lives there
2. Protobuf init is async — test with `js_is_protobuf_ready()` before assuming it works
3. Browser environment: no `require()`, no Node.js APIs, no CommonJS modules
4. Extensions call `registerMessageConverter({ fromSchemaName, toSchemaName, converter })` during activation
5. `converter(message, event)` receives a deserialized JS object and returns `{ frameTransforms: [...] }`
6. Failed schemas are tracked per-session — restart to retry genuinely broken schemas

## When Debugging 3D/TF Issues
1. Check browser console for `[Extension]` prefixed messages
2. Verify protobuf loaded: `globalThis.__protobufjs` and `globalThis.__protoDescriptorRoot` exist
3. Check `globalThis.__extensionConverters` has entries for the expected schema
4. Check `globalThis.__protoDeserializers` has the schema after first message processed
5. If frames don't appear: verify converter returns `frameTransforms` array (not `frame_transforms`)
6. TfTree is in a thread-local — frames only appear after at least one transform is inserted

# Copilot Agent Instructions for Lichtblick (Rust/WASM)

## Project Overview
A complete port of [Lichtblick](https://github.com/lichtblick-suite/lichtblick) web application to Rust/WebAssembly. Robotics data visualization tool supporting MCAP files with lazy chunk loading, real-time playback, and panel-based layout system. **Zero JavaScript runtime** — all logic runs in WASM, with minimal JS glue for browser APIs and extension converters.

## Tech Stack
- **Language**: Rust 1.95+ → `wasm32-unknown-unknown`
- **UI Framework**: Leptos 0.7.8 (CSR mode, `csr` feature)
- **Build**: `cargo build --target wasm32-unknown-unknown` + `wasm-bindgen --target web`
- **Dev Server**: `python3 -m http.server 8081 --bind 127.0.0.1` from `dist/`
- **Core Domain**: MCAP file playback, ROS/CDR/Protobuf message decoding, WebGL2 3D rendering
- **Protobuf**: `prost-reflect 0.14` with `serde` feature (DynamicMessage + reflection)
- **Serialization**: Custom JSON serializer (snake_case proto fields → camelCase via JS bridge)
- **Extensions**: JS converter bridge (Rust→JSON string→JS parse+convertKeysToCamel→converter)

## Crate Architecture
```
crates/
├── lichtblick-app         # Web UI (Leptos components, player, state management)
│   ├── src/player.rs      # MCAP lazy player (chunk loading, playback tick, seek)
│   ├── src/mcap_reader.rs # MCAP format parser (summary, chunks, LZ4/zstd)
│   ├── src/decoder.rs     # CDR/ROS1/Protobuf message decoders
│   ├── src/extensions/    # Extension system (JS bridge, .foxe loader, IndexedDB)
│   │   ├── manager.rs     # JS bridge, converter registry, snakeToCamel, protobufjs init
│   │   ├── storage.rs     # IndexedDB persistence
│   │   ├── loader.rs      # .foxe ZIP parser (Store + Deflate)
│   │   └── types.rs       # ExtensionInfo, StoredExtension
│   ├── src/components/    # Sidebar tabs, topic list, panel layout, toolbar
│   ├── src/panels/        # Panel implementations
│   │   ├── three_dee_panel.rs  # WebGL2 3D + TF cache + SceneUpdate + OSI
│   │   ├── raw_messages_panel.rs  # Virtualized JSON tree (flat list, ~20 DOM nodes)
│   │   └── tf_tree.rs     # TfTree, transforms, SLERP interpolation
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
# Build WASM (debug — ~20s)
cargo build --target wasm32-unknown-unknown

# Generate JS bindings + deploy
wasm-bindgen target/wasm32-unknown-unknown/debug/lichtblick_app.wasm --out-dir dist --target web --no-typescript

# Serve (from project root, dist/ already contains index.html)
python3 -m http.server 8081 --bind 127.0.0.1 --directory dist

# Release build (optimized, ~2min)
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/lichtblick_app.wasm --out-dir dist --target web --no-typescript

# Tests
cargo test

# Lint
cargo clippy --target wasm32-unknown-unknown -- -D warnings

# Check for unused deps
cargo +nightly udeps --target wasm32-unknown-unknown
```

## MCAP Player Architecture

### Lazy Loading
- Only MCAP summary (footer) read on file open → instant start (~50ms for 1.4GB file)
- Chunks loaded on-demand via `File.slice()` + `FileReader.readAsArrayBuffer()`
- Each chunk decompressed (LZ4/zstd) → messages stored as `StoredMessage { topic, schema_name, log_time_ns, data: Rc<Vec<u8>> }`

### Playback Loop (`tick_and_reschedule`)
1. Advances `current_time_ns` by wall-clock delta × speed
2. Scans chunk_cache for messages in `(prev_time, current_time]` window
3. Updates `latest_messages` HashMap (per-topic most recent message)
4. Fires `frame_tick` signal every 2nd RAF (~30fps panels, 60fps progress bar)
5. Prefetches chunks 3s ahead (large files) or all chunks (small <100MB files)

### Seek Safety
- `load_generation: u64` counter incremented on every seek
- All chunk load callbacks check generation before applying results
- Stale loads silently discarded

## Performance Rules (Rust/WASM Critical)

### Zero-Allocation Hot Paths
- **Never allocate in render loop** — reuse `Vec`, `String` buffers across frames
- **`Rc<Vec<u8>>` for message data** — zero-copy sharing between player and panels
- **Pre-size Vecs** with `Vec::with_capacity()` when length is known
- **Collect-then-apply** for `RefCell` borrow conflicts (no nested borrows)

### Protobuf Decode Strategy
- OSI messages ~650KB each → only decode **1 message per tick** (full snapshot pattern)
- Custom `dynamic_message_to_json()` — writes directly to `String` buffer, no serde
- **WASM Stack Overflow Avoidance**: The custom `write_message_json` MUST check `msg.has_field(&field_desc)` for all singular message and oneof fields, skipping them if unset. This prevents infinite recursion on deep/cyclic protobuf schemas (like ASAM OSI) which panics browser WASM.
- `write_value_json()` / `write_message_json()` avoid intermediate allocations
- Timestamps as `{"sec": N, "nsec": N}` not RFC3339 strings

### Rendering Strategy
- **Single `render()` per tick** — never render mid-frame
- **TF cache per frame** — `HashMap<String, Option<[f32; 16]>>` cleared once at frame start
- **`scene_dirty` flag** — skip render if no scene entities changed
- **WebGL2 batch** — group by shader, minimize draw calls and state changes

### DOM Virtualization
- Raw Messages: flat virtual list, only ~20 visible rows rendered
- `FlatRow` struct + `flatten_json()` → absolute positioning with `top: Npx`
- Expands/collapses via `HashSet<String>` of expanded paths (no recursive DOM)

### WASM-Specific Patterns
- `wasm_bindgen` closures must be `'static` — use `Rc<RefCell<_>>` for shared state
- `Closure::wrap` for JS callbacks — store to prevent GC collection
- `JsValue::from_str()` for JSON transfer (faster than `Reflect::set` per-field)
- Avoid `serde_wasm_bindgen` on hot paths — JSON string bridge is 10x faster for large objects
- `web_sys::console::log_1()` for debug only — remove before benchmarking

## Extension Converter Bridge

### Pipeline: Rust → JS → Extension
```
Rust: prost-reflect DynamicMessage
  → dynamic_message_to_json(&msg) → JSON String (snake_case, {sec,nsec} timestamps)
  → JsValue::from_str(&json_string) → pass to JS

JS (inline in manager.rs):
  → typeof === 'string' ? JSON.parse(obj) : obj
  → convertKeysToCamel(parsed)  // snake_case→camelCase, seconds→sec, nanos→nsec
  → converter.converter(finalMsgObj, messageEvent, globalVars, context)
  → Returns: SceneUpdate | FrameTransforms

Rust (post-converter):
  → parse_scene_update_result() → WebGL entities (cubes, lines, triangles)
  → parse_js_frame_transform() → TfTree inserts
```

### Key Insight: Why convertKeysToCamel Exists
Extensions expect **camelCase** keys (designed for protobufjs output). prost-reflect `FieldDescriptor::name()` returns **snake_case** (proto3 canonical). The JS bridge reconciles this mismatch.

## Leptos Patterns

### Signals & Reactivity
```rust
let state = use_app_state();       // AppState from context
let layout = use_layout_state();   // LayoutState from context
let frame_tick = state.frame_tick; // RwSignal<u64> - triggers panel re-renders
```

### View Rules
- Views MUST use owned types: `String`, not `&str`
- Use `.into_any()` for different view types from match/if
- `Memo::new()` return type must implement `PartialEq`
- `NodeRef::<leptos::html::Div>::new()` for DOM element access

### Closures & Ownership
```rust
// Clone BEFORE moving into closure
let input_clone = input.clone();
let closure = Closure::once(move |_| { input_clone.files()... });
input.set_onchange(...); // Original still valid
```

## Layout System
- `LayoutNode` enum: `Panel { id, panel_type, topic }` | `Split { id, direction, ratio, first, second }`
- JSON format: `{ configById, layout, playbackConfig, globalVariables }`
- Persisted to `localStorage` (key: `lichtblick-layouts`)

## Alerts System
- Fires when any topic has Hz > 60 (excluding log schemas)
- Excluded: `rosgraph_msgs/Log`, `rcl_interfaces/msg/Log`, `foxglove.Log`

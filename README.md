# Lichtblick (Rust/WASM)

A complete port of [Lichtblick](https://github.com/lichtblick-suite/lichtblick) web application to Rust, compiled to WebAssembly. Built with **Leptos 0.7.8** and **Trunk 0.21.14**.

Lichtblick is a robotics data visualization tool supporting MCAP files with lazy chunk-based loading, real-time playback, and panel-based layout system.

## Current Status

### Working Features
- **MCAP lazy playback** — instant file open (summary-only), chunks loaded on demand
- **5-tab sidebar** — Panel settings, Topics (with Hz/count), Alerts (>60Hz), Layouts, Extensions
- **Stable topic stats** — Hz and message counts from MCAP Statistics record (never oscillate)
- **Playback controls** — Play/pause, speed (0.1x–3x), seek via progress bar
- **Performance alerts** — Warns when topics exceed 60Hz (matches original Lichtblick)
- **Layout system** — Split panels, create/import/export/load from localStorage
- **Image panel** — JPEG/PNG/H.264 via Blob URLs (browser-decoded)
- **3D panel** — WebGL2 point clouds, TF transforms, coordinate frames, SceneUpdate entities (cubes/lines)
- **Raw Messages panel** — JSON tree view of decoded CDR/ROS1/Protobuf messages
- **Additional panels** — Log, Plot, DataSourceInfo, Diagnostics, StateTransitions, Teleop
- **Extension system** — Install .foxe extensions, converter pipeline for custom message types
- **TF tree** — Full transform tree with SLERP interpolation, frame hierarchy, Display Frame selector
- **Protobuf support** — prost-reflect DynamicMessage decode, FileDescriptorSet schema registration
- **ASAM OSI converter** — Extension converts osi3.SensorView → FrameTransforms + SceneUpdate (vehicles, lanes, signs)

### Performance Characteristics
- **File open**: ~50ms (reads summary only, 132MB MCAP)
- **Playback**: 60fps loop, panels update at 30fps (throttled)
- **50x Faster Deserialization Bridge**: Native Rust `dynamic_message_to_json()` prunes unused nested messages/oneofs (avoiding WASM Stack Overflow) and passes a single JSON string to V8's parsed camelCase bridge, eliminating thousands of JS-WASM boundary crossings per frame.
- **Virtualized JSON Tree**: Flat, scroll-driven virtual list in the Raw Messages panel that keeps DOM elements under 30 nodes (avoiding DOM node explosion from 20,000+ fields).
- **Memory**: 100MB chunk cache cap with LRU eviction
- **Chunk prefetch**: 3s ahead, batch of 2 (keeps main thread responsive)
- **Seek**: Generation-counter invalidation of stale async loads
- **Converter pipeline**: timestamp dedup (skip repeated log_time_ns), failed schema tracking

## Architecture

```
crates/
├── lichtblick-app         # Web UI (Leptos components, player, state)
│   ├── src/player.rs      # MCAP lazy player (most complex file)
│   ├── src/mcap_reader.rs # MCAP format parser
│   ├── src/decoder.rs     # CDR/ROS1/Protobuf message decoders
│   ├── src/extensions/    # Extension system
│   │   ├── manager.rs     # JS bridge, converter registry, protobufjs init
│   │   ├── storage.rs     # IndexedDB persistence
│   │   ├── loader.rs      # .foxe ZIP parser (Store + Deflate)
│   │   └── types.rs       # ExtensionInfo, StoredExtension
│   ├── src/components/    # Sidebar, toolbar, panel layout
│   ├── src/panels/        # Panel implementations
│   │   ├── three_dee_panel.rs  # WebGL2 3D + TF + converter integration
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

### MCAP Player Pipeline

```
File.slice() → FileReader → decompress (LZ4/zstd) → parse messages → chunk_cache
                                                                          ↓
                              panels ← frame_tick ← tick_and_reschedule ← latest_messages
```

### Extension Converter Pipeline

```
MCAP message (binary) → prost-reflect DynamicMessage decode → dynamic_message_to_js() → JsValue
                                                                                            ↓
                    converter(message, messageEvent, globalVariables, context)  ← 4-arg calling convention
                                    ↓                                    ↓
            foxglove.FrameTransforms                        foxglove.SceneUpdate
                    ↓                                              ↓
    parse_js_frame_transform → TfTree              parse_scene_update_result → SCENE_ENTITIES
                    ↓                                              ↓
        Display Frame dropdown                      WebGL2 cubes + lines rendering
```

Key design decisions:
- **Lazy loading**: Only summary is read on open. Chunks are fetched via browser File API.
- **Generation counter**: Every seek increments a counter. Stale async loads are discarded.
- **Collect-then-apply**: Avoids RefCell borrow conflicts in the playback loop.
- **Stable stats**: Topic Hz/count comes from MCAP footer Statistics record, not runtime counting.
- **Rust-native protobuf**: Uses `prost-reflect` for decoding (not protobufjs). DescriptorPool compiled from MCAP schema's FileDescriptorSet.
- **Failed schema tracking**: Permanently broken schemas are recorded and never retried.
- **Timestamp dedup**: Converter pipeline skips messages with same log_time_ns as previously processed.
- **4-arg converter call**: Matches real Lichtblick's `messageProcessing.ts` — first arg is raw decoded message.

### Known Gaps / TODO
- **Protobuf Casing (snake_case vs. camelCase)**: The Rust dynamic message JS serializer outputs `snake_case` keys, but the JS extensions expect `camelCase` keys. We need to map keys to `camelCase` in `dynamic_message_to_js` to make the JS converters receive valid properties.
- **Coordinate conversion**: No Z-up (ROS/OSI) → Y-up (WebGL) rotation applied yet. Cubes/lines may appear at wrong orientation.
- **SceneUpdate deletions**: Not implemented. Entities accumulate but are never deleted.
- **Arrows/spheres/cylinders/triangles/models**: Only cubes and lines primitives are extracted and rendered.
- **Per-vertex line colors**: Only uniform color per line is supported (not per-point colors array).

## Extension System

The extension system supports `.foxe` format packages (ZIP archives) containing:
- `package.json` — Extension metadata (id, name, publisher, contributes)
- `dist/extension.js` — Bundled JavaScript extension code
- `README.md`, `CHANGELOG.md` — Optional documentation

### How Extensions Work
1. **Install**: Drag-and-drop .foxe file → ZIP parsed → stored in IndexedDB
2. **Activate**: JS source executed via `new Function()`. Extension calls `registerMessageConverter`
3. **Registry**: Converter functions stored in `globalThis.__extensionConverters[fromSchemaName]`
4. **Runtime**: For each message matching a registered schema:
   - Binary data → `prost-reflect::DynamicMessage::decode()` → `dynamic_message_to_js()` → JsValue
   - JsValue passed to converter as first arg: `converter(msg, event, globals, context)`
   - FrameTransforms output → parsed → inserted into TfTree
   - SceneUpdate output → cubes/lines extracted → stored in SCENE_ENTITIES → rendered in WebGL2

### Protobuf Decoding (Rust-native)
- **prost-reflect 0.14.7**: Decodes protobuf using compiled DescriptorPool (from MCAP schema FileDescriptorSet)
- **Custom JS conversion** (`dynamic_message_to_js`): snake_case fields, all defaults emitted, longs→f64
- **Timestamp handling**: `google.protobuf.Timestamp` converted to `{sec, nsec}` (not `{seconds, nanos}`)
- **Pool cache**: `PROTO_POOLS` thread-local HashMap<schema_name, DescriptorPool>

### Installed Extension
- **ASAM OSI Converter** (`lichtblick.asam-osi-converter-1.0.0`):
  - Converts `osi3.SensorView` → `foxglove.FrameTransforms` (ego_vehicle_bb_center, ego_vehicle_rear_axle, Global)
  - Converts `osi3.SensorView` → `foxglove.SceneUpdate` (vehicles/cubes, lanes/lines, traffic signs/lights)
  - Converts `osi3.GroundTruth` → `foxglove.FrameTransforms` + `foxglove.SceneUpdate`

## Prerequisites

1. **Rust toolchain** (1.95+):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **WASM target**:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. **Trunk** (WASM build tool & dev server):
   ```bash
   cargo install trunk --version 0.21.14
   ```

## Development

### Run in development mode (with hot reload):

```bash
cd /path/to/ailbtorust && bash dev.sh
```

This starts the dev server on port 8081 with file watching.

### Build check (fast, no serve):

```bash
cargo check --target wasm32-unknown-unknown
```

### Build for production:

```bash
trunk build --release
```

Output will be in the `dist/` directory.

## Testing with MCAP Files

1. Start dev server: `bash dev.sh`
2. Open browser at `http://localhost:8081`
3. Drag-and-drop an MCAP file onto the app
4. For OSI extensions: use an MCAP with `osi3.SensorView` topic (e.g., SanDiego scenario file)
5. Open 3D panel → Display Frame dropdown should show frames from TF tree + extension converters

### Run tests:

```bash
# Run all unit tests (native)
cargo test

# Run tests for a specific crate
cargo test -p lichtblick-messages
cargo test -p lichtblick-mcap
cargo test -p lichtblick-core
```

### E2E Tests (Playwright):

```bash
# Install Playwright
cd e2e && npm install && npx playwright install chromium

# Run against Rust app (port 8081)
npx playwright test --project=rust

# Run against Node.js reference (port 8080)
npx playwright test --project=nodejs

# Run visual comparison (both must be running)
npx playwright test --project=compare

# View HTML report
npx playwright show-report reports/html
```

### Check compilation:

```bash
cargo check --target wasm32-unknown-unknown
```

### Lint:

```bash
cargo clippy --target wasm32-unknown-unknown -- -D warnings
```

## Multi-Agent Development Workflow

This project uses a structured agent pipeline for development. See [AGENTS.md](AGENTS.md) for full details.

| Agent | Role | Tools |
|-------|------|-------|
| **Code** | Implement features & fixes | `cargo build`, `wasm-bindgen` |
| **Review** | Best practices, performance, security | `cargo clippy`, manual review |
| **QA** | Automated testing | Playwright E2E, `cargo fmt`, integration |
| **Compare** | Visual diff Rust vs Node.js | Playwright screenshots, console debug |

**Flow:** Code → Review → QA/Compare → (loop if issues) → Done

## Project Structure

| Crate | Purpose | Key Types |
|-------|---------|-----------|
| `lichtblick-core` | Foundation types | `Time`, `Topic`, `MessageEvent`, `PlayerState`, `LayoutData` |
| `lichtblick-messages` | Message path query language | `MessagePath`, `parse_message_path()`, `evaluate_message_path()` |
| `lichtblick-mcap` | MCAP file I/O | `McapReader`, `McapIterableSource` |
| `lichtblick-players` | Playback engines | `IterablePlayer`, `FoxgloveWebSocketPlayer` |
| `lichtblick-datasources` | Source factories | `McapLocalDataSourceFactory`, `FoxgloveWebSocketDataSourceFactory` |
| `lichtblick-panels` | Panel definitions | `PanelCatalog`, `PlotConfig`, `ThreeDeeConfig`, `ImageConfig` |
| `lichtblick-theme` | Visual theming | `dark_theme()`, `light_theme()` |
| `lichtblick-app` | Web UI (Leptos) | `App`, `Workspace`, `PlaybackControls` |

## Deployment

### Static hosting (Vercel, Netlify, S3):

```bash
trunk build --release --public-url /
# Deploy contents of dist/
```

### Docker:

```dockerfile
FROM nginx:alpine
COPY dist/ /usr/share/nginx/html/
```

## Reference

- [Lichtblick original (TypeScript)](https://github.com/lichtblick-suite/lichtblick)
- [MCAP format specification](https://mcap.dev/spec)
- [Leptos documentation](https://leptos.dev/)
- [Trunk documentation](https://trunkrs.dev/)

## License

MPL-2.0

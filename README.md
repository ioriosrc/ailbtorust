# Lichtblick (Rust/WASM)

A complete port of [Lichtblick](https://github.com/lichtblick-suite/lichtblick) web application to Rust, compiled to WebAssembly. Built with **Leptos 0.7.8** and **Trunk 0.21.14**.

Lichtblick is a robotics data visualization tool supporting MCAP files with lazy chunk-based loading, real-time playback, and panel-based layout system.

## Current Status

### Working Features
- **MCAP lazy playback** — instant file open (summary-only), chunks loaded on demand
- **4-tab sidebar** — Panel settings, Topics (with Hz/count), Alerts (>60Hz), Layouts
- **Stable topic stats** — Hz and message counts from MCAP Statistics record (never oscillate)
- **Playback controls** — Play/pause, speed (0.1x–3x), seek via progress bar
- **Performance alerts** — Warns when topics exceed 60Hz (matches original Lichtblick)
- **Layout system** — Split panels, create/import/export/load from localStorage
- **Image panel** — JPEG/PNG/H.264 via Blob URLs (browser-decoded)
- **3D panel** — WebGL point clouds, TF transforms, markers
- **Raw Messages panel** — JSON tree view of decoded CDR/ROS1 messages
- **Additional panels** — Log, Plot, DataSourceInfo, Diagnostics, StateTransitions, Teleop

### Performance Characteristics
- File open: ~50ms (reads summary only, 132MB MCAP)
- Playback: 60fps loop, panels update at 30fps (throttled)
- Memory: 100MB chunk cache cap with LRU eviction
- Chunk prefetch: 3s ahead, batch of 2 (keeps main thread responsive)
- Seek: Generation-counter invalidation of stale async loads

## Architecture

```
crates/
├── lichtblick-app         # Web UI (Leptos components, player, state)
│   ├── src/player.rs      # MCAP lazy player (most complex file)
│   ├── src/mcap_reader.rs # MCAP format parser
│   ├── src/decoder.rs     # CDR/ROS1 message decoders
│   ├── src/components/    # Sidebar, toolbar, panel layout
│   ├── src/panels/        # Panel implementations
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

Key design decisions:
- **Lazy loading**: Only summary is read on open. Chunks are fetched via browser File API.
- **Generation counter**: Every seek increments a counter. Stale async loads are discarded.
- **Collect-then-apply**: Avoids RefCell borrow conflicts in the playback loop.
- **Stable stats**: Topic Hz/count comes from MCAP footer Statistics record, not runtime counting.

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
# Must run from project root directory!
trunk serve --port 8081 --open
```

This will:
- Compile the Rust code to WASM
- Bundle and serve the web app
- Open your browser at `http://localhost:8081`
- Hot-reload on source changes

### Build for production:

```bash
trunk build --release
```

Output will be in the `dist/` directory.

### Run tests:

```bash
# Run all unit tests (native)
cargo test

# Run tests for a specific crate
cargo test -p lichtblick-messages
cargo test -p lichtblick-mcap
cargo test -p lichtblick-core
```

### Check compilation:

```bash
cargo check --target wasm32-unknown-unknown
```

### Lint:

```bash
cargo clippy --target wasm32-unknown-unknown -- -D warnings
```

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

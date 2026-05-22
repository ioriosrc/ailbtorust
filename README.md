# Lichtblick (Rust/WASM)

A complete port of [Lichtblick](https://github.com/lichtblick-suite/lichtblick) web application to Rust, compiled to WebAssembly.

Lichtblick is a robotics data visualization tool supporting MCAP files, ROS bag playback, and live WebSocket connections.

## Architecture

```
crates/
├── lichtblick-core        # Core types: Time, Topic, MessageEvent, Layout, Player types
├── lichtblick-messages    # Message path parsing and evaluation
├── lichtblick-mcap        # MCAP file reading and schema parsing
├── lichtblick-players     # Player implementations (Iterable, WebSocket)
├── lichtblick-datasources # Data source factories
├── lichtblick-panels      # Panel configs and logic (Plot, 3D, Image, etc.)
├── lichtblick-theme       # Theme system (dark/light)
└── lichtblick-app         # Web application (Leptos + WASM)
```

## Prerequisites

1. **Rust toolchain** (1.75+):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **WASM target**:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. **Trunk** (WASM build tool & dev server):
   ```bash
   cargo install trunk
   ```

4. **wasm-bindgen-cli** (optional, for manual builds):
   ```bash
   cargo install wasm-bindgen-cli
   ```

## Development

### Run in development mode (with hot reload):

```bash
trunk serve --open
```

This will:
- Compile the Rust code to WASM
- Bundle and serve the web app
- Open your browser at `http://localhost:8080`
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

# Run WASM tests (requires wasm-pack)
# cargo install wasm-pack
# wasm-pack test --headless --chrome crates/lichtblick-app
```

### Check compilation without building:

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

## Features

- **File playback**: Open MCAP files and play back recorded data
- **Live connections**: Connect to ROS systems via Foxglove WebSocket
- **24 panel types**: 3D, Plot, Image, Logs, Diagnostics, Map, etc.
- **Mosaic layout**: Split panels horizontally/vertically
- **Message path queries**: Filter and navigate nested message data
- **Dark/Light themes**: Full theme support
- **Global variables**: Share state between panels

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

## License

MPL-2.0

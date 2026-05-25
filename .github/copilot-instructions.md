# Copilot Agent Instructions for Lichtblick (Rust/WASM)

Welcome to the Lichtblick Rust port! You are assisting in developing a complex web application for robotics data visualization built entirely in Rust, compiled to WebAssembly (WASM), and utilizing the Leptos framework for UI.

## Context & Architecture
- **Language**: Rust (Edition 2021) targeted to `wasm32-unknown-unknown`.
- **UI Framework**: Leptos (Client-side rendering, `csr` feature).
- **Core Domain**: Handling robotics data (MCAP files, Foxglove WebSockets).
- **Sub-crates**: 
  - `lichtblick-app` (Leptos UI & WebGL)
  - `lichtblick-core` (Data models, schemas)
  - `lichtblick-mcap` (MCAP parsing)
  - `lichtblick-panels` (Visual panels: Plot, 3D, Image, Logs)
  - `lichtblick-messages` (Message path evaluation)

## Core Directives

1. **Leptos Idioms**: 
   - Always use Leptos reactive primitives (`Signal`, `RwSignal`, `create_effect`, `create_memo`).
   - For high-frequency updates (e.g., streaming ROS data at 100Hz+), **avoid** binding reactive signals directly to deep DOM elements if it causes full re-renders. Use `create_render_effect` and mutate `NodeRef` elements directly when performance is critical (like Canvas updates).
   - Use `move ||` closures for reactive reads in the view macro.

2. **WASM and FFI**:
   - Use `web_sys` and `js_sys` for browser APIs.
   - When passing large arrays to WebGL, prefer `js_sys::Float32Array::view(&rust_slice)` to avoid unnecessary copies across the WASM boundary. Ensure the underlying Rust slice lives long enough during the view!
   - Handle `.unwrap()` and `.expect()` carefully. In Web UI, a panic crashes the WASM module. Bubble errors up or handle them gracefully using Leptos `<ErrorBoundary>`.

3. **Performance First**:
   - Point cloud decoding and 3D rendering are bottleneck areas. Minimize memory allocations in `update` loops.
   - Reuse buffers in WebGL. Do not recreate `WebGlBuffer` every frame.

4. **Data Deserialization**:
   - The app reads MCAP and ROS2 CDR data. Be extremely careful with byte offsets and slices to prevent out-of-bounds panics.
   - Use crates like `bytemuck` or `zerocopy` when applicable, but remember that WASM is little-endian.

5. **Aesthetics & UI**:
   - Follow the design system in `lichtblick-theme`.
   - The UI should feel premium, fast, and responsive.

6. **Quality Assurance (Lint & Tests)**:
   - **Always write tests** for new components, data parsing logic, and utilities.
   - Run `cargo test` to ensure unit tests pass. When testing UI or WASM-specific logic, use `wasm-bindgen-test` and run `wasm-pack test --headless --chrome <crate_name>`.
   - Before suggesting any final code changes, ensure the code complies with strict linting rules. Run `cargo clippy --target wasm32-unknown-unknown -- -D warnings` and fix any issues.
   - Never remove existing tests without providing an explicit, valid reason. Code must remain covered and functional.

## When Asked to Create a Panel
- Implement the configuration struct in `lichtblick-panels`.
- Create the component in `lichtblick-app/src/panels/`.
- Register the panel in the `PanelCatalog`.
- Handle layout via the Mosaic grid system.

## When Asked to Debug WebGL
- Verify that `viewport` is updated dynamically on resize.
- Verify `requestAnimationFrame` is properly driving the render loop, but only when new data is available or camera moves (to save CPU).

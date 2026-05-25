# Copilot Agent Skills for Lichtblick (Rust/WASM)

This file defines the skills an AI agent or Copilot should employ when operating within the `ailbtorust` workspace.

## 1. WebGL & Shader Optimization
**Skill**: Enhancing 3D rendering performance.
**Context**: The application decodes point clouds (ROS2 CDR) and renders them in WebGL2 (`lichtblick-app/src/panels/three_dee_panel.rs`).
**Tasks**:
- Implement batched rendering for multiple PointClouds or Markers.
- Add support for indexing and instanced rendering (e.g., drawing thousands of Arrows or Cubes for `MarkerArray` efficiently).
- Safely update `Float32Array` buffers without reallocating WASM memory.
- Implement camera controls (Orbit, FPS, Fly).

## 2. Leptos Component Refactoring
**Skill**: Writing clean, performant Leptos components.
**Context**: Large panels often mix UI logic, layout, and heavy data processing.
**Tasks**:
- Split monolithic components (like `three_dee_panel.rs`) into smaller, modular components (e.g., `<ThreeDeeCanvas>`, `<ThreeDeeToolbar>`, `<LayerTree>`).
- Implement `create_memo` to prevent unnecessary recalculations of large datasets.
- Ensure proper DOM node cleanup when components are unmounted (e.g., closing WebSockets or clearing large Rust-side caches).

## 3. MCAP and Foxglove Data Parsing
**Skill**: Binary data deserialization and schema matching.
**Context**: `lichtblick-mcap` reads binary records and `lichtblick-messages` evaluates paths.
**Tasks**:
- Implement zero-copy deserialization where possible to extract timestamps, floats, and strings.
- Map FlatBuffers or Protobuf payloads dynamically by parsing the schemas contained within the MCAP `Schema` records.

## 4. UI/UX Polishing
**Skill**: Advanced CSS and WASM DOM manipulation.
**Context**: Building a premium robotics visualization tool.
**Tasks**:
- Implement seamless drag-and-drop for the panel mosaic layout.
- Build theme-aware generic components (Buttons, Selects, Dropdowns) in `lichtblick-app/src/components/`.
- Maintain crisp typography and dark mode support.

## 5. Error Handling & Safety
**Skill**: Idiomatic Rust error handling in WASM.
**Context**: WASM panics result in a dead UI. 
**Tasks**:
- Refactor `unwrap()` calls into `Result` bubbling with `thiserror`.
- Surface errors into toaster/notification components rather than console logs.

## 6. Testing & Quality Assurance
**Skill**: Rigorous testing of WASM and Rust logic.
**Context**: Complex binary parsing and reactive state management require strict testing to prevent regressions.
**Tasks**:
- Write robust unit tests for all pure Rust modules (like parsers and data structures).
- Run and fix `cargo clippy --target wasm32-unknown-unknown -- -D warnings` on every change.
- Never regress test coverage; if you alter an existing component's behavior, update or add the corresponding tests.

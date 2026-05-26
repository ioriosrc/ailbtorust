# Copilot Agent Skills for Lichtblick (Rust/WASM)

This file defines the skills an AI agent or Copilot should employ when operating within the `ailbtorust` workspace.

## 1. Extension System & Converter Pipeline
**Skill**: Managing extension lifecycle, protobufjs integration, and message converter pipeline.
**Context**: Extensions are `.foxe` ZIP files that provide JS converters for transforming binary protobuf messages into visualization data (e.g., FrameTransforms for 3D panel). The bridge lives in `manager.rs` inline_js.
**Tasks**:
- Add new `registerXxx` APIs to the ExtensionContext mock (e.g., `registerTopicAlias`, `registerCameraModel`).
- Debug protobufjs loading issues (CDN, descriptor.json fetch, async race conditions).
- Add new converter output types beyond FrameTransforms (e.g., Markers, PointClouds).
- Handle new extension formats or schema encodings (FlatBuffers, JSON Schema).
- Fix schema registration failures (FileDescriptorSet decode, nested message type resolution).
**Key knowledge**:
- Browser can't use CommonJS — must fetch descriptor.json, NOT load ext/descriptor/index.js
- `js_is_protobuf_ready()` must be true before any schema registration
- Failed schemas tracked in thread-local HashSet — avoids retry spam
- Timestamp dedup prevents redundant converter calls on same frame data

## 2. TF Tree & Coordinate Frames
**Skill**: Transform tree management, frame interpolation, 3D coordinate systems.
**Context**: `tf_tree.rs` implements a full transform tree with SLERP quaternion interpolation, used by the 3D panel for rendering objects in correct coordinate frames.
**Tasks**:
- Implement `TfTree::lookup()` chain traversal for multi-hop transforms.
- Add URDF-based static transforms (robot model → joint frames).
- Optimize transform lookup caching for high-frequency queries.
- Implement frame graph visualization panel.
- Handle circular reference detection in frame hierarchy.
**Key knowledge**:
- StampedTransform: parent_frame, child_frame, timestamp_ns, translation (Vec3d), rotation (Quaternion)
- TransformBuffer: ring buffer per parent→child pair for time-based lookup
- Sources: native TF/CDR, extension converters, PointCloud2 header.frame_id
- Auto-select display frame from preferred list when first frame appears

## 3. WebGL & Shader Optimization
**Skill**: Enhancing 3D rendering performance.
**Context**: The application decodes point clouds (ROS2 CDR) and renders them in WebGL2 (`lichtblick-app/src/panels/three_dee_panel.rs`).
**Tasks**:
- Implement batched rendering for multiple PointClouds or Markers.
- Add support for indexing and instanced rendering (e.g., drawing thousands of Arrows or Cubes for `MarkerArray` efficiently).
- Safely update `Float32Array` buffers without reallocating WASM memory.
- Implement camera controls (Orbit, FPS, Fly).
- Render frame axes visualization for TF tree frames.

## 4. Leptos Component Refactoring
**Skill**: Writing clean, performant Leptos components.
**Context**: Large panels often mix UI logic, layout, and heavy data processing.
**Tasks**:
- Split monolithic components (like `three_dee_panel.rs`) into smaller, modular components (e.g., `<ThreeDeeCanvas>`, `<ThreeDeeToolbar>`, `<LayerTree>`).
- Implement `create_memo` to prevent unnecessary recalculations of large datasets.
- Ensure proper DOM node cleanup when components are unmounted (e.g., closing WebSockets or clearing large Rust-side caches).

## 5. MCAP and Binary Data Parsing
**Skill**: Binary data deserialization and schema matching.
**Context**: `lichtblick-mcap` reads binary records and `lichtblick-messages` evaluates paths.
**Tasks**:
- Implement zero-copy deserialization where possible to extract timestamps, floats, and strings.
- Map FlatBuffers or Protobuf payloads dynamically by parsing the schemas contained within the MCAP `Schema` records.
- Handle MCAP attachments and metadata records.
- Support additional compression algorithms beyond LZ4/zstd.
**Key knowledge**:
- MCAP stores protobuf schemas as binary FileDescriptorSet (NOT .proto text)
- Schema data passed to `js_register_proto_schema()` for protobufjs decode on JS side
- Native Rust protobuf decode: uses field numbers from schema (NOT always sequential)

## 6. WASM-JS Interop & Browser APIs
**Skill**: Efficient communication between Rust/WASM and JavaScript.
**Context**: The app uses `wasm_bindgen` inline_js for extension bridge, IndexedDB, FileReader, WebGL, and protobufjs.
**Tasks**:
- Minimize JS↔WASM boundary crossings (batch operations where possible).
- Handle async JS operations (Promises) from Rust via `JsFuture`.
- Manage `Closure::once` / `Closure::wrap` lifecycle (`.forget()` for persistent callbacks).
- Debug TypedArray views into WASM memory (invalidated on memory growth).
**Key knowledge**:
- `spawn_local` for async operations from synchronous Leptos effects
- `JsFuture::from(promise).await` for awaiting JS Promises
- `serde-wasm-bindgen` for structured data transfer (Vec<u8>, HashMap, etc.)
- Thread-locals for per-component state (SCENE, TF_STATE, REGISTERED_SCHEMAS)

## 7. UI/UX Polishing
**Skill**: Advanced CSS and WASM DOM manipulation.
**Context**: Building a premium robotics visualization tool.
**Tasks**:
- Implement seamless drag-and-drop for the panel mosaic layout.
- Build theme-aware generic components (Buttons, Selects, Dropdowns) in `lichtblick-app/src/components/`.
- Maintain crisp typography and dark mode support.
- Dynamic dropdowns that update reactively (e.g., Display Frame selector from TfTree::frames).

## 8. Error Handling & Safety
**Skill**: Idiomatic Rust error handling in WASM.
**Context**: WASM panics result in a dead UI. 
**Tasks**:
- Refactor `unwrap()` calls into `Result` bubbling with `thiserror`.
- Surface errors into toaster/notification components rather than console logs.
- Gracefully handle extension failures (JS errors during activation, converter crashes).

## 9. Testing & Quality Assurance
**Skill**: Rigorous testing of WASM and Rust logic.
**Context**: Complex binary parsing and reactive state management require strict testing to prevent regressions.
**Tasks**:
- Write robust unit tests for all pure Rust modules (like parsers and data structures).
- Run and fix `cargo clippy --target wasm32-unknown-unknown -- -D warnings` on every change.
- Never regress test coverage; if you alter an existing component's behavior, update or add the corresponding tests.
- Test extension activation with mock .foxe packages.
- Verify TfTree transforms with known-good reference data.

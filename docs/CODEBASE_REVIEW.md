# Codebase Review: Lichtblick (Rust/WASM)

This document contains a high-level review of the `ailbtorust` repository, outlining areas for improvement regarding performance, security, architecture, and feature parity with the original TypeScript-based Lichtblick/Foxglove Studio.

## 1. Architecture & Best Practices
- **Monolithic Components**: Some UI components, notably `three_dee_panel.rs` (~900 lines), are doing too much. They mix Leptos UI rendering, WebGL context initialization, shader compilation, and binary data decoding. 
  - *Suggestion*: Extract WebGL rendering logic into a dedicated `renderer` module, and binary decoding into a `decoder` module.
- **Error Handling**: The codebase relies heavily on `.unwrap()` when interfacing with WebGL and JS APIs. In a WASM context, a panic crashes the entire module, resulting in a blank screen.
  - *Suggestion*: Use `Result` combined with Leptos `<ErrorBoundary>` to gracefully catch and display errors.
- **State Management**: Using `RwSignal` and `use_context` is correct for Leptos. Ensure that global state (like active data sources) doesn't trigger unnecessary re-renders in deeply nested components.
- **WASM Interop**: Frequent small calls between JS and WASM have overhead. 
  - *Suggestion*: Batch operations where possible, especially for DOM manipulation and WebGL buffer updates.

## 2. Performance Optimizations
- **Zero-Copy Deserialization**: The current approach to parsing PointCloud2 (in `three_dee_panel.rs`) allocates new `Vec<f32>` arrays every frame.
  - *Suggestion*: When data comes from MCAP/WebSocket, try to cast the byte slice directly into an `js_sys::Float32Array::view()` and pass it to WebGL without copying. Ensure alignment and endianness match (WASM is little-endian).
- **Leptos Reactivity**: High-frequency ROS messages (e.g., 100Hz IMU data or images) shouldn't be bound directly to reactive text nodes. It will destroy DOM performance.
  - *Suggestion*: For rapidly changing text (like raw message viewers), use `requestAnimationFrame` to sample the latest value at 60Hz and update the `NodeRef` directly (e.g., `element.set_text_content(...)`).
- **WebGL Buffers**: Do not recreate `WebGlBuffer` instances for static data (like the grid or axes) or frequently updated data. Pre-allocate large buffers and use `bufferSubData` for dynamic point clouds to minimize VRAM fragmentation.

## 3. Security Considerations
- **Binary Parsing**: Reading offsets directly from untrusted data (like `offset` fields in PointField) can lead to out-of-bounds panics.
  - *Suggestion*: Thoroughly validate all size and offset parameters against the actual byte slice length before indexing into it.
- **WebSocket Connections**: When connecting to Foxglove WebSocket servers, ensure `wss://` is supported and implement token/auth headers if required by secure networks.
- **XSS via Message Payloads**: If string payloads from ROS topics are rendered directly into HTML without escaping (e.g., `inner_html`), it's a vector for XSS.
  - *Suggestion*: Always use Leptos text nodes or sanitize HTML.

## 4. Feature Status (vs. Original Lichtblick/Foxglove)

### Implemented
1. **Extension System**: `.foxe` package loading, JS activation, message converter registry, protobufjs integration. Not a full plugin API (no custom panels in JS yet) but converter pipeline works end-to-end.
2. **TF Tree & Coordinate Frames**: Full transform hierarchy with SLERP interpolation, time-based lookup, Display Frame selector. Sources: TF CDR messages + extension converters.
3. **Layout Serialization**: Save, export, import, and share layout JSON (localStorage + file download). Format matches Lichtblick original.
4. **Dynamic Protobuf Deserialization**: FileDescriptorSet schemas decoded at runtime via protobufjs (CDN). Supports any protobuf message type (OSI, custom messages).

### Partially Implemented
1. **Advanced 3D Rendering**:
   - Has: PointCloud2, grid, axes, basic frame visualization
   - Missing: URDF robot models, mesh markers (STL/DAE), text markers, arrow markers, complex scene graph
2. **Dynamic Type Deserialization**:
   - Has: CDR (ROS2), ROS1 serialization, Protobuf (binary + schema-based with field numbers)
   - Missing: FlatBuffers support, JSON Schema support

### Not Yet Implemented
1. **Custom JS Panels**: Extensions can register panels but rendering them in WASM requires embedded JS runtime or iframe approach.
2. **Foxglove Data Platform Integration**: Cloud streaming via API.
3. **WebRTC Support**: Video streaming from ROS cameras.
4. **WebSocket Live Data**: Real-time connection to running robots (trait exists, implementation pending).
5. **Message Path Evaluation for Plot**: Arbitrary nested field access like `msg.pose.position.x` for Plot panel.

## Summary
The `ailbtorust` project has a solid foundation using Leptos with a working extension system, TF tree, and protobuf converter pipeline. The primary focus for future development should be on **expanding the 3D renderer** (URDF, mesh markers, full scene graph), **implementing message path evaluation** (for Plot panel field access like `msg.pose.position.x`), and **adding WebSocket live data** to support real-time robot connections. The extension system architecture (JS bridge via inline_js + protobufjs CDN) is proven and can be extended for additional converter output types beyond FrameTransforms.

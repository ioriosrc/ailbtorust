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

## 4. Missing Features (Compared to Original Lichtblick/Foxglove)

While the Rust port is highly ambitious, it currently lacks several features present in the original TypeScript version:

1. **Extension System API**: 
   - *Original*: Allows users to write custom panels in TS/React.
   - *Rust Port*: Difficult to replicate in WASM. A solution could involve Rhai/Lua scripting or compiling user panels as WASI components.
2. **Advanced 3D Rendering**:
   - *Original*: Supports complex TF (transform) trees, URDF (robot models), and diverse markers (meshes, arrows, text).
   - *Rust Port*: Appears to currently support basic PointCloud2, Grid, and Axes. Needs a full Scenegraph implementation to support complex coordinate frame transformations and mesh rendering.
3. **Layout Serialization & Workspaces**:
   - *Original*: Users can save, export, and share layout JSON files.
   - *Rust Port*: Needs a system to serialize the state of all panels and the mosaic layout into JSON and restore it from LocalStorage or a file.
4. **Dynamic Type Deserialization (ROS1 / ROS2 / Protobuf / FlatBuffers)**:
   - *Original*: Extracts schemas dynamically and constructs JSON-like representations for Plot and Raw Data panels.
   - *Rust Port*: Rust's static typing makes dynamic schema parsing harder. Needs a robust AST/Value representation (like `serde_json::Value` but for binary formats) to plot arbitrary nested message fields (e.g., `msg.pose.position.x`).
5. **Foxglove Data Platform Integration**:
   - Streaming directly from Foxglove cloud via their API.
6. **WebRTC Support**:
   - Video streaming from ROS cameras using WebRTC (common in modern remote operation setups).

## Summary
The `ailbtorust` project has an excellent foundation using Leptos. The primary focus for future development should be on **stabilizing binary parsing** (preventing panics), **expanding the 3D renderer** (Scenegraph, URDF, TF), and **implementing dynamic message evaluation** to bring the Plot and Raw Data panels to feature parity with the original suite.

---
description: "Rust/WASM performance optimization knowledge for the Lichtblick codebase. Covers profiling techniques, dynamic protobuf JSON serialization, WASM-to-JS bridge optimization, Leptos reactive patterns, WebGL2 rendering pipeline details, and virtualization."
---

# Performance Skill (Rust/WASM/WebGL2)

## Profiling Workflow

### Browser DevTools
1. **Performance tab**: Record during playback. Identify long WASM execution blocks (>16.6ms for 60fps, >33.3ms for 30fps).
2. **Memory tab**: Watch WASM linear memory growth. Ensure it plateaus and does not leak under chunk loading.
3. **Lighthouse / Performance Monitor**: Watch DOM node counts (should remain low due to virtualization) and frame rates.
4. **Console**: Check timings using temporary Rust `web_sys::window().unwrap().performance().unwrap().now()` measurements.

### Key Metrics
- **Frame budget**: 33.3ms (for 30fps throttled panel updates) or 16.6ms (for 60fps UI/progress updates).
- **GC pressure**: Frequent JS garbage collection indicates excessive object creation at the WASM-JS bridge.
- **WASM Memory**: Keep memory consumption stable by reusing buffers (like `String` and `Vec`) in hot paths.

## Common Bottlenecks & Solutions

### 1. WASM-JS Boundary Crossings (The Bridge)
- **Symptom**: Stuttering and low FPS during playback of high-frequency protobuf messages.
- **Cause**: Using `Reflect::set` to copy individual fields across the WASM-JS boundary. For a 650KB protobuf message, this crosses the boundary thousands of times, taking 30-50ms per frame.
- **Fix**: The **JSON Bridge Pattern**. Serialize the dynamic message to a JSON string in Rust, pass the single string to JS, and call `JSON.parse` in V8. This runs in ~2ms (a **50x speedup**).
- **Key Insight**: Map keys to `camelCase` and format timestamps as `{sec, nsec}` in JS to preserve compatibility with extensions.

### 2. Custom Protobuf JSON Serializer (WASM Stack Overflow)
- **Symptom**: Settings panel shows "(no frames)", 3D panel displays nothing, and browser console displays a WebAssembly Stack Overflow panic.
- **Cause**: In `prost-reflect`, `msg.get_field` returns default empty messages for unset message fields. Recursively serializing them causes infinite recursion on cyclic or deeply nested schemas (like ASAM OSI).
- **Fix**: Check `msg.has_field(&field_desc)` for all singular message and oneof fields, and skip them if they are unset.

```rust
// Custom JSON serialization logic:
let is_msg = matches!(field_desc.kind(), prost_reflect::Kind::Message(_));
let is_map = if let prost_reflect::Kind::Message(ref m_desc) = field_desc.kind() {
    m_desc.is_map_entry()
} else {
    false
};
let is_repeated = field_desc.cardinality() == prost_reflect::Cardinality::Repeated;
let is_singular_msg = is_msg && !is_map && !is_repeated;
let is_oneof = field_desc.containing_oneof().is_some();

if is_singular_msg || is_oneof {
    if !msg.has_field(&field_desc) {
        continue; // Skip unset fields to avoid stack overflow!
    }
}
```

### 3. DOM Node Explosion (Raw Messages Panel)
- **Symptom**: Browser freezes or runs out of memory when displaying large (20,000+ fields) JSON messages.
- **Cause**: Nested recursive Leptos components creating thousands of DOM elements.
- **Fix**: **Flat Virtual List**. Recursively flatten the JSON object into a flat `Vec<FlatRow>` in Rust. Calculate the scroll viewport index and render only the visible rows (~20 elements) positioned absolutely inside a scrollable container.

### 4. Redundant WebGL2 Render Passes (3D Panel)
- **Symptom**: GPU lag or driver crashes on high-frequency topics.
- **Cause**: Invoking `gl.render()` multiple times per tick (e.g. once per message, point cloud, and camera move event).
- **Fix**: **Single Render + Dirty Flag**. Set `scene_dirty = true` when entities change, and run a single `s.render()` at the end of the tick cycle.

## Memory Caching Crate Rules
- Use `Rc<Vec<u8>>` to share message buffers between the MCAP player and the UI panels without copying bytes.
- Limit the player cache size (e.g. 100MB chunk cache cap) and evict old chunks using an LRU policy.
- Clear static/thread-local caches (like TF frame matrices) at the start of each playback tick.

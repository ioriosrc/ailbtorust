---
applyTo: "**/*.rs"
---

# Performance Guidelines (Rust/WASM)

These rules apply to ALL Rust code targeting `wasm32-unknown-unknown`. Violations cause frame drops, GC pressure on the JS side, or WASM memory bloat.

## Allocation Rules

- **Never allocate inside `requestAnimationFrame` callbacks or `tick()` functions**
- Reuse `Vec` and `String` buffers — clear and refill, don't create new
- Use `Vec::with_capacity(n)` when the count is known or estimable
- Prefer `&[u8]` slices over cloning byte vectors
- Use `Rc<Vec<u8>>` for shared message data — zero-copy across player/panels
- Stack-allocate small arrays: `[f32; 16]` for transforms, `[f32; 4]` for quaternions

## WASM Memory Patterns

### Avoid Serde on Hot Paths
```rust
// SLOW: serde_json::to_string allocates, formats, then JS must re-parse
let json = serde_json::to_string(&msg)?;

// FAST: write directly to a pre-allocated String buffer
let mut buf = String::with_capacity(4096);
write_message_json(&msg, &mut buf);
```

### Minimize JS Boundary Crossings
```rust
// SLOW: One Reflect::set per field (N crossings for N fields)
for field in msg.fields() {
    Reflect::set(&obj, &key.into(), &val.into())?;
}

// FAST: Single crossing with JSON string
let json_str = dynamic_message_to_json(&msg);
JsValue::from_str(&json_str)  // One crossing, JS does JSON.parse
```

### RefCell Discipline
```rust
// WRONG: nested borrow panics at runtime
let cache = self.chunk_cache.borrow();
for msg in cache.iter() {
    self.latest_messages.borrow_mut().insert(msg.topic.clone(), msg.clone());
}

// CORRECT: collect-then-apply
let updates: Vec<_> = self.chunk_cache.borrow().iter().map(|m| (m.topic.clone(), m.clone())).collect();
for (k, v) in updates {
    self.latest_messages.borrow_mut().insert(k, v);
}
```

## Rendering Performance

### Frame Budget: 16.6ms (60fps) / 33.3ms (30fps throttled)

- **Single render per tick** — never call `render()` inside message processing loops
- **Dirty flag pattern** — skip WebGL render if `scene_dirty == false`
- **TF cache per frame** — compute each frame_id's 4×4 matrix once, cache in HashMap
- **Batch WebGL calls** — group by program/material, minimize `gl.bindBuffer()`/`gl.drawArrays()`
- **Clear cache at frame start** — `tf_cache.clear()` once, not per-entity

### Protobuf Decode Budget
- OSI messages ~650KB → target <5ms decode time
- Decode only **1 message per tick** for full-snapshot schemas (OSI SensorView)
- **WASM Stack Overflow Avoidance**: Prune unset singular messages and inactive oneofs by checking `msg.has_field(&field_desc)`. Never recurse into unset message fields, otherwise the compiler/browser WASM engine will panic with a stack overflow on cyclic/deep schemas (e.g. ASAM OSI).
- Use `prost-reflect` DynamicMessage (no generated code needed at compile time)
- Custom JSON writer avoids serde overhead (field iteration + direct write)

## DOM / Virtual List Performance

### Problem: Large messages = 20,000+ fields = DOM explosion
### Solution: Flat virtual list

```rust
// Flatten recursive JSON into Vec<FlatRow>
// Only render rows visible in scroll viewport (~20 rows)
// Use absolute positioning: top = idx * ROW_HEIGHT
// Expand/collapse via HashSet<path> — re-flatten on toggle
```

Key metrics:
- Row height: 22px (fixed for calculation)
- Overscan: 4 rows above/below viewport
- No recursive components — single flat Vec iteration
- `PartialEq` on `FlatRow` for `Memo` reactivity

## Chunk Loading Strategy

- **Prefetch 3s ahead** of current playback time
- **Max 2 concurrent chunk loads** — keep main thread responsive
- **100MB cache cap** — LRU eviction of oldest chunks
- **Generation counter** for seek invalidation
- **Never scan all chunks every frame** — time-range early-out

## Profiling Workflow

### Browser DevTools
1. **Performance tab**: Record during playback, look for long WASM frames (>16ms)
2. **Memory tab**: Watch for WASM linear memory growth (should plateau at ~200MB)
3. **Network tab**: Verify chunk loads are sequential, not thundering herd
4. **Console**: Check for `[PERF]` markers if using `web_sys::console::time()`

### Rust-side Timing
```rust
// Quick timing (remove before commit)
let start = web_sys::window().unwrap().performance().unwrap().now();
// ... work ...
let elapsed = web_sys::window().unwrap().performance().unwrap().now() - start;
web_sys::console::log_1(&format!("[PERF] decode: {:.1}ms", elapsed).into());
```

## Anti-Patterns (Do NOT)

| Anti-Pattern | Why | Fix |
|---|---|---|
| `serde_wasm_bindgen::to_value()` on large messages | Allocates JS objects per field | JSON string bridge |
| `Reflect::set` in loops | N JS boundary crossings | Build JSON string in Rust |
| Recursive Leptos components for trees | Stack overflow + DOM explosion | Flat virtual list |
| `clone()` on `Vec<u8>` message data | Copies 650KB per message | `Rc<Vec<u8>>` sharing |
| Multiple `render()` calls per frame | Redundant WebGL flushes | Single render + dirty flag |
| `HashMap::new()` inside render | Allocation every frame | Persistent cache, `.clear()` |

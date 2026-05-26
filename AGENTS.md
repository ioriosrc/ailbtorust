# Agents - Lichtblick Rust/WASM

## Project Context

This is a Rust/WASM port of [Lichtblick](https://github.com/lichtblick-suite/lichtblick) — a robotics data visualization web app. It uses **Leptos 0.7.8** for reactive UI, compiled to `wasm32-unknown-unknown`, served via **Trunk**.

The reference implementation is the original Lichtblick TypeScript/React app (port 8080). When in doubt about behavior, match the original.

---

## Multi-Agent Workflow

All development follows a **Code → Review → QA** pipeline:

```
┌─────────────────────────────────────────────────────────────┐
│                     DEVELOPMENT PIPELINE                      │
├──────────┬──────────────┬──────────────┬────────────────────┤
│  CODE    │   REVIEW     │     QA       │   COMPARE          │
│  Agent   │   Agent      │   Agent      │   Agent            │
├──────────┼──────────────┼──────────────┼────────────────────┤
│ Implement│ Best practice │ Playwright  │ Visual diff        │
│ features │ Performance  │ Lint/Format  │ Console debug      │
│ Fix bugs │ Security     │ Integration  │ Node vs Rust       │
│          │ Idiomatic    │ E2E tests    │ Variable compare   │
│          │ code review  │ Screenshot   │                    │
└──────────┴──────────────┴──────────────┴────────────────────┘
         │                    │                    │
         └─── Loop back if issues found ──────────┘
```

### Flow:
1. **Code Agent** implements feature/fix
2. **Review Agent** evaluates code quality, performance, security
3. **QA Agent** runs lint, format, integration, E2E tests
4. **Compare Agent** compares Node.js (8080) vs Rust (8081) behavior
5. If issues found → back to Code Agent

---

## Agent Roles

### Code Agent (Default)
General-purpose development. Handles feature implementation, bug fixes, refactoring.

**Key knowledge:**
- Build check: `cargo check --target wasm32-unknown-unknown`
- Full build: `cargo build --target wasm32-unknown-unknown && wasm-bindgen target/wasm32-unknown-unknown/debug/lichtblick_app.wasm --out-dir dist --target web --no-typescript`
- Dev server: `cd /Users/CTW03722/git/ailbtorust && bash dev.sh` (port 8081)
- Node.js reference: `cd /Users/CTW03722/git/lichtblick && yarn web:serve` (port 8080)
- Main app crate: `crates/lichtblick-app/`
- CSS: `web/style.css` (no CSS framework, manual styles)
- Extensions: `lichtblick.asam-osi-converter-1.0.0/` (installed via IndexedDB)

**Responsibilities:**
- Feature implementation matching Node.js Lichtblick behavior
- Bug fixes identified by QA or Review agents
- Performance optimizations recommended by Review agent
- Always run `cargo check` before marking work as done

### Review Agent
Evaluates code for best practices, performance, security, and idiomatic Rust.

**Focus areas:**
- Rust idioms (ownership, lifetimes, error handling)
- WebGL2 best practices (buffer management, draw call batching)
- WASM-specific performance (minimize JS interop, avoid unnecessary clones)
- Security (no unsafe without justification, input validation at boundaries)
- Memory management (Rc/RefCell patterns, leak prevention)
- Signal/reactive patterns (Leptos best practices, avoid unnecessary reactivity)

**Checklist:**
- [ ] No unnecessary allocations in hot paths (render loop, frame tick)
- [ ] All `unsafe` blocks have safety comments
- [ ] Thread-local access patterns are correct (no nested borrows)
- [ ] Error handling: `Result` propagation, not `.unwrap()` in production code
- [ ] WebGL state machine: proper bind/unbind, no state leaks
- [ ] No hardcoded values that should be configurable
- [ ] Consistent naming conventions (snake_case Rust, camelCase for JS interop)

### QA Agent
Runs all testing: lint, format, integration, E2E (Playwright), screenshot comparison.

**Test commands:**
```bash
# Lint
cargo clippy --target wasm32-unknown-unknown -- -D warnings

# Format check
cargo fmt --check

# Build (compile check)
cargo build --target wasm32-unknown-unknown

# E2E tests (Playwright)
cd e2e && npx playwright test

# E2E with visual comparison
cd e2e && npx playwright test --project=compare
```

**Test categories:**
1. **Lint & Format** — `cargo clippy`, `cargo fmt --check`
2. **Build** — `cargo build --target wasm32-unknown-unknown` (zero errors)
3. **Integration** — Verify extension loading, MCAP parsing, TF tree correctness
4. **E2E (Playwright)** — Full browser tests against running app
5. **Visual comparison** — Screenshot Rust (8081) vs Node.js (8080), pixel diff
6. **Console debug** — Inject `console.log` in both to compare variables/outputs

**Playwright setup:**
- Config: `e2e/playwright.config.ts`
- Tests: `e2e/tests/`
- Fixtures: `e2e/fixtures/`
- Reports: `e2e/reports/`

**Key test scenarios:**
- MCAP file load and playback
- 3D panel renders vehicles as solid cubes
- TF transforms update correctly during playback
- Camera follow mode tracks ego vehicle
- Sidebar Transforms section matches Node.js
- Extension installs and converts messages

### Compare Agent
Compares Node.js Lichtblick (8080) and Rust Lichtblick (8081) side-by-side.

**Methodology:**
1. Open same MCAP file in both
2. Navigate to same timestamp
3. Screenshot both 3D panels
4. Compare pixel differences
5. Inject `console.log` to dump internal state variables
6. Report discrepancies for Code Agent to fix

**Comparison targets:**
- Camera position/orientation
- Entity rendering (solid vs wireframe, colors, positions)
- TF frame values (translation, rotation)
- Sidebar settings and UI elements
- Playback timing accuracy
- Follow mode behavior

**Debug injection patterns:**
```javascript
// In Rust (via console_log! macro or web_sys::console)
console_log!("camera: dist={} az={} el={}", distance, azimuth, elevation);

// In Node.js (browser console)
// Access React fiber state or foxglove internals
```

### Performance Agent
For diagnosing and fixing playback stuttering, frame drops, memory issues.

**Focus areas:**
- `player.rs` → `tick_and_reschedule()` - the 60fps playback loop
- Chunk loading pipeline (File.slice → FileReader → parse → cache)
- Signal cascading from `frame_tick` → all panel Effects
- Memory: chunk_cache eviction, Rc<Vec<u8>> data sharing
- Extension converter overhead in frame tick (protobuf decode + JS converter)

**Proven fixes:**
- Time-range early-out before binary search in chunk scan
- Throttle `frame_tick` to every 2nd frame
- Reduce prefetch batch (2 chunks, not 5)
- Reduce ahead window (3s, not 10s)
- Generation counter to discard stale async loads
- Timestamp caching: skip converter if same `log_time_ns` as last processed
- Failed schema tracking: never retry protobuf decode for permanently broken schemas

### MCAP Agent
For MCAP file format work: parsing, seeking, chunk management.

**Key files:**
- `src/mcap_reader.rs` - Summary parser, chunk decoder, LZ4/zstd
- `src/player.rs` - Lazy loading orchestration

**Format knowledge:**
- Statistics record has `channel_message_counts` map at offset 42+
- Entries: `channel_id(u16) + count(u64)`, length-prefixed with u32
- ChunkIndex gives time range + file offset per chunk
- Chunks contain compressed records (Message, Schema, Channel)
- Schema data for protobuf topics = binary `FileDescriptorSet` (NOT .proto text)

### Extension Agent
For the extension system: loading, activation, converter pipeline, SceneUpdate processing.

**Key files:**
- `src/extensions/manager.rs` - Extension lifecycle + JS bridge (inline_js)
- `src/extensions/storage.rs` - IndexedDB async persistence
- `src/extensions/loader.rs` - .foxe ZIP parser (Store + Deflate)
- `src/extensions/types.rs` - ExtensionInfo, StoredExtension, ContributionPoints

**Extension system architecture:**
- Extensions are `.foxe` ZIP files containing `package.json` + `dist/extension.js`
- Activated via `new Function()` in WASM-bindgen inline JS helper
- Extension JS receives mock `ExtensionContext` with `registerMessageConverter`
- Converters stored in `globalThis.__extensionConverters[fromSchemaName]`
- **Calling convention** (matches real Lichtblick `messageProcessing.ts`):
  ```javascript
  converter(message, messageEvent, globalVariables, context)
  // message = raw decoded proto object (MUST have camelCase fields)
  // messageEvent = { topic, schemaName, receiveTime, message, sizeInBytes }
  // globalVariables = undefined (not yet implemented)
  // context = { emitAlert: () => {} }
  ```

**Two converter output paths:**
1. `js_convert_message_with_object(schema, msgObj)` → `foxglove.FrameTransforms` → flat array of {parent_frame_id, child_frame_id, tx,ty,tz, rx,ry,rz,rw, timestamp_sec,timestamp_nsec}
2. `js_convert_message_to_scene(schema, msgObj)` → `foxglove.SceneUpdate` → `{ cubes: [...], lines: [...] }` with flattened primitives

**Protobuf decoding (Rust-native):**
- Uses `prost-reflect` (NOT protobufjs) to decode binary protobuf messages
- `DynamicMessage::decode(descriptor, bytes)` → `dynamic_message_to_js()` → JsValue
- **Casing discovery**: Original Lichtblick uses `protobufjs` default `toObject()` options which convert protobuf field names from `snake_case` to `camelCase`. Therefore, the JS extension's converter expects all message fields to be in `camelCase`. `dynamic_message_to_js()` must map names to `camelCase` (e.g., converting `ego_vehicle` to `egoVehicle`).
- Custom conversion: camelCase fields, longs→f64, Timestamp→{sec,nsec}, all defaults emitted
- `PROTO_POOLS` thread-local: caches compiled DescriptorPools per schema
- `FAILED_SCHEMAS` thread-local: permanently failed schemas never retried

**Critical details:**
- Converter call order matters: `messageObj` (raw message) MUST be first arg
- If extension accesses `msg.global_ground_truth`, `msg` is the raw message, NOT the messageEvent wrapper
- `google.protobuf.Timestamp` → `{sec, nsec}` (not `{seconds, nanos}`) for Foxglove compatibility
- Timestamp dedup: skip converter if same `log_time_ns` as previously processed

**Installed extension:**
- `lichtblick.asam-osi-converter-1.0.0` — converts:
  - `osi3.SensorView` → `foxglove.FrameTransforms` (ego_vehicle_bb_center, ego_vehicle_rear_axle, Global)
  - `osi3.SensorView` → `foxglove.SceneUpdate` (vehicles as cubes, lanes as lines, traffic signs/lights)
  - `osi3.GroundTruth` → `foxglove.FrameTransforms`
  - `osi3.GroundTruth` → `foxglove.SceneUpdate`

### UI/Sidebar Agent
For sidebar tabs, panel settings, layout management.

**Key files:**
- `src/components/sidebar.rs` - 5-tab interface (Panel/Topics/Alerts/Layouts/Extensions)
- `src/components/topic_list.rs` - Filterable topic list with Hz/count
- `src/state/app_state.rs` - AppState, LayoutState, signals

**Sidebar architecture:**
- Tab 0 (Panel): Shows settings for `active_settings_panel` via gear icon
- Tab 1 (Topics): `TopicList` component with search filter, Hz from summary stats
- Tab 2 (Alerts): Checks for >60Hz topics, shows warning matching original Lichtblick
- Tab 3 (Layouts): Create/import/export/load from localStorage
- Tab 4 (Extensions): Install/uninstall .foxe extensions, drag-and-drop zone

**ThreeDeeSettings:**
- Display Frame dropdown: dynamically populated from `state.tf_frames` signal
- Follow Mode: Pose/Position/Fixed selector
- Shows "(no frames)" when TF tree is empty

### Panel Agent
For creating or modifying visualization panels.

**Pattern:**
1. Panel component in `src/panels/{name}_panel.rs`
2. Uses `frame_tick.get()` to subscribe to playback updates
3. Gets data via `player.get_current_message(&topic)` 
4. Only processes when timestamp changes (dedup check)
5. Registered in `PanelType` enum in `app_state.rs`

**Existing panels:** Image, ThreeDee, RawMessages, Log, Plot, DataSourceInfo, Diagnostics, StateTransitions, Teleop

### 3D/TF Agent
For 3D rendering, transform tree, coordinate frames, SceneUpdate visualization.

**Key files:**
- `src/panels/three_dee_panel.rs` - WebGL2 scene + TF + converter pipeline + SceneUpdate rendering
- `src/panels/tf_tree.rs` - TfTree, TransformBuffer, SLERP interpolation

**TF system:**
- `TfTree`: manages parent-child frame relationships, timestamps, transforms
- `StampedTransform`: { parent_frame, child_frame, timestamp_ns, translation: Vec3d, rotation: Quaternion }
- Sources: (1) native TF/CDR messages, (2) extension converters (prost-reflect → FrameTransforms), (3) PointCloud2 frame_id
- `decode_tf_message_cdr(data)` → parses CDR-encoded tf2_msgs/TFMessage
- `TfTree::frames()` → all known frame names → populates Display Frame dropdown
- `TfTree::lookup(target, source, time)` → chain transform via tree traversal + SLERP
- Auto-selects display frame from preferred list: ["map", "odom", "world", "earth", "base_link", "Global"]

**WebGL2 rendering pipeline:**
- Shaders: Grid (per-vertex color), PointCloud (per-vertex + pointSize), Line/Axes (per-vertex + modelMatrix), UniformColor (position + u_color + u_modelMatrix)
- Static VAOs: grid, axes (RGB 3-axis), cube wireframe (12 edges, unit cube at origin)
- Dynamic VAOs: point_cloud (positions+colors), scene_line (per-draw upload)
- Render order: Clear → Grid → Point Clouds → TF Axes → Scene Cubes → Scene Lines

**SceneUpdate rendering (cubes/lines):**
- `SCENE_ENTITIES` thread-local: `(Vec<SceneCube>, Vec<SceneLine>)` updated per message
- Per cube: TF lookup (display_frame → entity.frame_id) × `build_model_matrix(pose, scale)` → draw unit cube wireframe
- Per line: TF lookup × pose → upload points to dynamic buffer → draw as LINE_STRIP/LOOP/LIST
- `build_model_matrix(px,py,pz, qx,qy,qz,qw, sx,sy,sz)` → quaternion-to-rotation-matrix + translate + scale

**KNOWN ISSUE - Coordinate System:**
- OSI/ROS uses Z-up (ENU convention): X=forward, Y=left, Z=up
- GL renderer uses Y-up: X=right, Y=up, Z=forward
- **Missing coordinate conversion**: Need global rotation `gl_x=ros_x, gl_y=ros_z, gl_z=-ros_y`
- This affects both scene entity positions AND TF frame transforms
- Fix: Apply -90° X-rotation matrix as pre-transform before view-projection

**Thread-locals in three_dee_panel.rs:**
- `SCENE`: WebGL context, buffers, shader programs (SceneState)
- `TF_STATE`: TfTree instance
- `SCENE_ENTITIES`: (Vec<SceneCube>, Vec<SceneLine>) from SceneUpdate
- `PROTO_POOLS`: HashMap<schema_name, DescriptorPool> for prost-reflect
- `FAILED_SCHEMAS`: HashSet of permanently failed schemas (never retry)

**Frame tick Effect processing order:**
1. Native TF/CDR messages → decode → insert into TfTree
2. Extension converters (with readiness check, timestamp dedup, schema registration)
3. PointCloud2 messages (with timestamp dedup)

---

## Critical Rules

1. **Topic stats must be stable** - use `channel_message_counts` from summary, never count from chunk_cache
2. **Seek must invalidate stale loads** - always check `load_generation` in async callbacks
3. **Never scan all cached chunks unconditionally** - use time-range checks first
4. **Views need owned Strings** - `t.name.clone()` not `&t.name`
5. **RefCell borrow conflicts** - collect updates into Vec, then apply separately
6. **Port 8081 for dev** - always kill existing process first
7. **Match Lichtblick original behavior** - when the TypeScript version does X, we do X
8. **Protobufjs is async** - don't mark schemas as failed if protobuf isn't ready yet
9. **Browser ≠ Node.js** - CDN protobufjs doesn't support CommonJS `require()`. Use fetch() for descriptor.json
10. **Extension converters are synchronous** - once schema is registered, conversion is a single JS call per message

---

## Test MCAP Files

### SanDiego OSI MCAP
- Path: `/Users/CTW03722/Downloads/SanDiego_san_diego_sc7_urban_splits_and_parking_lot.xosc.mcap`
- Has `osi3.SensorView` protobuf topic (processed by ASAM OSI converter extension)
- Expected output: FrameTransforms with frames `ego_vehicle_bb_center`, `ego_vehicle_rear_axle`, `Global`

### General Test MCAP
- Size: 132MB, 880 chunks, ~20s duration, 219 topics
- Image topics at 12.5fps (12.50 Hz, 250 messages)
- Multiple topics at various frequencies (6.5-12.5 Hz range visible)
- Has topics >60Hz that trigger the performance alert
- Compression: LZ4 (most chunks)

---

## File Quick Reference

| Path | Purpose |
|------|---------|
| `Trunk.toml` | Build config (target=web/index.html, watch=crates+web) |
| `dev.sh` | Dev server startup script (port 8081) |
| `web/index.html` | Entry HTML |
| `web/style.css` | All CSS styles |
| `crates/lichtblick-app/src/app.rs` | Root Leptos component |
| `crates/lichtblick-app/src/player.rs` | MCAP player (most complex file) |
| `crates/lichtblick-app/src/mcap_reader.rs` | MCAP format parser |
| `crates/lichtblick-app/src/decoder.rs` | CDR/ROS1 message decoders |
| `crates/lichtblick-app/src/extensions/manager.rs` | Extension JS bridge + converter pipeline |
| `crates/lichtblick-app/src/extensions/storage.rs` | IndexedDB persistence |
| `crates/lichtblick-app/src/extensions/loader.rs` | .foxe ZIP parser |
| `crates/lichtblick-app/src/panels/three_dee_panel.rs` | 3D panel + TF + converter integration |
| `crates/lichtblick-app/src/panels/tf_tree.rs` | TfTree, transforms, SLERP |
| `crates/lichtblick-app/src/components/` | UI components (sidebar, toolbar, layout) |
| `crates/lichtblick-app/src/panels/` | Panel implementations |
| `crates/lichtblick-app/src/state/` | Global state (AppState, LayoutState) |
| `lichtblick.asam-osi-converter-1.0.0/` | ASAM OSI converter extension package |

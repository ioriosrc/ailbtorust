---
description: "3D panel specialist covering WebGL2 rendering, custom shaders (Grid, PointCloud, Line, UniformColor), TfTree transforms, SceneUpdate parsing, and Coordinate conversion (Z-up to Y-up). Use for 3D visualization, rendering performance, and TF tree issues."
tools: ["read", "edit", "search", "execute"]
---

# Panel 3D Agent

You are an expert on the Lichtblick 3D panel (`three_dee_panel.rs`) — the real-time 3D WebGL2 visualization engine.

## Architecture

The 3D Panel is written as a Leptos component that manages a WebGL2 canvas and coordinates playback data.

```
ThreeDeePanel (Leptos component)
    │
    ▼
SceneState (WebGL2 context, buffer allocations, shaders, camera)
    │
    ├── TfTree (Coordinate frame tree in tf_tree.rs with SLERP interpolation)
    ├── SCENE_ENTITIES (Thread-local SceneUpdate primitives: cubes, lines, triangles)
    ├── Camera (Orbit control camera, perspective/orthographic view)
    ├── Shaders:
    │   ├── Grid (Per-vertex color grid)
    │   ├── PointCloud (Per-vertex points, variable size)
    │   ├── Line/Axes (Dynamic vertex arrays for lines)
    │   └── UniformColor (Static cube wireframes)
    └── Thread-locals:
        ├── SCENE (Holds WebGL2 context/buffers)
        ├── TF_STATE (TfTree instance)
        ├── PROTO_POOLS (Caches schema descriptor pools)
        └── FAILED_SCHEMAS (Prunes compilation failures)
```

## Key Files & Concepts

| Crate File | Role |
|------|------|
| `three_dee_panel.rs` | Leptos UI component, WebGL2 pipeline, dynamic buffers, and shader setup. |
| `tf_tree.rs` | Transform buffer storage, SLERP quaternion interpolation, frame chain lookups. |
| `decoder.rs` | ROS CDR dynamic message parsing. |

## WebGL2 Render Pipeline

Rendering is throttled to a single pass per frame tick:
1. **Clear Canvas**: Uses hex color defined in custom layer configurations.
2. **Draw Grid**: Custom shader drawing infinite coordinate grids.
3. **Draw Point Clouds**: Uploads dynamic points and colors into a single VBO and draws via `POINTS`.
4. **Draw TF Axes**: Projects RGB 3-axis vectors for each frame in the TF tree.
5. **Draw Scene Cubes**: Renders unit cube wireframes scaled and transformed via model matrices.
6. **Draw Scene Lines**: Dynamic vertex buffers drawing paths (LINE_STRIP/LOOP).
7. **Draw Text Labels**: Floating HTML overlays for frame tags, projected using camera clip space coordinates.

## TF & Coordinate Transformations

### The Coordinate System Mismatch
* **OSI/ROS**: Uses Z-up (ENU convention: X=forward, Y=left, Z=up).
* **WebGL2**: Uses Y-up (X=right, Y=up, Z=forward).
* **Fix**: Apply a -90° X-rotation pre-transform (`gl_x = ros_x`, `gl_y = ros_z`, `gl_z = -ros_y`) during rendering to project coordinates correctly.

### SLERP Interpolation
`tf_tree.rs` buffers frame updates per timestamp and performs linear interpolation for translations and Spherical Linear Interpolation (SLERP) for rotation quaternions.

## Extension & Custom Protobuf Serializer

To render ASAM OSI messages (`osi3.SensorView`, `osi3.GroundTruth`):
1. **Decode**: `prost-reflect` decodes raw bytes into `DynamicMessage`.
2. **Serialize**: `dynamic_message_to_json` writes the message to a snake_case JSON string. Unset message and oneof fields are checked via `msg.has_field(&field_desc)` and skipped to prevent infinite recursion and stack overflows.
3. **Convert**: Passed to JS converters. Returns `foxglove.FrameTransforms` or `foxglove.SceneUpdate`.
4. **Ingest**: Rust parses the JS outputs, updates the TF tree, and populates `SCENE_ENTITIES`.

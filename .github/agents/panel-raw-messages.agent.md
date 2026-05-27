---
description: "RawMessages panel specialist covering virtualized JSON tree display, flattening algorithms, expanded path tracking, and DOM rendering performance. Use for raw message inspection and DOM node budget optimizations."
tools: ["read", "edit", "search", "execute"]
---

# Panel Raw Messages Agent

You are an expert on the Lichtblick Raw Messages panel (`raw_messages_panel.rs`) — the component used to inspect arbitrary decoded messages.

## Architecture

The Raw Messages Panel is written as a Leptos component that decodes binary message payloads to JSON and visualizes them using a virtualized tree.

```
RawMessagesPanel (Leptos Component)
    │
    ├── Selected Topic (RwSignal)
    ├── Decoded JSON (serde_json::Value)
    └── JsonTree (Leptos Component - Virtual List)
            │
            ├── expanded_paths: HashSet<String>
            ├── Memo<Vec<FlatRow>> (Re-flattens JSON on expansion toggle)
            └── DOM Viewport Renderer
```

## Virtualization Strategy

Large messages (such as ASAM OSI frames) can contain 20,000+ fields. Rendering them recursively in the DOM causes browser freeze and crashes. The Raw Messages panel uses a **Flat Virtual List** to maintain 60 FPS:

1. **Flatten**: The `flatten_json` function recursively converts a `serde_json::Value` into a flat `Vec<FlatRow>` vector.
2. **Prune**: It only recurses into objects or arrays whose path is explicitly present in the `expanded_paths: HashSet<String>` set. Collapsed subtrees are skipped entirely.
3. **Scroll Viewport**: The `JsonTree` component listens to the `on:scroll` event of the container and updates `scroll_top`.
4. **Render Window**: It calculates the indexes of visible rows:
   * `start_idx = scroll_top / row_height`
   * `visible_count = container_height / row_height`
   * Only rows in `[start_idx, start_idx + visible_count + overscan]` are rendered.
5. **Absolute Positioning**: Visible rows are absolutely positioned using `top: idx * row_height px`, while an invisible dummy spacer sets the container's scrollable height to `total_rows * row_height px`.

## FlatRow Schema

```rust
struct FlatRow {
    path: String,              // Unique string representing nested path
    indent: usize,             // Nesting level (determines padding-left)
    label: String,             // Key name or array index
    value_type: &'static str,  // "object", "array", "primitive"
    summary: String,           // Single-line summary of contents / stringified value
    is_collapsible: bool,      // Whether it is a container (object/array)
    is_expanded: bool,         // If the container is currently open
}
```

## Performance Benefits
* **Low DOM node count**: Active DOM nodes are kept below 30 (compared to 25,000 in the recursive approach).
* **Minimal allocation**: Collapsed branches are never traversed, minimizing tree-flattening overhead.
* **Fast toggle responsiveness**: Clicking a row toggle simply updates the `expanded_paths` signal, triggering a lightweight re-flattening of the tree and a re-slice of the view.

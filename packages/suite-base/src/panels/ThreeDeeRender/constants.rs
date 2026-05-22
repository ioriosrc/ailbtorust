```rust
use std::collections::VecDeque;

pub static PANEL_STYLE: CSSProperties = css! {
    width: "100%",
    height: "100%",
    display: "flex",
    position: "relative",
};

pub const MAX_TRANSFORM_MESSAGES: usize = 10_000;

pub const DEFAULT_FOLLOW_MODE: &str = "follow-pose";

pub const HOVER_PICK_THROTTLE_MS: u64 = 100;
```
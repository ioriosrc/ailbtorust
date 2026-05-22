```rust
use tss_rs::css;

fn main() {
    css!({
        vertical_bar: {
            position: "absolute",
            top: "0%",
            bottom: "100%",
            left: "0px",
            width: "1px",
            margin_left: "-1px",
            display: "block",
            pointer_events: "none",
        },
        playback_bar: {
            background_color: "#aaa",
        },
    });
}
```
```rust
use styled_components::{styled, css};

fn DiffSpan(props: &PropsDiffSpan) -> styled::Element {
    styled("span")
        .with_css(css!({
            display: "block";
            padding: "10px";
            border-radius: 5px;
            background-color: "#f0f0f0"; // Example color
            font-size: 14px; // Example font size
        }))
        .children(props.children)
}
```
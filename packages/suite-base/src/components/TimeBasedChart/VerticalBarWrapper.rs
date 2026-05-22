```rust
use styled-components::css;

fn VerticalBarWrapperProps {
    scales: Option<RpcScales>,
    x_value: Option<f64>,
}

impl VerticalBarWrapperProps {
    fn new(
        scales: Option<RpcScales>,
        x_value: Option<f64>,
    ) -> Self {
        Self { scales, x_value }
    }
}

fn vertical_bar_wrapper(props: VerticalBarWrapperProps) -> styled-components::HtmlElement {
    styled::div(css! {
        position: absolute;
        top: 0;
        bottom: 0;
        left: 0;
        right: 0;
        pointer-events: none;
        will-change: transform;
        visibility: hidden;
    }) {

        styled::div(props.children)
    }
}
```
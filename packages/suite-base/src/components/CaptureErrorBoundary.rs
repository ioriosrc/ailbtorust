```rust
use std::error::Error;

type Props = {
    on_error: Box<dyn Fn(&dyn Error)>,
};

type State = {
    had_error: bool,
};

#[derive(Debug)]
struct CaptureErrorBoundary {
    props: Props,
}

impl CapturaErrorBoundary {
    pub fn new(props: Props) -> Self {
        CaptureErrorBoundary { props }
    }

    fn component_did_catch(&mut self, error: Box<dyn Error>) {
        self.props.on_error(error.as_ref());
        self.state.had_error = true;
    }

    fn render(&self) -> String {
        if self.state.had_error {
            format!("<div>Error boundary</div>")
        } else {
            format!("{:#?}", self.props.children)
        }
    }
}
```
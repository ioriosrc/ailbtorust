```rust
use storybook::prelude::*;

#[derive(Story)]
pub struct IncompatibleLayoutVersionAlert;

#[story]
fn default() -> impl Component<IncompatibleLayoutVersionAlert> {}

#[story]
fn desktop(args: impl Into<Option<bool>>) -> impl Component<IncompatibleLayoutVersionAlert> {
    args.into().unwrap_or(false)
}
```
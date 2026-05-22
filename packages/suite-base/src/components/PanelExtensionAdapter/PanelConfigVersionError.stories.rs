```rust
use storybook::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let story = Story::new::<PanelConfigVersionError>("components/PanelConfigVersionError")
        .arg_default(|| PanelConfigVersionError {});

    story.add_render(|_| <PanelConfigVersionError />);

    Ok(())
}
```
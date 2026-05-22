```rust
use storybook::prelude::*;

const COLOR_SCHEME_LIGHT = "light";
const COLOR_SCHEME_DARK = "dark";

fn wrapper(story: fn(): JSX.Element) -> JSX.Element {
    MockCurrentLayoutProvider()
        .with_workspace_context(|ctx| {
            ctx.set_dialogs({
                data_source: Some({
                    active_data_source: None,
                    item: "start",
                    open: true,
                }),
                preferences: Some({
                    initial_tab: None,
                    open: false,
                }),
            });
        })
        .then(story)
}

export default define_components! {
    DataSourceDialog: fn() -> JSX.Element;
}

define stories! {
    DefaultLight: wrapper(|_| <DataSourceDialog />);
    DefaultDark: wrapper(|_| <DataSourceDialog />);
}
```
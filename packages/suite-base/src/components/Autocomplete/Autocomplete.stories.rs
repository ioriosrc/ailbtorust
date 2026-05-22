```rust
use async_test::fixture;
use storybook::prelude::*;
use storybook_rust::{Autocomplete, Stack};

#[derive(Default)]
struct Filters {
    items: Vec<String>,
    has_error: bool,
    filter_text: String,
    value: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stories = fixture! {
        FilteringToO;
        FilteringToOLight;
        UncontrolledValue;
        UncontrolledValueLight;
        SortWhenFilteringFalse;
        ManyItems;
        LongPathInPopup;
    };

    for story in stories {
        story.run(|args| async move {
            let canvas_element = await story.render();
            let input = within(canvas_element);
            fireEvent.click(input);
        });
    }

    Ok(())
}
```
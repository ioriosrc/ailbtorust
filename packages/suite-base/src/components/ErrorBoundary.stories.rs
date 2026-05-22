```rust
use storytaker::{StoryObj, StoryFn};

fn Broken(depth: u32) -> impl FnOnce() {
    move || {
        if depth > 20 {
            panic!("Hello!");
        } else {
            Broken(depth + 1);
        }
    }
}

pub fn Default(storybook: &mut StoryFn) {
    storybook.render(|| {
        DndProvider::new(HTML5Backend).with_children(|children| {
            ErrorBoundary::new(children)
        })
    });
}

pub fn ShowingDetails(storybook: &mut StoryFn) {
    storybook.render(|| {
        DndProvider::new(HTML5Backend).with_children(|children| {
            ErrorBoundary::new_with_custom_options(children, |_, options| {
                options.show_error_details(true);
                options.hide_error_source_locations(true);
            })
        })
    });
}
```
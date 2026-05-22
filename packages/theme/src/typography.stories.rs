```rust
use std::cell::{RefCell, RefMut};

use mui::{
    box::*,
    components::Typography as MuiTypography,
    styled::*,
};

#[derive(Debug)]
struct Wrapper {
    children: RefCell<String>,
}

impl Wrapper {
    fn new() -> Self {
        Self {
            children: RefCell::new("".to_string()),
        }
    }

    fn set_children(&self, content: &str) {
        self.children.borrow_mut().push_str(content);
    }
}

#[derive(Debug)]
struct StoryObj {}

fn main() {
    let mut wrapper = Wrapper::new();

    wrapper.set_children("h1. Heading\n");

    // Rest of the code remains the same
}
```
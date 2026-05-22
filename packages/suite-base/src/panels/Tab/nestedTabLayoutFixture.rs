```rust
use std::collections::HashMap;

struct Fixture {
    topics: Vec<String>,
    datatypes: HashMap<String, String>,
    frame: HashMap<String, String>,
    layout: Layout,
    saved_props: HashMap<String, SavedProps>,
}

struct Layout {
    direction: String,
    first: Option<LayoutPart>,
    second: Option<LayoutPart>,
    split_percentage: i32,
}

struct LayoutPart {
    first: Option<Box<dyn Any>>,
    second: Option<Box<dyn Any>>,
    direction: String,
    split_percentage: i32,
}

struct SavedProps {
    active_tab_idx: usize,
    tabs: Vec<Tab>,
}

struct Tab {
    title: String,
    layout: Option<Layout>,
}
```

Note: The provided Rust code is an abstract representation of the TypeScript/React `Fixture` object and does not include actual data structures or functions. In a real-world scenario, these would be fleshed out with more detailed structures and functions to handle the logic of rendering nested tabs in a graphical user interface or application.
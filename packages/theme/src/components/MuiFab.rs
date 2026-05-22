```rust
use std::borrow::{Borrow, Cow};

pub struct MuiFab {
    color: String,
}

impl MuiFab {
    pub fn new(color: impl Borrow<str>) -> Self {
        MuiFab { color: color.borrow().to_string() }
    }

    pub fn set_color(&mut self, color: impl Borrow<str>) {
        self.color = color.borrow().to_string();
    }
}

fn main() {
    let mut fab = MuiFab::new("inherit");
    println!("Initial color: {}", fab.color);

    fab.set_color("blue");
    println!("Updated color: {}", fab.color);
}
```
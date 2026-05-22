```rust
use mui::{
  components::{ IconButton, IconButtonProps },
  forward_ref,
};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct HoverableIconButton {
    pub icon: Rc<impl Fn() -> String>,
    pub active_icon: Option<Rc<impl Fn() -> String>>,
    pub color: Option<String>,
    pub active_color: Option<String>,
    pub children: Option<Rc<impl Fn() -> String>>,
    pub icon_position: String,
}

fn main() {
    // Implement the HoverableIconButton as needed
}
```
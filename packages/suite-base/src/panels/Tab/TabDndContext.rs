```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct DraggingTabItem {
    pub type_: &'static str,
    pub tab_index: usize,
    pub panel_id: String,
}

pub struct DraggingTabPanelState {
    item: Option<DraggingTabItem>,
    is_over: bool,
}

#[derive(Clone, Default)]
pub struct TabActions {
    add_tab: Rc<dyn Fn()>,
    remove_tab: Rc<dyn Fn(usize)>,
    select_tab: Rc<dyn Fn(usize)>,
    set_tab_title: Rc<dyn Fn(usize, &str)>,
}

impl Default for TabDndContext {
    fn default() -> Self {
        TabDndContext {
            prevent_tab_drop: false,
        }
    }
}
```
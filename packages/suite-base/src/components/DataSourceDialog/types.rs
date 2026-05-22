```rust
use std::rc::Rc;

#[derive(Debug)]
pub struct DataSourceOptionProps {
    text: String,
    secondary_text: String,
    icon: Rc<dyn std::any::Any>,
    onClick: Box<dyn Fn()>,
    href: Option<String>,
    target: &'static str,
}

#[derive(Debug)]
pub struct SidebarItem {
    id: String,
    title: String,
    text: Rc<dyn std::any::Any>,
    actions: Option<Rc<dyn std::any::Any>>,
}
```
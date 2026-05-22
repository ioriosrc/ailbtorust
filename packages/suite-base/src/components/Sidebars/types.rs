```rust
use std::collections::HashMap;

pub struct SidebarItemBadge {
    pub color: Option<&'static str>,
    pub count: usize,
}

pub struct SidebarItem {
    pub badge: Option<SidebarItemBadge>,
    pub component: Option<String>, // Replace with actual React component type
    pub iconName: Option<&'static str>,
    pub title: String,
    pub url: Option<String>,
}

pub type NewSidebarProps<K> = HashMap<K, SidebarItem>;

pub type LayoutNode = &'static str;

pub struct SidebarProps<OldLeftKey, LeftKey, RightKey> {
    pub items: HashMap<OldLeftKey, SidebarItem>;
    pub bottom_items: HashMap<OldLeftKey, SidebarItem>;
    pub selected_key: Option<OldLeftKey>,
    pub on_select_key: fn(OldLeftKey) -> ();

    pub left_items: HashMap<LeftKey, SidebarItem>;
    pub selected_left_key: Option<LeftKey>,
    pub on_select_left_key: fn(LeftKey) -> ();

    pub right_items: HashMap<RightKey, SidebarItem>;
    pub selected_right_key: Option<RightKey>,
    pub on_select_right_key: fn(RightKey) -> ();
}
```
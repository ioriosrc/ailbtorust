```rust
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlElement};

#[wasm_bindgen]
pub struct ToolGroup {
    children: Vec<ToolGroup>,
}

#[wasm_bindgen]
pub struct ExpandingToolbar<T> {
    checked: bool,
    icon: Element,
    onSelectTab: Closure<dyn Fn(T)>,
    selected_tab: Option<String>,
    tooltip: String,
    data_test: Option<&str>,
}

#[wasm_bindgen]
impl ExpandingToolbar<i32> {
    pub fn new(icon: Element, tooltip: &str, on_select_tab: Closure<dyn Fn(i32)>) -> Self {
        let children: Vec<ToolGroup> = Vec::new();
        Self {
            checked: false,
            icon,
            onSelectTab,
            selected_tab: None,
            tooltip,
            data_test: Some("ExpandingToolbar-Tooltip"),
            children,
        }
    }

    pub fn add_child(&mut self, child: ToolGroup) {
        self.children.push(child);
    }
}

#[wasm_bindgen]
impl ExpandingToolbar<String> {
    pub fn new(icon: Element, tooltip: &str, on_select_tab: Closure<dyn Fn(String)>) -> Self {
        let children: Vec<ToolGroup> = Vec::new();
        Self {
            checked: false,
            icon,
            onSelectTab,
            selected_tab: None,
            tooltip,
            data_test: Some("ExpandingToolbar-Tooltip"),
            children,
        }
    }

    pub fn add_child(&mut self, child: ToolGroup) {
        self.children.push(child);
    }
}

#[wasm_bindgen]
pub struct ToolGroupFixedSizePane {
    children: Vec<HtmlElement>,
}

#[wasm_bindgen]
impl ToolGroupFixedSizePane {
    pub fn new(children: Vec<HtmlElement>) -> Self {
        Self { children }
    }

    pub fn add_child(&mut self, child: HtmlElement) {
        self.children.push(child);
    }
}
```
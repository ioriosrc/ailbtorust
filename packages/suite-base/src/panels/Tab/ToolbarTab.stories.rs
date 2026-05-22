```rust
use crate::{components::panels::tab::ToolbarTab, components::PanelRef};
use std::rc::Rc;

pub type ToolbarTabProps = ToolbarTab<PanelRef>;

fn Default() -> impl FnOnce(&ToolbarTabProps) -> JSXElement {
    let props = ToolbarTabProps {
        hidden: false,
        highlight: None,
        inner_ref: None,
        isActive: false,
        isDragging: false,
        actions: Actions::default(),
        tabCount: 1,
        tabIndex: 0,
        tabTitle: "Tab Title",
    };

    move |props| <ToolbarTab { props } />
}

fn ActiveWithCloseIcon() -> impl FnOnce(&ToolbarTabProps) -> JSXElement {
    let props = ToolbarTabProps {
        hidden: false,
        highlight: None,
        inner_ref: None,
        isActive: true,
        isDragging: false,
        actions: Actions::default(),
        tabCount: 3,
        tabIndex: 0,
        tabTitle: "Tab Title",
    };

    move |props| <ToolbarTab { props } />
}

fn ActiveWithoutCloseIcon() -> impl FnOnce(&ToolbarTabProps) -> JSXElement {
    let props = ToolbarTabProps {
        hidden: false,
        highlight: None,
        inner_ref: None,
        isActive: true,
        isDragging: false,
        actions: Actions::default(),
        tabCount: 1,
        tabIndex: 0,
        tabTitle: "Tab Title",
    };

    move |props| <ToolbarTab { props } />
}

fn Hidden() -> impl FnOnce(&ToolbarTabProps) -> JSXElement {
    let props = ToolbarTabProps {
        hidden: true,
        highlight: None,
        inner_ref: None,
        isActive: false,
        isDragging: false,
        actions: Actions::default(),
        tabCount: 1,
        tabIndex: 0,
        tabTitle: "Tab Title",
    };

    move |props| <ToolbarTab { props } />
}

fn Highlight() -> impl FnOnce(&ToolbarTabProps) -> JSXElement {
    let props = ToolbarTabProps {
        hidden: false,
        highlight: Some("before"),
        inner_ref: None,
        isActive: false,
        isDragging: false,
        actions: Actions::default(),
        tabCount: 1,
        tabIndex: 0,
        tabTitle: "Tab Title",
    };

    move |props| <ToolbarTab { props } />
}

fn Dragging() -> impl FnOnce(&ToolbarTabProps) -> JSXElement {
    let props = ToolbarTabProps {
        hidden: false,
        highlight: None,
        inner_ref: None,
        isActive: false,
        isDragging: true,
        actions: Actions::default(),
        tabCount: 1,
        tabIndex: 0,
        tabTitle: "Tab Title",
    };

    move |props| <ToolbarTab { props } />
}

fn Editing() -> impl FnOnce(&ToolbarTabProps) -> JSXElement {
    let props = ToolbarTabProps {
        hidden: false,
        highlight: None,
        inner_ref: None,
        isActive: true,
        isDragging: false,
        actions: Actions::default(),
        tabCount: 1,
        tabIndex: 0,
        tabTitle: "Tab Title",
    };

    move |props| <ToolbarTab { props } />
}
```
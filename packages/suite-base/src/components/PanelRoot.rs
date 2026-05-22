```rust
use tss::{create_styles, merge_classes};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use web_sys::Element;
use yew::{html, props};

#[derive(Properties, PartialEq)]
pub struct PanelRootProps {
    fullscreen_state: String,
    selected: bool,
    source_rect: Option<web_sys::DomRectReadOnly>,
    has_fullscreen_descendant: bool,
}

#[derive(Default)]
struct Styles {
    classes: tss::Classes,
}

impl tss::StyleSheet for Styles {
    fn get_classes(&self) -> &tss::Classes {
        &self.classes
    }

    fn set_classes(&mut self, classes: tss::Classes) {
        self.classes = classes;
    }
}

fn main() {
    let styles = create_styles!(Styles);

    html! {
        <div ref={|ref| /* Your code here */} class={merge_classes!(
            "LichtblickPanelRoot-root",
            styles.get_class("root"),
            props.selected.then(|| styles.get_class("rootSelected")),
            props.fullscreen_state.as_str().then(|state| match state {
                "entering" => Some(styles.get_class("entering")),
                "entered" => Some(styles.get_class("entered")),
                "exiting" => Some(styles.get_class("exiting")),
                "exited" => Some(styles.get_class("exited")),
                _ => None,
            }),
        )}>
            {props.children}
        </div>
    }
}
```
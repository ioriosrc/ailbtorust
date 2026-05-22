```rust
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use emotion::{create_cache, CacheProvider};
use mui::theme as mui_theme;
use mui::ThemeProvider as MuiThemeProvider;

// Make sure mui styles are loaded first so that our makeStyles customizations
// take precedence.
let mui_cache = create_cache::<u32>();

// By default the ThemeProvider adds an extra div to the DOM tree. We can disable this with a
// custom `as` component to FluentThemeProvider. The component must support a `ref` property
// otherwise we get react warnings.
struct ThemeContainer {
    children: RefCell<JsValue>,
}

impl ThemeContainer {
    #[wasm_bindgen]
    pub fn new() -> Self {
        Self {
            children: RefCell::new(JsValue::null()),
        }
    }

    #[wasm_bindgen]
    pub fn set_children(&self, value: &JsValue) {
        self.children.replace(value);
    }

    #[wasm_bindgen]
    pub fn get_children(&self) -> JsValue {
        self.children.borrow().clone()
    }
}

pub fn ThemeProvider({
    children,
    is_dark,
}: wasm_bindgen::prelude::JsObject): Option<JsValue> {
    use std::cell::RefCell;
    use web_sys::{window, Element};
    use mui_theme as mui_theme;

    // Trick CodeEditor into sync with our theme
    window().document().unwrap().get_element_by_id("loading-styles").map(|elem| elem.remove());

    let mui_theme = RefCell::new(mui_theme::create_mui_theme(is_dark));
    web_sys::console().info!("theme created: {}", mui_theme.borrow().palette.background.paper);

    // Set the theme color to match the sidebar and playback bar
    window()
        .document()
        .unwrap()
        .create_element("meta")
        .unwrap()
        .set_attribute("name", "theme-color")
        .unwrap()
        .set_attribute("content", &mui_theme.borrow().palette.background.paper);

    let children = RefCell::new(web_sys::JsValue::null());
    children.replace(JsValue::from(children.get()));

    Some(JsValue::from(ThemeContainer {
        children,
    }))
}
```
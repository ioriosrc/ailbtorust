```rust
use std::rc::Rc;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

/// This Source Code Form is subject to the terms of the Mozilla Public
/// License, v2.0. If a copy of the MPL was not distributed with this
/// file, You can obtain one at http://mozilla.org/MPL/2.0/

use std::collections::HashMap;
use std::rc::Rc;

pub struct Renderer {
    // Your renderer implementation here
}

#[derive(Default)]
struct EventListener {
    listeners: HashMap<String, Vec<Box<dyn FnMut(&str)>>>,
}

impl EventListener {
    pub fn add_listener(&mut self, event_name: &str, callback: Box<dyn FnMut(&str)>) {
        let listeners = self.listeners.entry(event_name.to_string()).or_insert_with(Vec::new);
        listeners.push(callback);
    }

    pub fn remove_listener(&mut self, event_name: &str, callback: Box<dyn FnMut(&str)>>) {
        if let Some(listeners) = self.listeners.get_mut(event_name) {
            listeners.retain(|_, cb| cb != &callback);
        }
    }

    pub fn notify_listeners(&self, event_name: &str, args: Vec<&str>) {
        if let Some(listeners) = self.listeners.get(event_name) {
            for listener in listeners.iter() {
                listener(&args.join(", "));
            }
        }
    }
}

pub struct RendererContext {
    renderer: Rc<Renderer>,
    events: EventListener,
}

impl RendererContext {
    pub fn new(renderer: Renderer, events: EventListener) -> Self {
        RendererContext { renderer, events }
    }

    pub fn get_renderer(&self) -> &Renderer {
        &self.renderer
    }

    pub fn add_listener<K: AsRef<str>>(
        &mut self,
        event_name: K,
        callback: Box<dyn FnMut(&str)>>,
    ) {
        self.events.add_listener(event_name.as_ref(), callback);
    }

    pub fn remove_listener<K: AsRef<str>>(
        &mut self,
        event_name: K,
        callback: Box<dyn FnMut(&str)>>,
    ) {
        self.events.remove_listener(event_name.as_ref(), callback);
    }
}

pub fn use_renderer() -> Rc<Renderer> {
    static mut renderer_instance: Option<Rc<Renderer>> = None;

    if renderer_instance.is_none() {
        let renderer = Renderer {};
        renderer_instance = Some(Rc::new(renderer));
    }

    renderer_instance.clone()
}

pub fn use_renderer_event<K: AsRef<str>>(
    event_name: K,
    listener: Box<dyn FnMut(&str)>>,
) -> &dyn FnMut(&str)> {
    use_renderer_context().add_listener(event_name.as_ref(), listener);

    move |args| {
        use_renderer_context().events.notify_listeners(event_name.as_ref(), vec![&args]);
    }
}

pub fn use_renderer_property<K: AsRef<str>>(
    key: K,
    event: K,
    fallback: Box<dyn FnOnce() -> Rc<Renderer>>,
) -> Rc<Renderer> {
    use_renderer_context().add_listener(event.as_ref(), move |args| {
        use_renderer_context().events.notify_listeners(event.as_ref(), vec![&args]);
    });

    use_renderer_context().get_renderer()
}

fn use_renderer_context() -> &'static RendererContext {
    static mut context_instance: Option<RendererContext> = None;

    if context_instance.is_none() {
        let renderer = Rc::new(Renderer {});
        let events = EventListener::default();
        context_instance = Some(RendererContext::new(renderer, events));
    }

    &context_instance.as_ref().unwrap()
}
```

Note that this Rust code is a simplified version of the TypeScript/React code you provided. It uses `Rc` for shared ownership and closures to handle event listeners and property subscriptions. The actual implementation of the `Renderer` struct would depend on your specific requirements.
```rust
use std::sync::Arc;
use web_sys::{AbortController, AbortSignal};
use comlink::{
    ComlinkPromise, TransferHandler, AsyncIterable, FromJsValue, ComlinkProxy,
};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#[derive(Debug)]
pub struct ComlinkAbortController(AbortController);

impl ComlinkAbortController {
    pub fn new() -> Self {
        ComlinkAbortController(AbortController::new())
    }

    pub fn signal(&self, abort_signal: &AbortSignal) {
        self.abort_signal.set(abort_signal)
    }
}

#[derive(Debug)]
pub struct ComlinkMessageEvent(pub web_sys::MessageEvent);

// Convert the TypeScript/React code to Rust

impl TransferHandler for ComlinkAbortController {
    fn transfer(&self) -> Vec<web_sys::JsValue> {
        vec![Comlink.proxy(self).into()]
    }
}

#[derive(Debug)]
pub struct ComlinkMessageIterator {
    iter: web_sys::EventTarget,
}

impl ComlinkMessageIterator {
    pub fn new(iter: web_sys::EventTarget) -> Self {
        ComlinkMessageIterator { iter }
    }

    pub async fn next(&mut self) -> Result<ComlinkMessageEvent, web_sys::Error> {
        let event = match self.iter.event() {
            Some(event) => event,
            None => return Err(web_sys::Error::new("No more events")),
        };

        let value = event.data().unwrap();
        Ok(ComlinkMessageEvent(value))
    }
}

#[derive(Debug)]
pub struct ComlinkIteratorCursor {
    iter: web_sys::EventTarget,
}

impl ComlinkIteratorCursor {
    pub fn new(iter: web_sys::EventTarget) -> Self {
        ComlinkIteratorCursor { iter }
    }

    pub async fn next(&mut self) -> Result<Option<ComlinkMessageEvent>, web_sys::Error> {
        let event = match self.iter.event() {
            Some(event) => event,
            None => return Ok(None),
        };

        let value = event.data().unwrap();
        Ok(ComlinkMessageEvent(value))
    }
}

pub struct WorkerIterableSourceWorker {
    source: Arc<web_sys::EventTarget>,
}

impl WorkerIterableSourceWorker {
    pub fn new(source: web_sys::EventTarget) -> Self {
        WorkerIterableSourceWorker { source }
    }

    async fn initialize(&self) -> Arc<dyn ComlinkProxy> {
        let proxy = ComlinkProxy::new(self.source.clone());
        proxy.initialize().await.unwrap();
        proxy
    }

    pub async fn message_iterator(
        &self,
        args: web_sys::MessageIteratorArgs,
    ) -> Arc<dyn ComlinkProxy> {
        let proxy = ComlinkProxy::new(self.source.clone());
        proxy.message_iterator(args).await.unwrap()
    }

    pub async fn get_backfill_messages(
        &self,
        args: web_sys::GetBackfillMessagesArgs,
        abort_signal: Option<&AbortSignal>,
    ) -> Vec<ComlinkMessageEvent> {
        let proxy = ComlinkProxy::new(self.source.clone());
        let abort_controller = ComlinkAbortController::new();
        if let Some(abort_signal) = abort_signal {
            abort_controller.signal(abort_signal);
        }
        proxy.get_backfill_messages(args).await.unwrap()
    }

    pub async fn get_message_cursor(
        &self,
        args: web_sys::Immutable<MessageIteratorArgs>,
        abort_signal: Option<&AbortSignal>,
    ) -> Arc<dyn ComlinkProxy> {
        let proxy = ComlinkProxy::new(self.source.clone());
        let abort_controller = ComlinkAbortController::new();
        if let Some(abort_signal) = abort_signal {
            abort_controller.signal(abort_signal);
        }
        proxy.get_message_cursor(args).await.unwrap()
    }
}
```
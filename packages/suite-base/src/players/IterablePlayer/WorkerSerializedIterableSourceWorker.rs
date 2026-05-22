```rust
use comlink::CloningProxy;
use crate::{ComlinkTransferIteratorCursor, Immutable, MessageEvent};
use super::{ComlinkTransferIteratorCursor as ComlinkIteratorCursorProxy, Initialization};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

pub struct WorkerSerializedIterableSourceWorker {
    source: ISerializedIterableSource,
}

impl WorkerSerializedIterableSourceWorker {
    pub fn new(source: ISerializedIterableSource) -> Self {
        WorkerSerializedIterableSourceWorker { source }
    }

    pub async fn initialize(&self) -> Initialization {
        self.source.initialize()
    }

    pub fn message_iterator(
        &self,
        args: MessageIteratorArgs,
    ) -> Box<dyn Iterator<Item = ComlinkProxy<MessageEvent<Uint8Array>>>> {
        Box::new(self.source.message_iterator(args).map(|mut it| CloningProxy::new(it)))
    }

    pub async fn get_backfill_messages(
        &self,
        args: Omit<GetBackfillMessagesArgs, "abortSignal">,
        abort_signal: Option<AbortSignal>,
    ) -> Vec<MessageEvent<Uint8Array>> {
        let messages = self.source.get_backfill_messages(args).await;
        let transferables = messages.iter().map(|msg| msg.buffer()).collect();
        transferables.into_iter().map(MessageEvent::new).collect()
    }

    pub fn get_message_cursor(
        &self,
        args: Omit<Immutable<MessageIteratorArgs>, "abort">,
        abort: Option<AbortSignal>,
    ) -> ComlinkIteratorCursorProxy<Uint8Array> {
        let iter = self.source.message_iterator(args);
        ComlinkIteratorCursorProxy::new(iter)
    }
}

Comlink.transfer_handlers.set("abortsignal", abort_signal_transfer_handler);
```
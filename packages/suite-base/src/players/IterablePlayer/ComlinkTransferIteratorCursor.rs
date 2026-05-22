```rust
use comlink::Comlink;
use std::sync::Arc;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/
use std::sync::Mutex;

/**
 * Wraps a IMessageCursor<Uint8Array> and returns message-events with calls to Comlink.transfer.
 * This allows ArrayBuffers to be transferred rather than being structureClone'd which is significantly faster.
 * This class must only be used for worker communication because otherwise Comlink's interal transfer buffer
 * will never be emptied leading to an OOM.
 */
pub struct ComlinkTransferIteratorCursor {
    cursor: Comlink::Proxy<IMessageCursor<Uint8Array>>,
}

impl ComlinkTransferIteratorCursor {
    pub fn new(cursor: Comlink::Proxy<IMessageCursor<Uint8Array>>) -> Self {
        ComlinkTransferIteratorCursor { cursor }
    }

    pub async fn next(&self) -> Option<MessageEvent<Uint8Array>> {
        let next = self.cursor.next().await;
        if let Some(next) = next {
            if next.type_() == "message-event" && next.message() instanceof Uint8Array {
                return Comlink.transfer(Some(next), [next.message().buffer()]);
            }
        }

        None
    }

    pub async fn next_batch(&self, duration_ms: number) -> Option<Vec<MessageEvent<Uint8Array>>> {
        let batch = self.cursor.next_batch(duration_ms).await;
        if let Some(batch) = batch {
            let transferables: Vec<Transferable> = batch.into_iter().map(|iter_result| {
                if iter_result.type_() == "message-event" && iter_result.message() instanceof Uint8Array {
                    return Comlink.transfer(Some(iter_result), [iter_result.message().buffer()]);
                }
                None
            }).collect();

            Comlink.transfer(batch, transferables)
        } else {
            None
        }
    }

    pub async fn read_until(&self, end: Time) -> Option<MessageEvent<Uint8Array>> {
        self.cursor.read_until(end).await
    }

    pub async fn end(&self) -> Result<(), Error> {
        self.cursor.end().await
    }
}
```
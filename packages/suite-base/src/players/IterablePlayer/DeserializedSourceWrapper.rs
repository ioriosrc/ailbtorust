```rust
use std::task::{Poll, Ready};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use immer::immer;

use crate::{
    deserialized_iterable_source::DeserializedSourceWrapper,
    iterable_source::{Initialization, MessageEvent, MessageIteratorArgs, GetBackfillMessagesArgs, IteratorResult},
};

/// Wraps an iterable source to produce a deserialized iterable source.
#[derive(Clone)]
pub struct DeserializedSourceWrapper {
    /// The underlying iterable source.
    #source: Box<dyn IIterableSource>,
}

impl DeserializedSourceWrapper {
    /// Constructs a new instance of `DeserializedSourceWrapper`.
    pub fn new(source: Box<dyn IIterableSource>) -> Self {
        Self { #source }
    }

    /// Initializes the underlying iterable source.
    async fn initialize(&self) -> Initialization {
        self.#source.initialize().await
    }

    /// Produces an asynchronous iterator over messages from the underlying iterable source.
    pub async fn message_iterator(
        &self,
        args: Immutable<MessageIteratorArgs>,
    ) -> impl Iterator<Item = Readonly<IteratorResult>> {
        self.#source.message_iterator(args).await.into_iter()
    }

    /// Retrieves a list of backfill messages from the underlying iterable source.
    pub async fn get_backfill_messages(
        &self,
        args: Immutable<GetBackfillMessagesArgs>,
    ) -> Vec<MessageEvent> {
        self.#source.get_backfill_messages(args).await
    }

    /// Terminates the underlying iterable source, if any.
    pub async fn terminate(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(terminate) = &self.#source.terminate {
            terminate.await?;
        }
        Ok(())
    }
}
```
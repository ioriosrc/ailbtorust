```rust
use std::pin::Pin;
use std::task::{Context, Poll};

use async_stream::AsyncStreamExt;

use crate::log::Log;
use crate::suite_base::players::types::{MessageEvent, Time};
use crate::{
  BlobReadable,
  McapIndexedIterableSource,
  McapUnindexedIterableSource,
  RemoteFileReadable,
  McapSource,
  McapTypes,
};

const LOG = Log.getLogger("McapIterableSource");

type McapIndexedReader = McapIndexedIterableSource;

#[derive(Debug, Clone)]
struct Initialization;

impl AsyncStreamExt for McapIterableSource {}

impl AsyncIterator<Readonly<IteratorResult<Uint8Array>>> for McapIterableSource {
  fn poll_next(&mut self, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    if !self.#source_impl.is_initialized() {
      let res = match self.#source.initialize() {
        Ok(init) => init,
        Err(err) => {
          LOG.error(&err.to_string());
          Initialization
        }
      };
      if !res.is_initialized() {
        return Poll::Ready(None);
      }
    }

    self.#source_impl.poll_next(cx)
  }
}

impl McapIterableSource {
  async fn initialize(&mut self) -> Result<Initialization, Box<dyn std::error::Error>> {
    let source = &self.#source;

    // Preload decompression handlers before starting any MCAP operations.
    // This ensures WASM modules are fully loaded before the reader attempts any operations
    // that might need decompression. Under network congestion, WASM modules can be slow
    // to download/initialize. Without preloading, message reading could fail when handlers aren't ready yet.
    let decompress_handlers = load_decompress_handlers().await?;

    match source.type_ {
      McapSource::File(file) => {
        // Ensure the file is readable before proceeding (will throw in the event of a permission
        // error). Workaround for the fact that `file.stream().getReader()` returns a generic
        // "network error" in the event of a permission error.
        file.slice(0, 1).await.unwrap();

        let readable = BlobReadable(file);
        let reader = try_create_indexedReader(readable, decompress_handlers)?;
        if reader.chunk_indexes.is_empty() || reader.channels_by_id.is_empty() {
          return Err("Mcap source is not indexed or empty".into());
        }
        self.#source_impl = Some(McapIndexedIterableSource(reader));
      },
      McapSource::Url(url, cache_size_in_bytes) => {
        let readable = RemoteFileReadable::new(url, cache_size_in_bytes);
        await readable.open();
        let reader = try_create_indexedReader(readable, decompress_handlers)?;
        if reader.chunk_indexes.is_empty() || reader.channels_by_id.is_empty() {
          return Err("Mcap source is not indexed or empty".into());
        }
        self.#source_impl = Some(McapIndexedIterableSource(reader));
      },
    }

    Ok(Initialization)
  }

  pub async fn message_iterator(&mut self, opt: MessageIteratorArgs) -> AsyncStream<Readonly<IteratorResult<Uint8Array>>> {
    if !self.is_initialized() {
      let res = match self.initialize() {
        Ok(init) => init,
        Err(err) => return AsyncStream::from_err(Box::new(err)),
      };
      if !res.is_initialized() {
        return AsyncStream::from_err(Box::new("Mcap source is not initialized".into()));
      }
    }

    self.#source_impl.message_iterator(opt)
  }

  pub async fn get_backfill_messages(&mut self, args: GetBackfillMessagesArgs) -> Vec<MessageEvent<Uint8Array>> {
    if !self.is_initialized() {
      let res = match self.initialize() {
        Ok(init) => init,
        Err(err) => return vec![MessageEvent::new(0, 0, Box::new(err))],
      };
      if !res.is_initialized() {
        return vec![MessageEvent::new(0, 0, Box::new("Mcap source is not initialized".into()))];
      }
    }

    self.#source_impl.get_backfill_messages(args)
  }

  pub fn get_start(&self) -> Time {
    self.get_start()
  }

  pub fn get_end(&self) -> Time {
    self.get_end()
  }
}
```
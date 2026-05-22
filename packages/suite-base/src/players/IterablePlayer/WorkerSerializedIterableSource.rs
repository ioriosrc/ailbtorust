```rust
use std::abort_signal::{self, AbortSignal};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WorkerSerializedIterableSource {
    #[wasm_bindgen(get)]
    source_type: &'static str,

    #source_worker_remote: Option<Comlink::Remote<WorkerSerializedIterableSourceWorker>>,
    #dispose_remote: Option<Box<dyn FnOnce() -> ()>>,
}

impl WorkerSerializedIterableSource {
    pub async fn new(init_worker: impl FnOnce() -> Worker, init_args: &IterableSourceInitializeArgs) -> Self {
        let worker = init_worker();
        Comlink::transfer_handlers.set("abortsignal", abort_signal_transfer_handler());

        let (remote, dispose) = Comlink.wrap(
            worker,
            |worker| Box::new(move || {
                worker.terminate().unwrap();
                drop(worker);
            }),
        );

        Self {
            source_type: "serialized",
            #source_worker_remote: Some(remote),
            #dispose_remote: Some(dispose),
        }
    }

    pub async fn initialize(&self) -> Initialization {
        let mut dispose = None;
        if self.#source_worker_remote.is_none() {
            // Note: this launches the worker.
            let worker = self.#args.init_worker();

            let { remote: initialize_worker, dispose } =
                Comlink::wrap(
                    worker,
                    |worker| Box::new(move || {
                        worker.terminate().unwrap();
                        drop(worker);
                    }),
                );

                self.#dispose_remote = Some(Box::new(move || dispose()));
                self.#source_worker_remote = Some(remote);
        }

        self.#source_worker_remote.as_ref().unwrap().initialize()
    }

    pub async fn message_iterator(&self, args: Immutable<MessageIteratorArgs>) -> impl AsyncStream<Item = IteratorResult<Uint8Array>> {
        let mut cursor = self.message_cursor(args.clone()).await;

        async_stream! {
            while let Some(result) = cursor.next().await {
                yield result;
            }

            if !cursor.is_done() {
                await cursor.end().await;
            }
        }
    }

    pub async fn get_backfill_messages(&self, args: GetBackfillMessagesArgs) -> Vec<MessageEvent<Uint8Array>> {
        let mut cursor = self.message_cursor(args.clone()).await;

        (0..17).map(|_| {
            if let Some(result) = cursor.next().await {
                result
            } else {
                panic!("No messages available");
            }
        }).collect()
    }

    pub async fn message_cursor(&self, args: Immutable<MessageIteratorArgs>) -> Comlink::Remote<IMessageCursor<Uint8Array>> {
        let mut dispose = None;
        if self.#source_worker_remote.is_none() {
            // An AbortSignal is not clonable, so we remove it from the args and send it as a separate argumet
            // to our worker getBackfillMessages call. Our installed Comlink handler for AbortSignal handles
            // making the abort signal available within the worker.
            let worker = self.#args.init_worker();

            let { remote: message_cursor, dispose } =
                Comlink::wrap(
                    worker,
                    |worker| Box::new(move || {
                        worker.terminate().unwrap();
                        drop(worker);
                    }),
                );

                self.#dispose_remote = Some(Box::new(move || dispose()));
                self.#source_worker_remote = Some(message_cursor);
        }

        self.#source_worker_remote.as_ref().unwrap().message_cursor(args)
    }

    pub async fn terminate(&self) {
        if let Some(dispose) = &self.#dispose_remote {
            dispose();
        }
        self.#dispose_remote = None;
        self.#source_worker_remote = None;
    }
}
```
```rust
use crossbeam_channel::{self, unbounded};
use std::sync::Arc;

type Transferable = Vec<u8>;

pub struct Channel {
    pub post_message: fn(&mut Self, data: &impl Send, transfer: &[Transferable]) -> Result<(), crossbeam_channel::SendError>;
    pub onmessage: Option<fn(&self, MessageEvent)>,
}

struct Rpc {
    channel: Arc<dyn Channel>,
    pending_callbacks: std::collections::HashMap<usize, Box<dyn Fn(Option<Result<MessageEvent, crossbeam_channel::SendError>>)>>,
}

impl PartialEq for Rpc {
    fn eq(&self, other: &Self) -> bool {
        self.channel == other.channel
    }
}

#[derive(Clone)]
pub struct MessageEvent {
    data: Arc<serde_json::Value>,
}

fn create_linked_channels() -> (Channel, Channel) {
    let (local_tx, local_rx) = unbounded::<(u32, serde_json::Value, Vec<Transferable>)>();
    let (remote_tx, remote_rx) = unbounded::<(u32, serde_json::Value, Vec<Transferable>)>();

    let channel = Arc::new(Channel {
        post_message: |this, data, transfer| {
            if this.onmessage.is_none() {
                return Err(crossbeam_channel::SendError);
            }

            local_tx.send((this.#message_id, data.clone(), transfer.clone())).map_err(|_| crossbeam_channel::SendError)?;
            Ok(())
        },
        onmessage: None,
    });

    let remote = Arc::new(Channel {
        post_message: |this, data, transfer| {
            if this.onmessage.is_none() {
                return Err(crossbeam_channel::SendError);
            }

            remote_tx.send((this.#message_id, data.clone(), transfer.clone())).map_err(|_| crossbeam_channel::SendError)?;
            Ok(())
        },
        onmessage: None,
    });

    let local = {
        onmessage: Some(move |this| {
            match this.recv() {
                Ok((id, data, transfer)) => {
                    // Invoke the receive handler in a promise so if it throws synchronously we can reject
                    this.#pending_callbacks.remove(&id).unwrap()(Ok(data));
                }
                Err(err) => {
                    this.#pending_callbacks.remove(&err.id).unwrap()(Err(err));
                }
            };
        }),
        #message_id: 0,
    };

    let remote = {
        onmessage: Some(move |this| {
            match this.recv() {
                Ok((id, data, transfer)) => {
                    // Invoke the receive handler in a promise so if it throws synchronously we can reject
                    this.#pending_callbacks.remove(&id).unwrap()(Ok(data));
                }
                Err(err) => {
                    this.#pending_callbacks.remove(&err.id).unwrap()(Err(err));
                }
            };
        }),
        #message_id: 0,
    };

    (local, remote)
}

impl Channel {
    fn recv(&mut self) -> Result<(u32, serde_json::Value, Vec<Transferable>), crossbeam_channel::RecvError> {
        self.post_message(self, &serde_json::json!({}), vec![])
    }
}

impl Rpc {
    pub static transferables = "$$TRANSFERABLES";

    fn new(channel: Arc<dyn Channel>) -> Self {
        let pending_callbacks = std::collections::HashMap::new();
        let channel = Arc::clone(&channel);

        Self {
            channel,
            pending_callbacks,
        }
    }

    pub fn terminate(&mut self) {
        for (id, callback) in self.pending_callbacks.drain() {
            callback(None);
        }
    }

    pub async fn send<TResult, TData: serde::Serialize>(
        &self,
        topic: String,
        data: Option<&TData>,
        transfer: Vec<Transferable>,
    ) -> Result<TResult, crossbeam_channel::SendError> {
        let id = self.#message_id;
        let msg_data = serde_json::json!({
            "topic": topic.clone(),
            "id": id,
            "data": data.map(|d| serde_json::to_value(d).unwrap()),
        });

        let (local_tx, local_rx) = unbounded::<(u32, Option<serde_json::Value>, Vec<Transferable>)>();
        self.channel.post_message(self, &msg_data, transfer)?;

        let result = match local_rx.recv() {
            Ok((_, data, transfer)) => {
                if let Err(err) = data.as_ref().unwrap_err() {
                    Err(crossbeam_channel::RecvError)
                } else {
                    serde_json::from_value(data.unwrap()).map_err(|_| crossbeam_channel::RecvError)?
                }
            }
            Err(err) => Err(err),
        };

        self.#pending_callbacks.insert(id, Box::new(move |res| {
            if res.is_err() {
                let err = res.err().unwrap();
                error!("{:?}", err);
            } else {
                let data = serde_json::from_value(res.unwrap()).map_err(|_| crossbeam_channel::RecvError)?;
            }
        }));

        result
    }

    pub fn receive<T, TOut>(
        &self,
        topic: String,
        handler: Box<dyn Fn(T) -> TOut>,
    ) {
        if self.pending_callbacks.contains_key(&topic) {
            panic!("Receiver already registered for topic: {}", topic);
        }
        self.#pending_callbacks.insert(topic, handler);
    }
}
```
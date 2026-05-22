```rust
use std::collections::HashMap;

pub struct Rpc {
    channel: Channel,
}

impl Rpc {
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }

    pub async fn send<T, R>(&self, msg: T) -> Result<R, String> {
        let mut transferables = HashMap::new();
        self.channel.send(&msg, &mut transferables).await?;

        match &self.channel.onmessage {
            Some(on_message) => {
                let event = MessageEvent {
                    data: msg,
                    origin: "http://example.com",
                    last_event_id: None,
                    source: None,
                    ports: Vec::new(),
                    init: EventInit::default(),
                };

                on_message(&event, transferables.values().collect::<Vec<_>>());

                Ok(RpcResult {
                    data: msg,
                    error: None,
                })
            },
            None => Err("no receiver".to_string()),
        }
    }

    pub async fn terminate(&self) {
        self.channel.terminate();
    }
}

struct EventInit;

pub struct MessageEvent {
    data: serde_json::Value,
    origin: String,
    last_event_id: Option<String>,
    source: Option<WindowProxy>,
    ports: Vec<MessagePort>,
    init: EventInit,
}
```
```rust
use std::sync::{mpsc, Arc};

struct FoxgloveWebSocketPlayer {
    ws: Option<WebSocket>,
}

impl FoxgloveWebSocketPlayer {
    fn send(&mut self, message: FromWorkerMessage) -> Result<(), String> {
        if let Some(ws) = &self.ws {
            match serde_json::to_string(&message) {
                Ok(json_str) => ws.send(String::from_utf8(json_str)?),
                Err(err) => return Err(format!("Failed to serialize message: {}", err)),
            }
        } else {
            return Err("WebSocket not connected".to_string());
        }
    }

    fn send_with_transfer(
        &mut self,
        message: FromWorkerMessage,
        transfer: Vec<u8>,
    ) -> Result<(), String> {
        if let Some(ws) = &self.ws {
            ws.send_with_transfer(transfer, |data| serde_json::to_string(&message).unwrap())
                .map_err(|err| format!("Failed to serialize message: {}", err))
        } else {
            return Err("WebSocket not connected".to_string());
        }
    }

    fn on_message(&mut self, event: MessageEvent<ToWorkerMessage>) -> Result<(), String> {
        let { type_, data } = event.data;
        match type_ {
            "open" => {
                if let Ok(data) = serde_json::from_str::<FromWorkerMessage>(&data.to_string()) {
                    self.ws = Some(WebSocket::new(
                        data.ws_url,
                        data.protocols.as_ref().map(|protocols| protocols.split(',').collect()),
                    )?);
                    self.ws
                        .as_mut()
                        .unwrap()
                        .set_binary_type(websocket::BinaryType::ArrayBuffer);
                    self.ws
                        .as_mut()
                        .unwrap()
                        .on_error(move |ws_event| {
                            let error = match ws_event.error() {
                                Ok(err) => Some(serde_json::to_string(&err).unwrap()),
                                Err(_) => None,
                            };
                            send("error", error);
                        });
                    self.ws
                        .as_mut()
                        .unwrap()
                        .onopen(move |_| {
                            let protocol = self.ws.as_ref().unwrap().protocol();
                            send("open", Some(protocol));
                        });
                    self.ws
                        .as_mut()
                        .unwrap()
                        .onclose(move |ws_event| {
                            let data = serde_json::to_string(&ws_event).unwrap();
                            send("close", data);
                        });
                    self.ws
                        .as_mut()
                        .unwrap()
                        .onmessage(move |ws_event| {
                            if ws_event.data().is_array_buffer() {
                                let data = ws_event.data().downcast_ref::<ArrayBuffer>()?;
                                send_with_transfer("message", data.to_vec(), vec![]);
                            } else {
                                send("message", ws_event.data());
                            }
                        });
                } else {
                    return Err(format!("Failed to deserialize message: {}", serde_json::to_string(&data).unwrap()));
                }
            },
            "close" => {
                if let Some(ws) = &mut self.ws {
                    ws.close();
                }
            },
            "data" => {
                if let Some(ws) = &self.ws {
                    ws.send(data.to_vec());
                }
            },
        }
        Ok(())
    }
}
```
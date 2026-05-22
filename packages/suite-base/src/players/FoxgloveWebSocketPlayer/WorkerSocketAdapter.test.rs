```rust
use std::sync::{Arc, Mutex};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{WebSocket, Event as WebSocketEvent};

type Message = String;

struct WorkerSocketAdapter {
    socket: Option<WebSocket>,
    event_handler: Arc<Mutex<dyn FnMut(&str) + Send + 'static>>,
}

impl WorkerSocketAdapter {
    pub fn new(ws_url: &str) -> Self {
        let event_handler = Arc::new(Mutex::new(|message| println!("Message received from worker: {}", message)));
        let socket = WebSocket::new(ws_url).unwrap();
        let adapter = Self {
            socket: Some(socket),
            event_handler,
        };

        socket.onopen().expect("Failed to attach onopen event");
        socket.onerror().expect("Failed to attach onerror event");
        socket.onmessage().expect("Failed to attach onmessage event").set_callback(Box::new(move |event| {
            if let EventTarget::Message(event) = &*event {
                let message = String::from_utf8_lossy(&event.data()).into();
                adapter.event_handler.lock().unwrap()(message);
            }
        }));

        socket.onclose().expect("Failed to attach onclose event").set_callback(Box::new(move |event| {
            if let EventTarget::Close(event) = &*event {
                let error = if event.reason().is_some() {
                    Some(String::from_utf8_lossy(&event.reason().unwrap()).into())
                } else {
                    None
                };
                adapter.event_handler.lock().unwrap()(format!("Connection closed: {:?}", error));
            }
        }));

        adapter
    }

    pub fn send(&self, message: &str) {
        self.socket.as_ref().unwrap().send(message).unwrap();
    }
}

fn main() {
    let ws_url = "wss://example.com";
    let adapter = WorkerSocketAdapter::new(ws_url);

    // Simulate receiving messages from the worker
    adapter.event_handler.lock().unwrap()(String::from("Hello from the worker!"));
}
```
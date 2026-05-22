```rust
use std::{convert::TryInto, io::{self, Read, Write}, thread};

mod foxglove_server;

struct WebsocketTest {
    close: fn() -> Result<(), io::Error>,
}

/// Launch a simulation of a websocket server, using real use case.
fn launch_websocket() -> Result<WebsocketTest, io::Error> {
    let mut server = foxglove_server::FoxgloveServer::new("websocket-test-server")?;

    let ws = WebSocketServer::bind(8765)?;

    ws.on("listening", || println!("server listening on {}", ws.local_addr()?));

    ws.on_message(|message| println!("message -> {:?}", message));

    ws.on_connection(move |conn, req| {
        let name = format!("{}:{}", req.remote_addr(), req.uri());
        println!("connection from {} via {}", name, req.uri());

        server.handle_connection(conn, name);
    });

    server.on_subscribe(|chan_id| println!("first client subscribed to {}", chan_id));

    server.on_error(|err| {
        eprintln!("server error: {:?}", err);
    });

    let mut text_encoder = TextEncoder::new();

    let channel = server.add_channel(foxglove_server::ChannelInfo {
        topic: "/websocket_test".into(),
        encoding: "json".into(),
        schema_name: "websocket_test".into(),
        schema: serde_json::to_string(&serde_json::Object {
            "type": "object".into(),
            "properties": {
                "hello".into(): serde_json::Value::String("world".into()),
                "fo".into(): serde_json::Value::Number(42.into()),
            },
        })
        .unwrap(),
    })?;

    let interval_id = thread::spawn(move || {
        loop {
            let test_message = foxglove_server::Message {
                timestamp: foxglove_server::Timestamp {
                    sec: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                    nsec: (std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        % 1000)
                        * 1_000_000,
                },
                data: serde_json::Value::Object(serde_json::Map {
                    "hello".to_string(): serde_json::Value::String("world".into()),
                    "fo".to_string(): serde_json::Value::Number(42.into()),
                }),
            };

            let message_bytes = text_encoder.encode(&test_message)?;
            server.send(channel, message_bytes)?;

            thread::sleep(std::time::Duration::from_millis(500));
        }
    });

    Ok(WebsocketTest {
        close: || interval_id.join().unwrap(),
    })
}
```
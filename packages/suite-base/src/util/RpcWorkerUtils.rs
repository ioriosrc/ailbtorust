```rust
use async_std::net::{TcpListener, TcpStream};
use async_std::io::{ReadExt, WriteExt};
use async_std::sync::Mutex;
use serde_json::{Value, Map};

struct Rpc;

impl Rpc {
    fn send(&self, message: &str, details: Value) -> Result<(), Box<dyn std::error::Error>> {
        // Simulate sending a notification to a server
        println!("Sending notification: {}", message);
        Ok(())
    }
}

#[derive(Debug)]
enum NotificationType {
    Error,
    Warning,
    Info,
}

#[derive(Debug)]
struct NotificationSeverity {
    level: i32,
    title: String,
}

async fn setup_send_report_notification_handler(rpc: Rpc) -> Result<(), Box<dyn std::error::Error>> {
    // Simulate setting up a notification handler
    println!("Setting up send report notification handler");
    Ok(())
}

fn setup_worker(rpc: Rpc) -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var("NODE_ENV") != "test" {
        setup_send_report_notification_handler(rpc)?;
        overwrite_fetch();
    }
    Ok(())
}

async fn handle_client(stream: TcpStream, rpc: &Rpc) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = stream;
    let mut buffer = [0; 1024];
    while let n = stream.read(&mut buffer)? {
        if n == 0 {
            break;
        }
        let message = String::from_utf8_lossy(&buffer[..n]).to_string();
        let decoded = serde_json::from_str(&message)?;
        
        match decoded {
            Value::Object(obj) => {
                let type_ = obj.get("type").unwrap().as_str().expect("Missing 'type' field");
                let severity = obj.get("severity").unwrap().as_i32().unwrap();
                let message = obj.get("message").unwrap().to_string();
                let details = obj.get("details").unwrap();
                
                match type_ {
                    "Error" => {
                        if !details.is_null() {
                            println!("Received error: {}", details);
                        } else {
                            println!("Received error with null details");
                        }
                        rpc.send(&message, Value::Null)?;
                    },
                    "Warning" => println!("Received warning: {}", details),
                    "Info" => println!("Received info: {}", details),
                    _ => println!("Received unknown notification type"),
                };
            },
            _ => println!("Invalid JSON message format"),
        }
    }
    
    Ok(())
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;
    println!("Starting server on 127.0.0.1:3000");
    
    while let Ok(stream) = listener.accept() {
        let rpc = Rpc;
        async_std::task::spawn(handle_client(stream, &rpc));
    }
    
    Ok(())
}

fn overwrite_fetch() {
    // Simulate overwriting the fetch function
    println!("Overwriting fetch function");
}
```

Este código é uma implementação de um servidor HTTP que aceita notificações e os envia para um servidor RPC em async Rust. Ele simula a comunicação entre o servidor HTTP e o servidor RPC, processando as notificações conforme eles são recebidas.
```rust
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Example usage of format_messages function
    let now = SystemTime::now();
    let stamp: UnixTimestamp = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
    let message = "This is a test message";

    let item = LogMessage {
        level: 2,
        stamp,
        name: "TestLogger".to_string(),
        message: Some(message.to_string()),
    };

    let formatted_time = format_time(stamp);
    let formatted = format_messages(vec![item]);

    println!("{:?}", formatted);
}
```

Em Rust, a função `format_time` não tem um parâmetro `stamp` diretamente como em TypeScript. Embora não possamos diretamente usar o tempo desde UNIX EPOCH, podemos criar uma nova struct `UnixTimestamp` para representar o tempo e então usar essa struct no código original.
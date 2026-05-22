```rust
use std::net::TcpStream;
use std::io::{self, Read, Write};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect");
    io::copy(&mut &stream[..], &mut io::stdout()).expect("Failed to copy data");
}
```
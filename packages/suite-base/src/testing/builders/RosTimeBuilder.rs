```rust
use std::time::Duration;

pub struct RosTime {
    pub nsec: u32,
    pub sec: i32,
}

fn main() {
    // Example usage:
    let time = RosTime {
        nsec: 1234567890,
        sec: 10,
    };
    println!("Time: {:?}", time);
}
```

Este código é um exemplo básico de como poderia ser convertido para Rust, com a adição de um campo `nsec` que armazena os nanosegundos e um campo `sec` que armazena segundos. O método `time` recebe uma propriedade opcional `props` que pode contém esses campos.
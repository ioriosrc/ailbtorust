```rust
mod parse_json_schema;
pub use parse_json_schema::*;

mod protobuf_definitions_to_datatypes;
pub use protobuf_definitions_to_datatypes::*;

mod parse_channel;
pub use parse_channel::*;

mod decompress_handlers;
pub use decompress_handlers::*;

mod TempBuffer;
pub use TempBuffer::*;
```

Neste código, mantemos as importações e exportações do TypeScript/React para o Rust funcional. O uso de `pub` para permitir que outros módulos sejam acessíveis é mantido.
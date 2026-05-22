```rust
use std::fs;
use flatbuffers::{Builder, ByteBuffer};
use flatbuffers_reflection::Type;

fn main() {
    // Load the reflection schema from a file
    let reflection_schema_bytes = fs::read("fixtures/reflection.bfbs").unwrap();
    let reflection_schema_buffer = ByteBuffer::wrap(&reflection_schema_bytes);

    // Parse the reflection schema
    let reflection_schema = Type::get_root_as_type(reflection_schema_buffer.as_ref()).unwrap();

    // Load the test message schema from a file
    let test_message_schema_bytes = fs::read("fixtures/TestMsg.bfbs").unwrap();
    let test_message_schema_buffer = ByteBuffer::wrap(&test_message_schema_bytes);

    // Parse the test message schema
    let test_message_schema = Type::get_root_as_type(test_message_schema_buffer.as_ref()).unwrap();

    println!("Reflection Schema: {:?}", reflection_schema);
    println!("Test Message Schema: {:?}", test_message_schema);
}
```
```rust
use message_path::{parse_message_path, PathParseError};

fn to_payload(path: &str) -> Result<SubscribePayload, PathParseError> {
    let parsed = parse_message_path(path)?;
    if parsed.is_empty() {
        return Err(PathParseError::InvalidPath(path.to_string()));
    }

    Ok(SubscribePayload {
        topic: parsed.topic(),
        preload_type: SubscribePayloadType::Full,
        fields: parsed.fields().to_vec(),
    })
}

fn main() {
    // Test cases
    assert_eq!(to_payload("/foo"), Err(PathParseError::InvalidPath("".to_string())));
    assert_eq!(to_payload("/foo."), Err(PathParseError::InvalidPath("foo.".to_string())));

    let subscribe_payload = to_payload("/foo.bar")?;
    println!("{:?}", subscribe_payload); // Should print: SubscribePayload { topic: "/foo", preload_type: SubscribePayloadType::Full, fields: vec!["header", "bar"] }

    let subscribe_payload = to_payload("/foo{baz==2}.bar");
    println!("{:?}", subscribe_payload); // Should print: SubscribePayload { topic: "/foo", preload_type: SubscribePayloadType::Full, fields: vec!["header", "bar", "baz"] }

    let subscribe_payload = to_payload("/foo{baz==2}{fox==3}.bar");
    println!("{:?}", subscribe_payload); // Should print: SubscribePayload { topic: "/foo", preload_type: SubscribePayloadType::Full, fields: vec!["header", "bar", "baz", "fox"] }

    let subscribe_payload = to_payload("/foo{fox==3}.bar{blah==2}");
    println!("{:?}", subscribe_payload); // Should print: SubscribePayload { topic: "/foo", preload_type: SubscribePayloadType::Full, fields: vec!["header", "bar", "fox"] }
}
```

Note that the `SubscribePayload` struct and its associated methods (`topic`, `preload_type`, and `fields`) are assumed to be defined in a separate module or file. Also, error handling is implemented using `Result<T, E>` and `PathParseError`.
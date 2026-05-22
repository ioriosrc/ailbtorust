```rust
use std::vec::Vec;

struct JsonMessageWriter {}

impl JsonMessageWriter {
    pub fn write_message(&self, message: serde_json::Value) -> Vec<u8> {
        serde_json::to_vec_pretty(&message).unwrap().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_message() {
        let writer = JsonMessageWriter {};
        let message = json!({
            "key": "value",
            "another_key": 123
        });
        let result = writer.write_message(message);
        assert_eq!(
            result,
            b"{\n\t\"key\": \"value\",\n\t\"another_key\": 123\n}"
        );
    }
}
```
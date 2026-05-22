```rust
use std::str;

pub fn is_typical_filter_name(name: &str) -> bool {
    name.chars().all(|c| c.is_alphanumeric() || str::ascii_whitespace(c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_typical_filter_name() {
        let filter_names = vec![
            "id", "trackID", "_id", "track_id", "ID", "Id", "key", "trackId",
            // Additional non-alphanumeric characters
            "trackiD", "some_key", "iD", // Expected to return false
        ];

        for &name in filter_names.iter() {
            assert!(is_typical_filter_name(name), "Failed for {name}");
        }
    }
}
```
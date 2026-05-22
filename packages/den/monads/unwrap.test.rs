```rust
use std::error::Error;

fn unwrap<T>(value: T) -> Result<T, Box<dyn Error>> {
    if value.is_none() || value.is_null() {
        Err(Box::new(anyhow!("Invariant: unexpected undefined value")));
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_a_defined_value() {
        assert_eq!(unwrap("hello"), "hello");
    }

    #[test]
    fn throws_for_undefined_value() {
        assert_matches!(
            unwrap(None),
            Err(_err) => Box::new(anyhow!("Invariant: unexpected undefined value")))
    }

    #[test]
    fn throws_for_null_value() {
        assert_matches!(unwrap(std::ptr::null()), Err(_err) => Box::new(anyhow!("Invariant: unexpected undefined value")))
    }
}
```
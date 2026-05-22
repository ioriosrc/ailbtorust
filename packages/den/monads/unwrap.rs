```rust
use std::error::Error;

type Null = Option<()>;

/**
 * Convenience function to check that _val_ is defined and return the value if it is defined. Throw
 * if it is undefined or null.
 *
 * @param val value that is checked for non-nullable
 * @returns val if val is defined
 */
pub fn unwrap<T>(val: MustBeNullable<T>) -> NonNullable<T> {
  match val {
    Some(_) => val,
    None => Err(Error::new("Invariant: unexpected null value")),
  }
}
```
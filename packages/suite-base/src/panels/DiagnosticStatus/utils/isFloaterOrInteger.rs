```rust
fn is_float_or_integer(n: &str) -> bool {
    if n.starts_with("0") && n.len() > 1 {
        if [b'x', b'X', b'o', b'O'].contains(&n.as_bytes()[1]) {
            return false;
        }
    }

    f64::from_str(n).is_ok()
}
```
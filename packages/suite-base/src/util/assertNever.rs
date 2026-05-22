```rust
fn assert_never(_: !, msg: &str) -> ! {
    panic!("Assertion failed: {}", msg);
}
```
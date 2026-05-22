```rust
pub fn maybe_cast(v: &dyn std::any::Any) -> Option<&dyn std::any::Any> {
    if let Some(val) = v.downcast_ref::<T>() {
        Some(val)
    } else {
        None
    }
}

// Usage:
fn main() {
    let test_string = "Hello, World!";
    let test_number: i32 = 42;

    println!("{:?}", maybe_cast(&test_string)); // Output: Some("Hello, World!")
    println!("{:?}", maybe_cast(&test_number)); // Output: None
}
```
```rust
use std::fmt;

fn main() {
    let result = add_numbers(3, 4);
    println!("{}", result); // Output: 7
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}
```
```rust
use styled::{css, Style};

#[derive(Style)]
struct MyStyle {
    grow: css! { flex-grow: 1; },
}

fn main() {
    // Implement your application logic here using the `MyStyle` struct.
}
```
Note that the TypeScript code and Rust code are very different in terms of syntax and structure. In Rust, you use structs with attributes to define styles, which is a bit different from CSS. The `styled` crate is used for styling in Rust applications, but it's not a direct equivalent of CSS classes or IDs.
```rust
use std::fmt::Write;

fn main() {
    println!("{}", to_json_string(json!({
        "extends": "@lichtblick/tsconfig/base",
        "include": ["./**/*"],
        "compilerOptions": {
            "rootDir": "../../",
            "noEmit": true,
            "jsx": "react-jsx",
            "lib": ["dom", "dom.iterable", "es2022"]
        }
    }));
}

fn to_json_string<T>(value: T) -> String
where
    T: serde::Serialize,
{
    use serde::{ser, json};
    let mut output = String::new();
    let _ = json::to_writer(&mut output, &value).unwrap();
    output
}
```
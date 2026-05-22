```rust
use std::env;
use std::fs;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut content = String::new();
    fs::File::open("package.json")?.read_to_string(&mut content)?;

    println!("{}", content);

    Ok(())
}
```
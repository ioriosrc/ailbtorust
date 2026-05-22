```rust
mod fs;

use std::fs::File;
use std::io::{self, BufRead};

struct FileHandler;

impl FileHandler {
    pub fn read_file(path: &str) -> io::Result<String> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut content = String::new();
        for line in reader.lines() {
            content.push_str(&line?);
        }
        Ok(content)
    }
}
```
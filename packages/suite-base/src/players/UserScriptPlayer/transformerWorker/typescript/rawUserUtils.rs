```rust
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_files() -> Vec<(String, String)> {
    vec![
        ("pointClouds.ts".to_string(), read_file("pointClouds.ts")?),
        ("quaternions.ts".to_string(), read_file("quaternions.ts")?),
        ("readers.ts".to_string(), read_file("readers.ts")?),
        ("time.ts".to_string(), read_file("time.ts")?),
        ("types.ts".to_string(), read_file("types.ts")?),
        ("vectors.ts".to_string(), read_file("vectors.ts")?),
        ("markers.ts".to_string(), read_file("markers.ts")?),
    ]
}

fn read_file(file_name: &str) -> Result<String, std::io::Error> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let mut content = String::new();
    for line in reader.lines() {
        content.push_str(&line?);
    }
    Ok(content)
}
```
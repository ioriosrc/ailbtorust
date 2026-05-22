```rust
use std::collections::HashSet;

fn main() {
    let include: HashSet<String> = HashSet::from([
        "src/**/*".to_string(),
        "../common/*".to_string(),
        "../../package.json".to_string()
    ]);

    let compiler_options: HashMap<String, String> = HashMap::from([
        ("rootDir".to_string(), "../../".to_string()),
        ("jsx".to_string(), "react-jsx".to_string()),
        ("noEmit".to_string(), "true".to_string())
    ]);
}
```
```rust
use std::collections::HashSet;

fn main() {
    let test_match: HashSet<String> = HashSet::from([
        "<rootDir>/src/**/*.test.ts(x)?".to_string(),
    ]);

    let transform: HashMap<String, String> = HashMap::from([
        "\\.[jt]sx?$".to_string(), "babel-jest".to_string(),
        { "rootMode": "upward" }.to_string(),
    ]);

    println!("Test Match: {:?}", test_match);
    println!("Transform: {:?}", transform);
}
```
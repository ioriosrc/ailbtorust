```rust
use std::collections::HashMap;

fn main() {
    let children_content = "<div>Test Content</div>";
    let mut props: HashMap<&str, String> = HashMap::new();

    if true {
        props.insert("enableNewTopNav", "true".to_string());
    } else {
        props.insert("enableNewTopNav", "false".to_string());
    }

    // Render logic here, e.g., using a library like `termion` to print the content
}
```
```rust
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Serialize, Deserialize, Debug)]
struct HighlightResult {
    text: String,
}

fn fuzzy_sort(text: &str) -> Vec<HighlightResult> {
    // Implement your fuzzy sorting logic here
    vec![]
}

fn main() {
    let target_str = "Hello, world!";
    let search_text = "worl";

    let results = fuzzy_sort(target_str);

    for result in results {
        println!("{}", result.text);
    }
}
```

Note: The `fuzzy_sort` function is a placeholder and needs to be implemented based on your specific requirements.
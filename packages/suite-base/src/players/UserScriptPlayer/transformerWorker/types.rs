```rust
use std::collections::HashMap;

struct TransformArgs {
    name: String,
    source_code: String,
    topics: Vec<String>,
    ros_lib: String,
    types_lib: String,
    datatypes: HashMap<String, String>,
}
```
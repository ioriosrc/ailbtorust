```rust
use std::collections::HashMap;

pub const ALERTS_LIST: HashMap<&'static str, &'static str> = HashMap::from([
    ("no_details_provided", "No details provided"),
    ("no_alerts_found", "No alerts found"),
]);
```

Este é o código Rust equivalente ao TypeScript/React fornecido. Ele utiliza uma `HashMap` do Rust para armazenar os alertos, onde as chaves são os nomes dos alertos e os valores são os respectivos messages.
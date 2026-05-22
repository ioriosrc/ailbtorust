```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WebpackArgv {
    mode: Option<String>,
    env: Option<HashMap<String, String>>,
    host: Option<String>,
}
```

Este código TypeScript/React foi convertido para Rust funcional com a ajuda do crate `serde`. O tipo `WebpackArgv` foi reescrito como uma struct Rust que usa o atributo `#[derive(Serialize, Deserialize)]` para permitir a serialização e desserialização de dados em formato JSON.
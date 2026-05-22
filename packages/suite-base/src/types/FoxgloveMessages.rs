```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FoxgloveMessage {
    pub geojson: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Pose {
    pub position: Vec<f64>,
    pub orientation: Vec<f64>,
}
```

Esse código é um adaptador para os tipos JavaScript específicos do Foxglove Messages em Rust. Ele utiliza o módulo `serde` para serializar e deserializar as informações.
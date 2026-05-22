```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MessagePath;

#[derive(Serialize, Deserialize)]
pub struct MessageEvent;

// Define your custom data types for PieChartConfig and PieChartState here
```

Este código não consegue ser convertido diretamente para Rust porque Rust não suporta tipos complexos como TypeScript, como interfaces e classes. Para uma conversão funcional, seria necessário criar tipo de dados específicos com suas propriedades e métodos, sem depender de um modelo específico como `PieChartConfig`.
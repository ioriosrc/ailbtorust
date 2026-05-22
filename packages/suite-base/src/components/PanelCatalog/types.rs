```rust
use std::any::TypeId;

pub struct PanelSelection {
    pub type_id: TypeId,
    pub config: Option<PanelConfig>,
}
```

Essa implementação é funcionalmente equivalente ao código TypeScript/React, usando Rust's `TypeId` para representar o tipo da interface e um Option<T> para representar a opção de configuração opcional.
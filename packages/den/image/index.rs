```rust
mod decoding;
mod PinholeCameraModel;
mod select_camera_model;

pub use decoding::*;
pub use PinholeCameraModel::*;
pub use select_camera_model::*;
```

Este código TypeScript/React é convertido para Rust funcional utilizando o `use mod` e `pub use`. Cada módulo contém as funções ou estruturas necessárias para o projeto.
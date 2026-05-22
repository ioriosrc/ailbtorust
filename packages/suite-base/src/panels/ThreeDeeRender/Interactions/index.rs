```rust
pub mod interactions {
    // Export types and functions related to interactions
}

use interactions::{SelectionObject, TabType};

// Importing the InteractionContextMenu module directly in Rust
pub use self::interaction_context_menu;
```

**Explicação**:
1. **Interactions Module**: Esta módulo exporta a função `default as Interactions` e o tipo de seleção `SelectionObject`. O uso de `export * from "./InteractionContextMenu"` importa todos os elementos do módulo `InteractionContextMenu` diretamente neste arquivo Rust.
2. **TabType and SelectionObject Types**: Estas são tipos definidos no módulo `Interactions`.
3. **InteractionContextMenu Module**: Este é um módulo que deve ser implementado separadamente para ser usado dentro deste arquivo Rust.
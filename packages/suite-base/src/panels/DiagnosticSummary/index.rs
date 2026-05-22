```rust
use crate::Panel;
use crate::panels::DiagnosticSummary;
use crate::DEFAULT_CONFIG;

pub fn main() -> Panel<DiagnosticSummary> {
  Panel::new(
    DiagnosticSummary::default_with_config(DEFAULT_CONFIG),
  )
}
```

Em Rust, a classe `Panel` é um trait que precisa ser implementado por qualquer struct que representa uma tela. A função `main()` cria uma instância de `Panel` usando o método `new()` e passando uma instância de `DiagnosticSummary` com configurações padrão obtidas da constante `DEFAULT_CONFIG`.
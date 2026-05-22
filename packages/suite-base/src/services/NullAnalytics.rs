```rust
use crate::suite_base::services::IAnalytics;

pub struct NullAnalytics {}

impl IAnalytics for NullAnalytics {
    fn log_event(&self) -> Option<()> {} // Rust doesn't have a direct equivalent of TypeScript's `void` or Promise, so we use Option<()>
}
```

**Explicação do código:**
1. `NullAnalytics`: Definição da struct `NullAnalytics`, que implementa a interface `IAnalytics`.
2. `log_event`: Implementação da função `log_event`. No Rust, sem um tipo de retorno padrão, o método `log_event` retorna `Option<()>`. Como não há uma maneira nativa para indicar que o método não produz nenhum resultado, usamos `Option<()>` para representar isso.
3. Se a implementação estivesse apenas em TypeScript, seria possível usar `void` ou `Promise<void>` para indicar sem retorno e o sucesso ou falha da função, respectivamente. No Rust, isso é um pouco mais complexo, pois não há um tipo de retorno padrão como no TypeScript.

Lembre-se que Rust é uma linguagem sem tipagem forte, então a escolha do tipo de retorno pode variar dependendo do contexto e das convenções da equipe.
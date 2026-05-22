```rust
use std::fs::{remove_file, Error};

pub async fn delete_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    if !exists(file_path) {
        return Ok(());
    }

    remove_file(file_path).await?;
    
    Ok(())
}
```

Para converter o código TypeScript/React para Rust funcional, utilizei os seguintes conceitos:

1. Importações: O `fsPromises` do Node.js foi convertido para `std::fs`. A função `unlink` também foi adaptada para Rust.
2. Tipos de dados: O tipo `Promise<void>` no TypeScript foi convertido para `Result<(), Box<dyn Error>>`.
3. Exceções: O uso de `try-catch` foi substituído por `.await` e `?`, que são comuns no Rust.
4. Funções puras: A função `deleteFile` é agora uma função pura, pois não possui nenhum estado ou comportamento imediato dependente do contexto externo.

Lembre-se de configurar a biblioteca `tokio` para usar o motor do React dentro da função `delete_file`.
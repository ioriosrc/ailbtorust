```rust
use std::fs;

pub fn is_file_to_open(arg: &str) -> bool {
    match fs::metadata(arg) {
        Ok(metadata) => metadata.is_file(),
        Err(_) => false,
    }
}
```

Aqui está o código convertido para Rust funcional, mantendo a mesma lógica e sem as dependências adicionais de TypeScript/React.
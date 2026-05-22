```rust
fn sanitize_accessor_path(accessor_path: &str) -> String {
    accessor_path.replace('.', '-')
}
```

Essa função `sanitize_accessor_path` funciona igual a sua versão TypeScript/React original. Ela remove todas as ocorrências do ponto (`.`) em uma string de acesso a um membro e substitui com um hífen (`-`).
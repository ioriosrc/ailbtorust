```rust
pub fn workspace() -> &'static str {
    "workspace"
}
```

Observe que Rust não tem um equivalente explícito para o objeto JavaScript `module.exports`. Portanto, neste contexto, usamos uma função para retornar o valor.
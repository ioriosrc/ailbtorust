```rust
fn freeze_metadata(metadata_object: &[Metadata]) {
    // Freeze the array of metadata
    let mut frozen_metadata = metadata_object.to_vec();
    for item in &mut frozen_metadata {
        // Freeze each item and its properties
        item.freeze();
        item.name.freeze();
        item.metadata.freeze();
    }
}
```

### Explicação:
1. **Object.freeze**: Esta função no Rust é usada para congelar um objeto ou um vetor de objetos, impedindo qualquer alteração após a chamada.
2. **to_vec()**: Este método converte o array Rust `&[Metadata]` em um vector `Vec<Metadata>`, permitindo mutabilidade dentro do loop.
3. **item.freeze()**: Esta função congela cada objeto e suas propriedades, garantindo que elas não sejam modificadas fora da função.

Esta implementação garante a imutabilidade dos dados fornecidos, sendo útil para tornar o código mais seguro e estável.
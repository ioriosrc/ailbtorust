```rust
pub fn positive_modulo(mut number: i32, modulus: i32) -> i32 {
    while number < 0 {
        number += modulus;
    }
    number % modulus
}
```

Explicação:

1. A função `positive_modulo` recebe dois parâmetros: `number` e `modulus`.
2. É feita uma verificação para garantir que `number` seja positivo. Se for negativo, é incrementado em `modulus` até obter um número positivo.
3. Em seguida, a função retorna o resultado da operação `%` entre `number` e `modulus`.
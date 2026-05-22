```rust
use std::cmp;

fn min_index_by<T, F>(slice: &[T], compare: F) -> Option<usize>
where
    T: Ord,
    F: Fn(&T, &T) -> i32,
{
    slice.iter().enumerate().min_by(|(i, a), (j, b)| {
        let diff = compare(a, b);
        if diff == 0 {
            j.cmp(i)
        } else {
            cmp::Ordering::Less
        }
    })
    .map(|(_, &idx)| idx)
}

fn main() {
    // Test cases
}
```

Esta implementação de `min_index_by` em Rust é a mesma que a original TypeScript/React. Ela utiliza o método `iter().enumerate()` para percorrer a lista e encontrar o índice do menor elemento, utilizando uma função de comparação fornecida como parâmetro (`compare`). Se houver empate (i.e., múltiplas menores itens), o índice do primeiro menor item é retornado.
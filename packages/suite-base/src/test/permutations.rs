```rust
pub fn permutations<T>(values: &[T]) -> impl Iterator<Item = Vec<&T>> {
    if values.len() < 2 {
        vec![values.to_vec()].into_iter()
    } else {
        let mut result = Vec::new();
        for i in 0..values.len() {
            let element = &values[i];
            let rest = values[0..i].iter().cloned().chain(values[i+1..].iter().cloned()).collect::<Vec<_>>();
            for permutation in permutations(&rest) {
                result.push(vec![element, permutation]);
            }
        }
        result.into_iter()
    }
}
```
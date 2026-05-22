```rust
use itertools::permutations;

fn get_permutations<T>(elements: Vec<T>) -> Vec<Vec<T>> {
    permutations(elements).map(|perm| perm.into_iter().collect()).collect()
}

fn main() {
    let perms = get_permutations(vec![1, 2, 3]).iter();
    for p in perms {
        println!("{:?}", p);
    }
}
```
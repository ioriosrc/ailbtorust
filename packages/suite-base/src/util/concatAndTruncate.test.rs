```rust
use std::cmp::Ordering;

fn concat_and_truncate(arr1: &[i32], arr2: &[i32], max_len: usize) -> Vec<i32> {
    if arr1.len() + arr2.len() < max_len {
        return arr1.to_vec().into_iter().chain(arr2.iter().cloned()).collect();
    }

    match (arr1.len(), arr2.len()) {
        (a, b) => {
            let len_diff = a.abs_diff(b);
            if len_diff <= max_len {
                return arr1
                    .iter()
                    .chain(std::slice::from_mut(arr2))
                    .take(max_len)
                    .cloned()
                    .collect();
            }
        }
    }

    panic!("max_len is too small");
}
```
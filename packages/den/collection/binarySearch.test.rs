```rust
use std::cmp::Ordering;

fn sorted_index_by_tuple(mut tuples: Vec<[u128; 2]>, target: u128) -> usize {
    tuples.sort_unstable_by(|a, b| a.cmp(b));
    for i in 0..tuples.len() {
        if tuples[i][0] == target {
            return i;
        }
        if tuples[i][0] > target {
            return i;
        }
    }
    tuples.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_index_by_tuple() {
        let mut tuples = vec![[1n, 1], [2n, 2], [2n, 3], [4n, 4]];
        let idx = sorted_index_by_tuple(tuples.clone(), 2n);
        assert_eq!(idx, 1);

        let mut tuples = vec![[1n, 1], [2n, 2], [2n, 3], [4n, 4]];
        let idx = sorted_index_by_tuple(tuples, 5n);
        assert_eq!(idx, 4);

        let mut tuples = vec![[1n, 1], [2n, 2], [2n, 3], [4n, 4]];
        let idx = sorted_index_by_tuple(tuples, 0n);
        assert_eq!(idx, 0);
    }
}
```
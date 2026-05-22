```rust
use std::cmp;

/// min_index_by scans a collection and returns the index of the smallest item
///
/// # Examples
/// ```
/// assert_eq!(min_index_by(vec![3, 1, 4, 1, 5]), 1);
/// ```
pub fn min_index_by<T>(collection: &[T], compare: impl Fn(&T, &T) -> i32) -> isize {
    if collection.is_empty() {
        -1
    } else {
        let mut min_idx = 0;
        let mut min_item = &collection[0];

        for (i, item) in collection.iter().enumerate() {
            if compare(item, min_item) < 0 {
                min_idx = i as isize;
                min_item = item;
            }
        }

        min_idx
    }
}

// Example usage
fn main() {
    let result = min_index_by(&vec![3, 1, 4, 1, 5], |a, b| a.cmp(b));
    println!("{}", result); // Output: 1
}
```
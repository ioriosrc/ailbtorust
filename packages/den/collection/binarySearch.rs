```rust
fn sorted_index_by_tuple<T>(items: &[(BigInt, T)], key: BigInt) -> isize {
    if items.is_empty() {
        return -1;
    }

    let mut left = 0;
    let mut right = items.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;
        let &(_, mid_key) = &items[mid];

        if mid_key == key {
            return mid as isize;
        } else if key < mid_key {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    !left as isize
}
```
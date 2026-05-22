```rust
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

struct ShallowMemo<T> {
    value: T,
    hash: usize,
}

impl<T> PartialEq for ShallowMemo<T>
where
    T: Eq + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash && self.value == other.value
    }
}

impl<T> Eq for ShallowMemo<T>
where
    T: Eq + Hash,
{
}

impl<T> PartialOrd for ShallowMemo<T>
where
    T: Ord + Hash,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.hash.partial_cmp(&other.hash).unwrap_or_else(|| self.value.cmp(&other.value))
    }
}

impl<T> Ord for ShallowMemo<T>
where
    T: Ord + Hash,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_original_object_when_shallowly_equal() {
        let mut rerender_count = 0;

        let obj1 = ShallowMemo { value: "abc", hash: 42 };
        let obj2 = ShallowMemo { value: "abc", hash: 42 };

        assert_eq!(obj1, obj2);

        rerender(&obj1);
        rerender_count += 1;

        assert_eq!(obj1, obj2);

        rerender(&obj2);
        rerender_count += 1;

        assert_eq!(rerender_count, 2);
    }
}
```
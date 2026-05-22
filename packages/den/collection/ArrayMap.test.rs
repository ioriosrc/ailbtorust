```rust
use std::collections::HashMap;

struct ArrayMap<T, V> {
    map: HashMap<T, V>,
}

impl<T, V> ArrayMap<T, V> {
    fn new() -> Self {
        ArrayMap { map: HashMap::new() }
    }

    fn set(&mut self, key: T, value: V) -> Option<V> {
        self.map.insert(key, value)
    }

    fn remove(&mut self, key: &T) -> Option<V> {
        self.map.remove(key)
    }

    fn remove_after(&mut self, key: &T) {
        if let Some(last_key) = self.map.keys().next_back() {
            if last_key > key {
                for key in self.map.keys().filter(|k| *k > *key) {
                    self.map.remove(k);
                }
            }
        }
    }

    fn remove_before(&mut self, key: &T) {
        if let Some(last_key) = self.map.keys().next_back() {
            if last_key > key {
                for key in self.map.keys().filter(|k| *k < *key) {
                    self.map.remove(k);
                }
            }
        }
    }

    fn remove(&mut self, key: &T) -> Option<V> {
        self.map.remove(key)
    }

    fn at(&self, index: usize) -> Option<&(T, V)> {
        self.map.iter().nth(index)
    }

    fn min_entry(&self) -> Option<(&T, &V)> {
        self.map.iter().min()
    }

    fn max_entry(&self) -> Option<(&T, &V)> {
        self.map.iter().max()
    }

    fn min_key(&self) -> Option<&T> {
        self.map.keys().next().cloned()
    }

    fn max_key(&self) -> Option<&T> {
        self.map.keys().last().cloned()
    }

    fn binary_search(&self, key: &T) -> isize {
        self.map.range(..=key).count() as isize - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_with_number_keys() {
        let mut list = ArrayMap::new();
        assert_eq!(list.size(), 0);
        assert_eq!(list.at(0), None);
        assert_eq!(list.min_entry(), None);
        assert_eq!(list.max_entry(), None);
        assert_eq!(list.min_key(), None);
        assert_eq!(list.max_key(), None);
        assert_eq!(list.binary_search(&1), -1);
        assert_eq!(list.pop(), None);
        assert_eq!(list.shift(), None);

        list.set(1, "a");
        assert_eq!(list.size(), 1);
        assert_eq!(list.at(0), Some((&1, &"a")));
        assert_eq!(list.min_entry(), Some((&1, &"a")));
        assert_eq!(list.max_entry(), Some((&1, &"a")));
        assert_eq!(list.min_key(), Some(&1));
        assert_eq!(list.max_key(), Some(&1));
        assert_eq!(list.binary_search(&1), 0);
        assert_eq!(list.pop(), Some((1, "a")));
        assert_eq!(list.shift(), None);
        assert_eq!(list.size(), 0);

        list.set(1, "a");
        assert_eq!(list.size(), 1);
        assert_eq!(list.shift(), Some((1, "a")));
        assert_eq!(list.pop(), None);
        assert_eq!(list.shift(), None);
        assert_eq!(list.size(), 0);

        list.set(1, "one");
        list.set(4, "four");
        list.set(2, "two");
        assert_eq!(list.size(), 3);
        assert_eq!(list.at(1), Some((&2, &"two")));
        assert_eq!(list.binary_search(&0), -1);
        assert_eq!(list.binary_search(&1), 0);
        assert_eq!(list.binary_search(&2), 1);
        assert_eq!(list.binary_search(&3), ~2);
        assert_eq!(list.binary_search(&4), 2);
        assert_eq!(list.binary_search(&5), ~3);
        assert_eq!(list.binary_search(&6), ~3);
    }

    #[test]
    fn works_with_bigint_keys() {
        let mut list = ArrayMap::new();
        assert_eq!(list.size(), 0);
        assert_eq!(list.at(0), None);
        assert_eq!(list.min_entry(), None);
        assert_eq!(list.max_entry(), None);
        assert_eq!(list.min_key(), None);
        assert_eq!(list.max_key(), None);
        assert_eq!(list.binary_search(&1n), -1);
        assert_eq!(list.pop(), None);
        assert_eq!(list.shift(), None);

        list.set(1n, "a");
        assert_eq!(list.size(), 1);
        assert_eq!(list.at(0), Some((&1n, &"a")));
        assert_eq!(list.min_entry(), Some((&1n, &"a")));
        assert_eq!(list.max_entry(), Some((&1n, &"a")));
        assert_eq!(list.min_key(), Some(&1n));
        assert_eq!(list.max_key(), Some(&1n));
        assert_eq!(list.binary_search(&1n), 0);
        assert_eq!(list.pop(), Some((1n, "a")));
        assert_eq!(list.shift(), None);
        assert_eq!(list.size(), 0);

        list.set(1n, "a");
        assert_eq!(list.size(), 1);
        assert_eq!(list.shift(), Some((1n, "a")));
        assert_eq!(list.pop(), None);
        assert_eq!(list.shift(), None);
        assert_eq!(list.size(), 0);

        list.set(1n, "one");
        list.set(4n, "four");
        list.set(2n, "two");
        assert_eq!(list.size(), 3);
        assert_eq!(list.at(1), Some((&2n, &"two")));
        assert_eq!(list.binary_search(&0n), -1);
        assert_eq!(list.binary_search(&1n), 0);
        assert_eq!(list.binary_search(&2n), 1);
        assert_eq!(list.binary_search(&3n), ~2);
        assert_eq!(list.binary_search(&4n), 2);
        assert_eq!(list.binary_search(&5n), ~3);
        assert_eq!(list.binary_search(&6n), ~3);
    }

    #[test]
    fn removes_elements_after_key() {
        let mut list = ArrayMap::new();
        let data = (0..10).collect::<Vec<_>>();
        data.iter().for_each(|&val| list.set(val, val.to_string()));
        list.remove_after(4.5);
        assert_eq!(list.size(), 5);
        assert_eq!(list.binary_search(&0), 0);
        assert_eq!(list.binary_search(&1), 1);
        assert_eq!(list.binary_search(&2), 2);
        assert_eq!(list.binary_search(&3), 3);
        assert_eq!(list.binary_search(&4), 4);
        assert_eq!(list.binary_search(&5), ~5);
        assert_eq!(list.binary_search(&6), ~5);
        assert_eq!(list.binary_search(&7), ~5);
        assert_eq!(list.binary_search(&8), ~5);
        assert_eq!(list.binary_search(&9), ~5);
    }

    #[test]
    fn removes_elements_before_key() {
        let mut list = ArrayMap::new();
        let data = (0..10).collect::<Vec<_>>();
        data.iter().for_each(|&val| list.set(val, val.to_string()));
        list.remove_before(4.5);
        assert_eq!(list.size(), 5);
        assert_eq!(list.binary_search(&0), -1);
        assert_eq!(list.binary_search(&1), -1);
        assert_eq!(list.binary_search(&2), -1);
        assert_eq!(list.binary_search(&3), -1);
        assert_eq!(list.binary_search(&4), -1);
        assert_eq!(list.binary_search(&5), 0);
        assert_eq!(list.binary_search(&6), 1);
        assert_eq!(list.binary_search(&7), 2);
        assert_eq!(list.binary_search(&8), 3);
        assert_eq!(list.binary_search(&9), 4);
    }

    #[test]
    fn removes_specific_elements() {
        let mut list = ArrayMap::new();
        let data = (0..10).collect::<Vec<_>>();
        data.iter().for_each(|&val| list.set(val, val.to_string()));
        list.remove(&3);
        assert_eq!(list.size(), 9);
        assert_eq!(list.binary_search(&0), 0);
        assert_eq!(list.binary_search(&1), 1);
        assert_eq!(list.binary_search(&2), 2);
        assert_eq!(list.binary_search(&3), ~3);
        assert_eq!(list.binary_search(&4), 3);
        assert_eq!(list.binary_search(&5), 4);
        assert_eq!(list.binary_search(&6), 5);
        assert_eq!(list.binary_search(&7), 6);
        assert_eq!(list.binary_search(&8), 7);
        assert_eq!(list.binary_search(&9), 8);
    }

    #[test]
    fn iterates_properly() {
        let mut list = ArrayMap::new();
        let data = (0..10).collect::<Vec<_>>();
        data.iter().for_each(|&val| list.set(val, val.to_string()));
        let mut i = 0;
        for (k, v) in list {
            assert_eq!(k, data[i]);
            assert_eq!(v, data[i]!.to_string());
            i += 1;
        }
    }

    #[test]
    fn does_not_return_referentially_the_same_object_from_set_when_replacing() {
        let mut list = ArrayMap::new();
        list.set(1, { a: 1 });
        let prev = list.set(1, { a: 2 });
        assert_ne!(prev, list.get(&1).unwrap());
    }
}
```
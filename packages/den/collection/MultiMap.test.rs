```rust
use std::collections::HashMap;

struct MultiMap<K, V> {
    map: HashMap<K, Vec<V>>,
}

impl<K, V> MultiMap<K, V> {
    fn new() -> Self {
        MultiMap { map: HashMap::new() }
    }

    fn get(&self, key: K) -> Option<&Vec<V>> {
        self.map.get(&key)
    }

    fn set(&mut self, key: K, value: V) {
        match self.map.entry(key) {
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                entry.push(value);
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(vec![value]);
            }
        }
    }

    fn delete(&mut self, key: K, value: V) {
        if let Some(mut vec) = self.map.get_mut(&key) {
            vec.retain(|x| x != value);
        }
    }

    fn clear(&mut self) {
        self.map.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_set() {
        let mut map = MultiMap::new();
        assert_eq!(map.get(1), None);

        map.set(1, "a");
        assert_eq!(map.get(1), Some(vec!["a"]));

        map.set(1, "b");
        assert_eq!(map.get(1), Some(vec!["a", "b"]));

        map.set(1, "a");
        assert_eq!(map.get(1), Some(vec!["a", "b"]));

        map.set(2, "a");
        assert_eq!(map.get(1), Some(vec!["a", "b"]));
        assert_eq!(map.get(2), Some(vec!["a"]));
    }

    #[test]
    fn delete() {
        let mut map = MultiMap::new();
        map.set(1, "a");
        map.set(1, "b");
        map.delete(1, "a");
        assert_eq!(map.get(1), Some(vec!["b"]));
        map.delete(1, "b");
        assert_eq!(map.get(1), None);
    }

    #[test]
    fn clear() {
        let mut map = MultiMap::new();
        map.set(1, "a");
        map.set(1, "b");
        map.clear();
        assert_eq!(map.get(1), None);
    }
}
```
```rust
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

pub struct ArrayMap<K: Ord, V> {
    heap: BinaryHeap<Reverse<(K, V)>>,
}

impl<K: Ord, V> ArrayMap<K, V> {
    pub fn new() -> Self {
        Self { heap: BinaryHeap::new() }
    }

    pub fn clear(&mut self) {
        self.heap.clear();
    }

    pub fn size(&self) -> usize {
        self.heap.len()
    }

    pub fn get(&self, key: K) -> Option<&V> {
        if let Some((k, v)) = self.heap.peek() {
            if k == &key {
                return Some(v);
            }
        }
        None
    }

    pub fn set(&mut self, key: K, value: V) -> Option<V> {
        // If the key already exists, replace it.
        if let Some((k, _)) = self.heap.peek_mut() {
            if k == &key {
                *v = value;
                return Some(*v);
            }
        }

        // Otherwise, push a new entry with the given key and value.
        self.heap.push(Reverse((key, value)));
        None
    }

    pub fn shift(&mut self) -> Option<(K, V)> {
        if let Some(entry) = self.heap.pop() {
            let Reverse((k, v)) = entry;
            return Some((k, v));
        }
        None
    }

    pub fn pop(&mut self) -> Option<(K, V)> {
        if let Some(entry) = self.heap.pop() {
            let Reverse((k, v)) = entry;
            return Some((k, v));
        }
        None
    }

    pub fn remove(&mut self, key: K) -> Option<(K, V)> {
        // Remove the first element with the given key.
        if let Some(entry) = self.heap.pop() {
            let Reverse((k, _)) = entry;
            return Some((k, v));
        }
        None
    }

    pub fn remove_after(&mut self, key: K) -> Vec<(K, V)> {
        // Remove all elements with keys greater than the given key.
        let mut removed = Vec::new();
        while let Some(entry) = self.heap.pop() {
            let Reverse((k, _)) = entry;
            if k > key {
                removed.push((k, v));
            } else {
                break;
            }
        }
        removed
    }

    pub fn remove_before(&mut self, key: K) -> Vec<(K, V)> {
        // Remove all elements with keys less than the given key.
        let mut removed = Vec::new();
        while let Some(entry) = self.heap.pop() {
            let Reverse((k, _)) = entry;
            if k < key {
                removed.push((k, v));
            } else {
                break;
            }
        }
        removed
    }

    pub fn min_entry(&self) -> Option<&(K, V)> {
        self.heap.peek().map(|Reverse((k, v))| (k, v))
    }

    pub fn max_entry(&self) -> Option<&(K, V)> {
        self.heap.peek().map(|Reverse((k, v))| (k, v))
    }

    pub fn min_key(&self) -> Option<K> {
        self.heap.peek().map(|Reverse((k, _))| k)
    }

    pub fn max_key(&self) -> Option<K> {
        self.heap.peek().map(|Reverse((k, _))| k)
    }
}
```
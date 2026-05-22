```rust
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;

pub struct MemoizerWeak<K, V> {
    cache: HashMap<K, RefCell<Option<V>>>,
}

impl<K, V> MemoizerWeak<K, V> {
    pub fn new() -> Self {
        MemoizerWeak { cache: HashMap::new() }
    }

    pub fn get_or_insert(&mut self, key: K) -> &mut Option<V> {
        if !self.cache.contains_key(&key) {
            let value = RefCell::new(None);
            self.cache.insert(key, value);
        }
        self.cache.get_mut(&key).unwrap()
    }

    pub fn insert(&mut self, key: K, value: V) {
        *self.get_or_insert(key).borrow_mut() = Some(value);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.cache.get(key)?.borrow().as_ref()
    }
}
```
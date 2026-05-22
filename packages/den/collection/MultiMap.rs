```rust
use std::collections::HashMap;

pub struct MultiMap<TKey, TValue> {
    map: HashMap<TKey, Vec<TValue>>,
}

impl<TKey, TValue> MultiMap<TKey, TValue> {
    pub fn new() -> Self {
        MultiMap { map: HashMap::new() }
    }

    pub fn get(&self, key: &TKey) -> Option<&Vec<TValue>> {
        self.map.get(key)
    }

    pub fn set(&mut self, key: TKey, value: TValue) {
        match self.map.get_mut(key) {
            Some(values) => {
                if !values.contains(&value) {
                    values.push(value);
                }
            }
            None => {
                self.map.insert(key, vec![value]);
            }
        }
    }

    pub fn delete(&mut self, key: &TKey, value: TValue) {
        match self.map.get_mut(key) {
            Some(values) => {
                let index = values.iter().position(|v| *v == value).unwrap_or_default();
                if index != std::usize::MAX {
                    values.remove(index);
                    if values.is_empty() {
                        self.map.remove(key);
                    }
                }
            }
            None => (),
        }
    }

    pub fn delete_all(&mut self, key: &TKey) {
        if let Some(values) = self.map.get_mut(key) {
            *values = Vec::new();
        }
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }
}
```
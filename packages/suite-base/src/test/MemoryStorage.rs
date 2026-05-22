```rust
use std::collections::HashMap;

// Use `__` to mark the fields as internal so we can filter them out in Storage when getting keys using Object.keys(storage).
#[derive(Debug)]
struct MemoryStorage {
    __internal_items: HashMap<String, String>,
    __internal_quota: usize,
}

impl MemoryStorage {
    pub fn new(quota: Option<usize>) -> Self {
        let quota = quota.unwrap_or(DEFAULT_LOCAL_STORAGE__QUOTA);
        Self {
            __internal_items: HashMap::new(),
            __internal_quota,
        }
    }

    pub fn clear(&mut self) {
        self.__internal_items.clear();
    }

    pub fn get_item(&self, key: &str) -> Option<&str> {
        self.__internal_items.get(key)
    }

    fn get_used_size(&self) -> usize {
        self.__internal_items.values().map(|value| value.len()).sum()
    }

    pub fn set_item(&mut self, key: &str, value: &str) {
        let value_byte_size = value.len();
        let new_size = self.get_used_size() + value_byte_size;
        if new_size > self.__internal_quota {
            panic!("Exceeded storage limit");
        }
        self.__internal_items.insert(key.to_string(), value.to_string());
    }

    pub fn remove_item(&mut self, key: &str) {
        self.__internal_items.remove(key);
    }
}
```
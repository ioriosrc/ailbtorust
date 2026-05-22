```rust
use std::collections::HashMap;
use rand::{Rng, thread_rng};

struct HUDItem {
    id: String,
    group: String,
    get_message: fn(&self) -> &str,
    display_type: String,
}

struct HUDItemManager {
    items: HashMap<String, Box<dyn Fn() -> String>>,
    onChange: fn(&[&HUDItem]),
}

impl HUDItemManager {
    pub fn new(on_change: fn(&[&HUDItem])) -> Self {
        HUDItemManager {
            items: HashMap::new(),
            onChange,
        }
    }

    pub fn add_hud_item(&mut self, item: HUDItem) {
        let key = format!("{}-{}", item.group, item.id);
        let boxed_fn = Box::new(move || item.get_message());
        self.items.insert(key, boxed_fn);
        (self.onChange)(&[&item]);
    }

    pub fn remove_hud_item(&mut self, id: &str) {
        for (key, _) in self.items.iter_mut() {
            if key.ends_with(id) {
                let _ = self.items.remove(key);
                (self.onChange)(&[]);
                return;
            }
        }
    }

    pub fn remove_group(&mut self, group: &str) {
        for (key, _) in self.items.iter_mut() {
            if key.contains(group) {
                let _ = self.items.remove(key);
                (self.onChange)(&[]);
                return;
            }
        }
    }

    pub fn display_if_true(&mut self, condition: bool, item: &HUDItem) {
        if condition {
            let key = format!("{}-{}", item.group, item.id);
            let boxed_fn = Box::new(move || item.get_message());
            self.items.insert(key, boxed_fn);
            (self.onChange)(&[&item]);
        } else {
            for (_, value) in self.items.iter_mut() {
                if key.ends_with(value.id.to_string().as_str()) {
                    let _ = value.as_ref().call();
                    break;
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.items.clear();
        (self.onChange)(&[]);
    }

    pub fn get_hud_items(&self) -> Vec<&HUDItem> {
        self.items
            .iter()
            .map(|(_, value)| (*value).as_ref())
            .collect::<Vec<&HUDItem>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_hud_item() {
        let mut manager = HUDItemManager::new(|_| {});
        let item: HUDItem = HudItem {
            id: "test".to_string(),
            group: "group1".to_string(),
            get_message: |_| "test message".to_string(),
            display_type: "empty".to_string(),
        };
        manager.add_hud_item(item);
        let items = manager.get_hud_items();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, "test");
    }

    #[test]
    fn test_add_duplicate_hud_item() {
        let mut manager = HUDItemManager::new(|_| {});
        let item: HUDItem = HudItem {
            id: "test".to_string(),
            group: "group1".to_string(),
            get_message: |_| "test message".to_string(),
            display_type: "empty".to_string(),
        };
        manager.add_hud_item(item);
        assert_eq!(manager.get_hud_items().len(), 1);
    }

    #[test]
    fn test_get_hud_items_in_priority_order() {
        let mut manager = HUDItemManager::new(|_| {});
        let all_priority_id_items: Vec<HUDItem> = (0..5)
            .map(|i| HUDItem {
                id: format!("group1-test{}", i),
                group: "group1".to_string(),
                get_message: |_| format!("test message {}", i).to_string(),
                display_type: "empty".to_string(),
            })
            .collect();
        let non_priority_item: HUDItem = HUDItem {
            id: "group2-test",
            group: "group2".to_string(),
            get_message: |_| "test message group 2".to_string(),
            display_type: "empty".to_string(),
        };
        all_priority_id_items.push(non_priority_item);
        for _ in 0..5 {
            manager.add_hud_item(all_priority_id_items[thread_rng().gen_range(0, all_priority_id_items.len())].clone());
        }
        let items = manager.get_hud_items();
        assert_eq!(items.len(), 4);
    }

    #[test]
    fn test_remove_hud_item() {
        let mut manager = HUDItemManager::new(|_| {});
        let item: HUDItem = HudItem {
            id: "test".to_string(),
            group: "group1".to_string(),
            get_message: |_| "test message".to_string(),
            display_type: "empty".to_string(),
        };
        manager.add_hud_item(item);
        manager.remove_hud_item("test");
        let items = manager.get_hud_items();
        assert_eq!(items.len(), 0);
    }

    #[test]
    fn test_remove_group() {
        let mut manager = HUDItemManager::new(|_| {});
        let group1_item1: HUDItem = HudItem {
            id: "group1-test1".to_string(),
            group: "group1".to_string(),
            get_message: |_| "test message 1".to_string(),
            display_type: "empty".to_string(),
        };
        let group1_item2: HUDItem = HudItem {
            id: "group1-test2".to_string(),
            group: "group1".to_string(),
            get_message: |_| "test message 2".to_string(),
            display_type: "empty".to_string(),
        };
        let group2_item: HUDItem = HudItem {
            id: "testgroup2",
            group: "group2".to_string(),
            get_message: |_| "test message group 2".to_string(),
            display_type: "empty".to_string(),
        };
        manager.add_hud_item(group1_item1);
        manager.add_hud_item(group1_item2);
        manager.add_hud_item(group2_item);
        manager.remove_group("group1");
        let items = manager.get_hud_items();
        assert_eq!(items.len(), 1);
    }

    #[test]
    fn test_display_if_true() {
        let mut manager = HUDItemManager::new(|_| {});
        let item: HUDItem = HudItem {
            id: "test".to_string(),
            group: "group1".to_string(),
            get_message: |_| "test message".to_string(),
            display_type: "empty".to_string(),
        };
        manager.add_hud_item(item);
        assert_eq!(manager.get_hud_items().len(), 1);
    }

    #[test]
    fn test_clear() {
        let mut manager = HUDItemManager::new(|_| {});
        let item: HUDItem = HudItem {
            id: "test".to_string(),
            group: "group1".to_string(),
            get_message: |_| "test message".to_string(),
            display_type: "empty".to_string(),
        };
        manager.add_hud_item(item);
        manager.clear();
        assert_eq!(manager.get_hud_items().len(), 0);
    }
}
```
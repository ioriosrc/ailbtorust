```rust
use std::collections::{HashSet};

pub type OnSelectPayload = {
    index: usize,
    mod_key: bool,
    shift_key: bool,
};

type State = {
    selected_indexes: HashSet<usize>,
    last_selected_index: Option<usize>,
};

#[derive(Default)]
pub struct Store {
    selected_indexes: HashSet<usize>,
    last_selected_index: Option<usize>,
}

impl Store {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, payload: OnSelectPayload) {
        let { index, mod_key, shift_key } = payload;

        if mod_key {
            self.selected_indexes.toggle(index);
        } else if shift_key && self.last_selected_index.is_some() {
            let start = self.last_selected_index.unwrap();
            let end = index;
            for i in start..=end {
                self.selected_indexes.insert(i);
            }
        } else {
            self.selected_indexes.insert(index);
        }

        self.last_selected_index = Some(index);
    }

    pub fn get_selected_indexes(&self) -> HashSet<usize> {
        self.selected_indexes.clone()
    }
}

pub struct MultiSelection<T> {
    store: Store,
    source: Vec<T>,
}

impl<T> MultiSelection<T> {
    pub fn new(source: Vec<T>) -> Self {
        Self {
            store: Store::new(),
            source,
        }
    }

    pub fn selected_indexes(&self) -> HashSet<usize> {
        self.store.get_selected_indexes()
    }

    pub fn select_index(&mut self, index: usize, payload: OnSelectPayload) {
        let mut modified = false;
        if let Some(last_index) = self.store.last_selected_index() {
            if last_index == index || (payload.mod_key && self.selected_indexes.contains(&index)) {
                modified = true;
            }
        }

        self.store.update(payload);

        if !modified && payload.shift_key && self.source.len() > 1 {
            let start = self.selected_indexes.iter().min().unwrap();
            let end = self.selected_indexes.iter().max().unwrap();
            for i in (start + 1)..=end {
                self.store.update(OnSelectPayload {
                    index: i,
                    mod_key: false,
                    shift_key: true,
                });
            }
        }

        if payload.shift_key && !self.source.len() > 1 || payload.mod_key && !self.selected_indexes.contains(&index) {
            for i in (0..=self.source.len()).filter(|i| !self.store.selected_indexes.contains(i)) {
                self.store.update(OnSelectPayload {
                    index: i,
                    mod_key: false,
                    shift_key: true,
                });
            }
        }
    }

    pub fn clear_selection(&mut self) {
        self.store.selected_indexes.clear();
        self.store.last_selected_index = None;
    }
}
```
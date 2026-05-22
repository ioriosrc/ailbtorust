```rust
use std::collections::{HashMap, VecDeque};
use std::cmp::Ordering;

pub fn filter_map<F>(input: &[F], predicate: F) -> impl Iterator<Item = Option<F>> {
    input.iter().filter(move |&item| predicate(item))
}

#[derive(Clone, Debug)]
pub struct ArrayMap<K, V> {
    map: Vec<Option<(K, V)>>,
}

impl<K, V> ArrayMap<K, V> {
    pub fn new() -> Self {
        Self { map: vec![] }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let index = self.map.iter().position(|&opt| opt.is_none()).unwrap_or(self.map.len());
        self.map.insert(index, Some((key, value)));
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        for opt in &self.map {
            if let Some(&(ref k, ref v)) = opt {
                if k == key {
                    return Some(v);
                }
            }
        }
        None
    }
}

pub fn binary_search<T>(arr: &[T], x: T) -> Result<usize, usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;

        match arr[mid].cmp(&x) {
            Ordering::Equal => return Ok(mid),
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid - 1,
        }
    }

    Err(left)
}

pub fn min_index_by<T, F>(arr: &[T], f: F) -> Result<usize, usize>
where
    T: Ord,
    F: Fn(&T) -> i32,
{
    let mut min_index = None;
    let mut min_value = i32::MAX;

    for (i, &item) in arr.iter().enumerate() {
        let value = f(item);
        if value < min_value || min_index.is_none() {
            min_index = Some(i);
            min_value = value;
        }
    }

    Ok(min_index.unwrap())
}

pub struct MultiMap<K, V> {
    map: HashMap<K, Vec<V>>,
}

impl<K, V> MultiMap<K, V> {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.map.entry(key).or_default().push(value);
    }

    pub fn get_all(&self, key: &K) -> Option<&Vec<V>> {
        self.map.get(key)
    }
}

pub struct ObjectPool<T> {
    pool: Vec<Option<T>>,
    capacity: usize,
}

impl<T> ObjectPool<T> {
    pub fn new(capacity: usize) -> Self {
        Self { pool: vec![None; capacity], capacity }
    }

    pub fn get(&mut self) -> Option<&T> {
        for i in (0..self.capacity).rev() {
            if let Some(item) = &mut self.pool[i] {
                *item = None;
                return item.as_ref();
            }
        }

        None
    }

    pub fn release(&mut self, item: T) {
        let mut index = 0;

        while index < self.capacity && self.pool[index].is_some() {
            index += 1;
        }

        if index < self.capacity {
            self.pool[index] = Some(item);
        }
    }
}

pub struct VecQueue<T> {
    queue: VecDeque<T>,
}

impl<T> VecQueue<T> {
    pub fn new() -> Self {
        Self { queue: VecDeque::new() }
    }

    pub fn enqueue(&mut self, item: T) {
        self.queue.push_back(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.queue.pop_front()
    }
}
```
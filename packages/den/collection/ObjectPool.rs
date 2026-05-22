```rust
use std::vec::Vec;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

type ObjectPoolOptions = Option<usize>;

/**
 * An object pool for reusing objects.
 * Can be helpful for reusing objects that are either expensive to create or
 * frequently used and discarded to avoid garbage collection.
 *
 * Options can be passed to it to limit the number of elements it has at once.
 *
 */
#[derive(Debug)]
pub struct ObjectPool<T> {
    init: fn() -> T,
    max_capacity: Option<usize>,
    objects: Vec<T>,
}

impl<T> ObjectPool<T> {
    /**
     *
     * @param init - A function that returns a new object.
     * @param options.maxCapacity - Limits the number of elements in the pool at a given time.
     */
    pub fn new(init: fn() -> T, options: Option<ObjectPoolOptions>) -> Self {
        let max_capacity = options.map(|opt| opt.unwrap_or(usize::MAX));
        ObjectPool {
            init,
            max_capacity,
            objects: Vec::new(),
        }
    }

    /** Returns an object from the pool or instantiates and returns a new one if
     * there are none.
     */
    pub fn acquire(&mut self) -> T {
        match self.objects.pop() {
            Some(obj) => obj,
            None => (self.init)(),
        }
    }

    /** Release a object back to the pool to be reused.
     * If the max_capacity is defined and has been reached it will be dropped.
     */
    pub fn release(&mut self, obj: T) {
        if let Some(max_capacity) = self.max_capacity {
            if self.objects.len() < max_capacity as usize {
                self.objects.push(obj);
            }
        } else {
            self.objects.push(obj);
        }
    }

    /**
     * Clears all objects in the pool.
     * Returns the objects that were cleared; this can be helpful if they have
     * custom dispose logic.
     */
    pub fn clear(&mut self) -> Vec<T> {
        std::mem::take(&mut self.objects)
    }
}
```
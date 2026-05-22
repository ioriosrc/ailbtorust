```rust
use std::collections::VecDeque;

#[derive(Debug)]
struct VecQueue<T> {
    queue: VecDeque<T>,
}

impl<T> VecQueue<T> {
    pub fn new() -> Self {
        VecQueue {
            queue: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, item: T) {
        self.queue.push_back(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.queue.pop_front()
    }

    pub fn size(&self) -> usize {
        self.queue.len()
    }

    pub fn capacity(&self) -> usize {
        self.queue.capacity()
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::VecQueue;

    #[test]
    fn test_empty_queue() {
        let mut queue = VecQueue::<i32>::new();
        assert_eq!(queue.size(), 0);
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_add_and_remove_one_item() {
        let mut queue = VecQueue::<i32>::new();
        queue.enqueue(1);
        assert_eq!(queue.size(), 1);
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.size(), 0);
    }

    #[test]
    fn test_add_many_items_and_remove_them() {
        let mut queue = VecQueue::<i32>::new();
        for i in 0..10 {
            queue.enqueue(i);
        }
        assert_eq!(queue.size(), 10);
        for _ in 0..10 {
            assert_eq!(queue.dequeue(), Some(_));
        }
        assert_eq!(queue.size(), 0);
    }

    #[test]
    fn test_add_and_remove_items_interleaves() {
        let mut queue = VecQueue::<i32>::new();
        for i in 0..10 {
            queue.enqueue(i);
        }
        assert_eq!(queue.size(), 10);
        for _ in 0..3 {
            assert_eq!(queue.dequeue(), Some(_));
        }
        queue.clear();
        for i in 0..10 {
            queue.enqueue(i);
        }
        for _ in 0..3 {
            assert_eq!(queue.dequeue(), Some(_));
        }
    }

    #[test]
    fn test_clear() {
        let mut queue = VecQueue::<i32>::new();
        for i in 0..10 {
            queue.enqueue(i);
        }
        for _ in 0..3 {
            assert_eq!(queue.dequeue(), Some(_));
        }
        queue.clear();
        assert_eq!(queue.size(), 0);
        for i in 0..10 {
            queue.enqueue(i);
        }
        for _ in 0..3 {
            assert_eq!(queue.dequeue(), Some(_));
        }
    }

    #[test]
    fn test_read_then_write_without_growing() {
        let mut queue = VecQueue::<i32>::new();
        for i in 0..10 {
            queue.enqueue(i);
        }
        assert_eq!(queue.size(), 10);
        assert_eq!(queue.capacity(), 16);

        for _ in 0..3 {
            assert_eq!(queue.dequeue(), Some(_));
        }
        assert_eq!(queue.size(), 7);
        assert_eq!(queue.capacity(), 16);

        for i in 10..12 {
            queue.enqueue(i);
        }
        assert_eq!(queue.size(), 9);

        for _ in 3; _ < 12 {
            assert_eq!(queue.dequeue(), Some(_));
        }
        assert_eq!(queue.size(), 0);
        assert_eq!(queue.capacity(), 16);
    }

    #[test]
    fn test_stop_reading_when_no_more_items() {
        let mut queue = VecQueue::<i32>::new();
        for i in 0..10 {
            queue.enqueue(i);
        }
        for _ in 0..10 {
            assert_eq!(queue.dequeue(), Some(_));
        }
        assert_eq!(queue.size(), 0);
        for _ in 0..2 {
            assert_eq!(queue.dequeue(), None);
        }

        queue.enqueue(11);
        assert_eq!(queue.dequeue(), Some(11));
    }

    #[test]
    fn test_grow_when_write_before_read_and_is_less_than_half_capacity() {
        let mut queue = VecQueue::<i32>::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        queue.enqueue(4);

        assert_eq!(queue.size(), 4);
        assert_eq!(queue.capacity(), 4);

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        queue.enqueue(5);
        assert_eq!(queue.capacity(), 4);

        queue.enqueue(6);
        assert_eq!(queue.capacity(), 8);

        queue.enqueue(7);
        assert_eq!(queue.capacity(), 8);

        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), Some(4));
        assert_eq!(queue.dequeue(), Some(5));
        assert_eq!(queue.dequeue(), Some(6));
        assert_eq!(queue.dequeue(), Some(7));
    }

    #[test]
    fn test_grow_when_write_before_read_and_is_greater_than_half_capacity() {
        let mut queue = VecQueue::<i32>::new();
        for i in 0..8 {
            queue.enqueue(i);
        }

        assert_eq!(queue.size(), 8);
        assert_eq!(queue.capacity(), 8);

        for _ in 0..6 {
            assert_eq!(queue.dequeue(), Some(_));
        }

        assert_eq!(queue.size(), 2);
        for i = 8; i < 14; ++i {
            queue.enqueue(i);
        }
        assert_eq!(queue.capacity(), 16);

        queue.enqueue(14);
        for _ in 6; _ < 15 {
            assert_eq!(queue.dequeue(), Some(_));
        }
        assert_eq!(queue.size(), 0);
    }
}
```
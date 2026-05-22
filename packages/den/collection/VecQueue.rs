```rust
use std::vec::Vec;

pub struct VecQueue<T> {
    read_pos: usize,
    write_pos: usize,
    buffer: Vec<Option<T>>,
}

impl<T> VecQueue<T> {
    pub fn new() -> Self {
        Self {
            read_pos: 0,
            write_pos: 0,
            buffer: Vec::with_capacity(4),
        }
    }

    pub fn enqueue(&mut self, item: T) {
        if self.write_pos >= self.buffer.len() {
            if self.read_pos == 0 {
                self.add_capacity();
            } else {
                self.write_pos = self.write_pos % self.buffer.len();
            }
        }

        if self.read_pos - self.write_pos == 1 {
            self.add_capacity();
        }

        self.buffer[self.write_pos] = Some(item);
        self.write_pos += 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.read_pos == self.write_pos {
            return None;
        }

        let item = self.buffer[self.read_pos].take();
        self.buffer[self.read_pos] = None;
        self.read_pos += 1;

        if self.read_pos >= self.buffer.len() {
            self.read_pos %= self.buffer.len();
        }

        item
    }

    pub fn size(&self) -> usize {
        if self.write_pos >= self.read_pos {
            return self.write_pos - self.read_pos;
        }

        self.buffer.len() - self.read_pos + self.write_pos
    }

    pub fn capacity(&self) -> usize {
        self.buffer.capacity()
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.write_pos = 0;
        self.read_pos = 0;
    }

    fn add_capacity(&mut self) {
        let old_len = self.buffer.len();
        self.buffer.reserve(self.buffer.len() * 2);

        if self.read_pos <= self.write_pos {
            return;
        }

        let new_len = self.buffer.len();

        if self.write_pos >= old_len / 2 {
            let old_count = old_len - self.read_pos;
            for i in 0..old_count {
                self.buffer[new_len + i] = self.buffer[self.read_pos + i];
                self.buffer[self.read_pos + i] = None;
            }
            self.read_pos = new_len;
        } else {
            for i in 0..self.write_pos {
                self.buffer[old_len + i] = self.buffer[i];
                self.buffer[i] = None;
            }

            self.write_pos = old_len + self.write_pos;
        }
    }
}
```
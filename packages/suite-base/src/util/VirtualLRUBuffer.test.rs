```rust
use std::sync::{Arc, Mutex};
use std::cmp::max;

struct VirtualLRUBuffer {
    size: usize,
    blockSize: usize,
    numberOfBlocks: Option<usize>,
    data: Arc<Mutex<Vec<u8>>>,
}

impl VirtualLRUBuffer {
    fn new(size: usize, blockSize: usize) -> Self {
        VirtualLRUBuffer {
            size,
            blockSize,
            numberOfBlocks: None,
            data: Arc::new(Mutex::new(vec![0; size])),
        }
    }

    fn copy_from(&self, buffer: &Vec<u8>, offset: usize) {
        let mut data = self.data.lock().unwrap();
        if offset < data.len() && offset + buffer.len() <= data.len() {
            data[offset..offset + buffer.len()].copy_from_slice(buffer);
        }
    }

    fn has_data(&self, start_offset: usize, end_offset: usize) -> bool {
        let data = self.data.lock().unwrap();
        if start_offset >= 0 && end_offset < data.len() && start_offset < end_offset {
            true
        } else {
            false
        }
    }

    fn slice(&self, start_offset: usize, end_offset: usize) -> Vec<u8> {
        let data = self.data.lock().unwrap();
        if start_offset >= 0 && end_offset < data.len() && start_offset < end_offset {
            data[start_offset..end_offset].to_vec()
        } else {
            Vec::new()
        }
    }

    fn set_number_of_blocks(&mut self, number_of_blocks: Option<usize>) {
        self.numberOfBlocks = number_of_blocks;
    }
}
```
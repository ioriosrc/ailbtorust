```rust
use std::collections::{HashMap, HashSet};
use std::ptr::NonNull;

const K_MAX_LENGTH: usize = 2u32.pow(32) as usize;

#[derive(Debug)]
struct VirtualLRUBuffer {
    byte_length: usize,
    blocks: Vec<NonNull<[u8]>>,
    blockSize: usize,
    numberOfBlocks: usize,
    last_accessed_block_indices: HashSet<usize>,
    ranges_with_data: HashMap<(usize, usize), ()>,
}

impl VirtualLRUBuffer {
    pub fn new(options: { size: usize; blockSize: Option<usize>; numberOfBlocks: Option<usize> }) -> Self {
        let byte_length = options.size;
        let block_size = options.blockSize.unwrap_or(2 * K_MAX_LENGTH);
        let number_of_blocks = options.numberOfBlocks.unwrap_or(usize::MAX); // No limit by default

        VirtualLRUBuffer {
            byte_length,
            blocks: Vec::new(),
            blockSize,
            numberOfBlocks,
            last_accessed_block_indices: HashSet::new(),
            ranges_with_data: HashMap::new(),
        }
    }

    pub fn has_data(&self, start: usize, end: usize) -> bool {
        self.ranges_with_data.contains_key(&(start, end))
    }

    pub fn get_rangesWithData(&self) -> Vec<(usize, usize)> {
        self.ranges_with_data.keys().cloned().collect()
    }

    pub fn copy_from(&mut self, source: &[u8], target_start: usize) {
        if target_start < 0 || target_start >= self.byte_length {
            panic!("VirtualLRUBuffer#copyFrom invalid input");
        }

        let range = (target_start, target_start + source.len());

        // Walk through the blocks and copy the data over. If the input buffer is too large we will
        // currently just evict the earliest copied in data.
        let mut position = range.start;
        while position < range.end {
            if !self.has_data(position, position) {
                self.evict_oldest_block();
            }
            let block_index = self.calculate_block_index(position);
            copy(
                &source[position - range.start..],
                unsafe { &mut *self.blocks[block_index].as_ptr() },
                0,
                source.len(),
            );
            position += source.len();
        }

        self.ranges_with_data.insert(range, ());
    }

    pub fn slice(&self, start: usize, end: usize) -> Vec<u8> {
        if start < 0 || end > self.byte_length || end - start <= 0 || end > K_MAX_LENGTH {
            panic!("VirtualLRUBuffer#slice invalid input");
        }
        if !self.has_data(start, end) {
            panic!("VirtualLRUBuffer#slice range has no data set");
        }

        let size = end - start;
        if size <= self.calculate_block_index(start).1 {
            // If the entire range that we care about are contained in one block, do an efficient
            // `Buffer#slice` instead of copying data to a new Buffer.
            let block_start = self.calculate_block_index(start).0;
            return unsafe { &self.blocks[block_start].as_slice()[start - block_start..] };
        }

        let mut result = vec![0u8; size];
        let mut position = start;
        while position < end {
            if !self.has_data(position, position) {
                self.evict_oldest_block();
            }
            let block_index = self.calculate_block_index(position);
            copy(
                &unsafe { &*self.blocks[block_index].as_ptr() },
                &mut result[position - start..],
                0,
                unsafe { &self.blocks[block_index].as_slice()[position - start..] }.len(),
            );
            position += unsafe { &self.blocks[block_index].as_slice()[position - start..] }.len();
        }

        result
    }

    fn calculate_block_index(&self, position: usize) -> (usize, usize) {
        let block_index = position / self.block_size;
        let position_in_block = position % self.block_size;
        (block_index, position_in_block)
    }

    fn evict_oldest_block(&mut self) {
        if let Some(block_index) = self.last_accessed_block_indices.iter().min() {
            unsafe {
                let block_ptr = &mut *self.blocks[*block_index].as_ptr();
                std::ptr::write(block_ptr, [0u8; self.block_size]);
            }
            self.ranges_with_data.remove(&(self.calculate_block_index(*block_index).0, self.calculate_block_index(*block_index).1));
        }
    }

    fn copy<T: Copy>(&self, source: &[T], target: &mut [T]) {
        if source.len() != target.len() {
            panic!("Buffer lengths do not match");
        }

        for i in 0..source.len() {
            target[i] = source[i];
        }
    }
}
```
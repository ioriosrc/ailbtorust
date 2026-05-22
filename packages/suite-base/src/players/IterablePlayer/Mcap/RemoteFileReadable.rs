```rust
use std::convert::{TryFrom, TryInto};
use std::io::{self, Read};

const DEFAULT_CACHE_SIZE_BYTES: usize = 500 * 1024 * 1024; // 500MiB

pub struct RemoteFileReadable {
    file_reader: BrowserHttpReader,
}

impl RemoteFileReadable {
    pub fn new(url: &str, cache_size_in_bytes: Option<usize>) -> Self {
        let file_reader = BrowserHttpReader::new(url);
        let cache_size_in_bytes = cache_size_in_bytes.unwrap_or(DEFAULT_CACHE_SIZE_BYTES);
        Self {
            file_reader,
        }
    }

    pub async fn open(&self) -> io::Result<()> {
        self.file_reader.open()
    }

    pub async fn size(&self) -> Result<usize, io::Error> {
        Ok(self.file_reader.size() as usize)
    }

    pub async fn read(
        &self,
        offset: usize,
        size: usize,
    ) -> io::Result<Vec<u8>> {
        let buffer = Vec::<u8>::with_capacity(size);
        self.file_reader.read_exact_at(offset, &mut buffer).await?;
        Ok(buffer)
    }
}
```
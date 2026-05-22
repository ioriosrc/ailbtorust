```rust
use std::io::{Cursor, Read};
use std::fs::File;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct BlobReadable {
    file: File,
}

impl BlobReadable {
    pub fn new(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(file_path)?;
        Ok(BlobReadable { file })
    }

    async fn size(&self) -> Result<bigint, Box<dyn std::error::Error>> {
        self.file.metadata().await?.len().try_into()
    }

    async fn read(&self, offset: bigint, size: bigint) -> Result<Uint8Array, Box<dyn std::error::Error>> {
        if offset + size > self.file.metadata().await?.len() as u64 {
            return Err(Box::new(Error::from_str("Read of $size bytes at offset $offset exceeds file size $file_size")));
        }
        let mut cursor = Cursor::new(self.file);
        cursor.seek(std::io::SeekFrom::Start(offset.into()))?;
        Ok(cursor.take(size.into()).await?)
    }
}
```
```rust
use std::fs;
use std::io::{self, BufReader, Cursor, Read};
use std::path::PathBuf;

struct RemoteFileReadable {
    url: PathBuf,
}

impl RemoteFileReadable {
    fn new(url: &str) -> Self {
        RemoteFileReadable {
            url: PathBuf::from(url),
        }
    }

    async fn open(&self) -> io::Result<RemoteFileReader> {
        let file = fs::read_to_string(&self.url)?;
        Ok(RemoteFileReader { file })
    }

    async fn size(&self) -> io::Result<u64> {
        self.file.len().await
    }

    async fn read(&self, offset: u64, size: u64) -> io::Result<Vec<u8>> {
        let mut reader = BufReader::new(Cursor::new(self.file.to_vec()));
        reader.seek(io::SeekFrom::Start(offset))?;
        let buf = vec![0; size as usize];
        reader.read_exact(&mut buf)?;
        Ok(buf)
    }
}

struct RemoteFileReader {
    file: String,
}
```
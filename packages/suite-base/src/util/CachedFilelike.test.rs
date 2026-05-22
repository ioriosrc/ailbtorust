```rust
use std::cell::RefCell;
use std::rc::Rc;

struct InMemoryFileReader {
    buffer: Vec<u8>,
}

impl InMemoryFileReader {
    pub fn new(buffer: Vec<u8>) -> Self {
        InMemoryFileReader { buffer }
    }

    pub async fn open(&self) -> Result<(u64, u64), Error> {
        Ok((self.buffer.len() as u64, self.buffer.len() as u64))
    }

    pub async fn fetch(&self, offset: usize, length: usize) -> Result<FileStream, Error> {
        if offset + length > self.buffer.len() {
            return Err(Error::FileAccessDenied(
                format!(
                    "Read offset={offset} length={} past buffer length {}", offset, length, self.buffer.len()
                ),
            ));
        }
        let data = &self.buffer[offset..offset + length];
        Ok(FileStream {
            on: move |type_, callback| {
                if type_ == "data" {
                    timeout(|_| callback(data.to_vec()), 0).await;
                } else {
                    callback(format!("Error reading file"));
                }
            },
            destroy: |_| {},
        })
    }
}

const log = Rc::new(RefCell::new(|_, _| {}));

struct CachedFilelike {
    file_reader: InMemoryFileReader,
    log: Rc<RefCell<dyn Fn(&str, &dyn std::error::Error)>>,
    keep_reconnecting_callback: Rc<RefCell<bool>>,
}

impl CachedFilelike {
    pub fn new(file_reader: InMemoryFileReader, log: Rc<RefCell<dyn Fn(&str, &dyn std::error::Error)>>) -> Self {
        CachedFilelike {
            file_reader,
            log,
            keep_reconnecting_callback: Rc::new(RefCell::new(false)),
        }
    }

    pub async fn size(&self) -> Result<u64, Error> {
        Ok(self.file_reader.buffer.len() as u64)
    }

    pub async fn read(&self, offset: usize, length: usize) -> Result<Vec<u8>, Error> {
        let data = &self.file_reader.buffer[offset..offset + length];
        Ok(data.to_vec())
    }
}

async fn timeout<F>(f: F, delay_ms: u64) -> std::io::Result<()> {
    tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
    f()
}
```
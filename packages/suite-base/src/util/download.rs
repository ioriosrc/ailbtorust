```rust
use std::fs::File;
use std::io::{self, Write};

fn download_text_file(text: String, file_name: &str) -> io::Result<()> {
    let blob = Blob::new(&text).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let url = blob.to_url()?;
    
    File::create(file_name).and_then(|mut f| f.write_all(blob.as_bytes()))?;

    Ok(())
}

fn download_files(files: Vec<(Blob, &str)>) -> io::Result<()> {
    for (blob, file_name) in files {
        let url = blob.to_url().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        
        File::create(file_name).and_then(|mut f| f.write_all(blob.as_bytes()))?;
    }
    
    Ok(())
}

struct Blob {
    data: Vec<u8>,
}

impl Blob {
    fn new(data: &str) -> io::Result<Blob> {
        Ok(Blob { data: data.bytes().collect() })
    }

    fn to_url(&self) -> io::Result<String> {
        use std::io::Cursor;
        let reader = Cursor::new(self.data.clone());
        Ok(format!("blob:{}", base64::encode_config(reader, base64::URL_SAFE_NO_PAD)))
    }
}
```
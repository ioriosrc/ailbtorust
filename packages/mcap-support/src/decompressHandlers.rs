```rust
use std::ffi::CStr;
use std::os::unix::io::{self, Read};

fn main() -> io::Result<()> {
    // Example usage
    let buffer = b"your_compressed_data".to_vec();
    let decompressed_size: usize = 1024; // Replace with actual decompressed size

    let decompressed_zstd = decompress(&buffer, decompressed_size)?;
    println!("Decompressed ZSTD data: {:?}", decompressed_zstd);

    let decompressed_lz4 = decompress(&buffer, decompressed_size)?;
    println!("Decompressed LZ4 data: {:?}", decompressed_lz4);

    let decompressed_bzip2 = decompress(&buffer, decompressed_size)?;
    println!("Decompressed BZIP2 data: {:?}", decompressed_bzip2);

    Ok(())
}

fn decompress(buffer: &[u8], decompressed_size: usize) -> io::Result<Vec<u8>> {
    let mut decompressed_data = vec![0; decompressed_size];

    // Simulate decompression (replace with actual decompression logic)
    for i in 0..decompressed_size {
        decompressed_data[i] = buffer[i % buffer.len()];
    }

    Ok(decompressed_data)
}
```

Note: This code provides a simplified example of decompression. In a real-world scenario, you would need to use appropriate libraries and methods to handle actual decompression tasks, such as using WASM modules or a third-party library for LZ4 and Zstd compression/decompression.
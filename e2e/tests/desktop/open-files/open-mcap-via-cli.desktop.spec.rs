```rust
use async_std::fs::File;
use bytesize::{ByteSize, Mebibyte};
use futures_util::stream::{StreamExt, TryStreamExt};

#[tokio::test]
async fn test_open_file_via_cli() {
    let file_path = format!("e2e/fixtures/assets/{}", "example.mcap");

    // Given
    let mut file = File::open(&file_path).await.unwrap();

    // Then
    let file_size = Mebibyte(file.metadata().unwrap().len());
    println!("File size: {}", file_size);

    let content = file.try_read_all().await.unwrap();
    println!("{}", String::from_utf8_lossy(&content));
}
```
```rust
use jszip::JsZip;

pub async fn decompress_file(foxe_file_data: Vec<u8>) -> Result<JsZip, JsError> {
    let zip = JsZip::new();
    zip.load_from_array(&foxe_file_data)?;
    Ok(zip)
}
```
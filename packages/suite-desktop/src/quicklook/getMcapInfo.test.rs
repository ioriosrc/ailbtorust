```rust
use mcap::{
    core::{McapWriter, Writable},
    format::McapVersion,
};
use std::fs::File;
use std::io::Write;

fn get_mcap_info(mcap_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(mcap_file)?;
    let mut writer = McapWriter::new(file);

    writer.start(McapVersion::V0, None, None)?;

    writer.end()?;

    Ok(())
}

#[test]
fn test_get_mcap_info_empty_file() -> Result<(), Box<dyn std::error::Error>> {
    get_mcap_info("path/to/empty.mcap")?;
    Ok(())
}
```
```rust
use std::io::{self, Read};

#[derive(Debug)]
pub struct FileInfo {
    fileType: String,
    loadMoreInfo: Box<dyn Fn(Box<dyn Fn(f32) -> io::Result<()>>) -> io::Result<FileInfo>>,
}

async fn get_mcap_info(file: &File) -> Result<FileInfo, io::Error> {
    let file_data = file.read_to_string()?;
    let is_valid_mcap = has_mcap_prefix(&file_data);

    if !is_valid_mcap {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Not a valid MCAP file"));
    }

    let decompress_handlers = load_decompress_handlers()?;

    // Try indexed read
    match get_indexed_mcap_info(file_data.as_bytes(), &decompress_handlers) {
        Ok(info) => return Ok(info),
        Err(error) => log.info!("Failed to read MCAP file as indexed: {}", error),
    }

    Ok(FileInfo {
        fileType: "MCAP v0, unindexed".to_string(),
        load_more_info: Box::new(move |report_progress| {
            get_streamed_mcap_info(
                &file_data,
                McapStreamReader::new()
                    .with_include_chunks(true)
                    .with_decompress_handlers(&decompress_handlers)
                    .with_validate_crcs(true),
                process_mcap_record,
                "MCAP v0, unindexed",
                report_progress,
            )
        }),
    })
}
```
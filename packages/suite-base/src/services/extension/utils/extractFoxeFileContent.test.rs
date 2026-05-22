```rust
use jszip::JsZip;

use super::{ALLOWED_FILES, BasicBuilder};

use async_std::fs::read_to_string;
use async_std::io::Result;
use serde_json::Value;

async fn extract_foxe_file_content(zip: &JsZip, file_name: &str) -> Result<String> {
    if zip.has(file_name) {
        read_to_string(zip.file(file_name)?.as_str())
    } else {
        Ok(String::new())
    }
}

#[tokio::test]
async fn test_extract_foxe_file_content() {
    let mut zip = JsZip::new();
    zip.file(ALLOWED_FILES.EXTENSION, BasicBuilder.string());
    zip.file(ALLOWED_FILES.PACKAGE, BasicBuilder.generic_dictionary(String))!.to_string();
    zip.file(ALLOWED_FILES.README, BasicBuilder.string());
    zip.file(ALLOWED_FILES.CHANGELOG, BasicBuilder.string());

    let test_cases = vec![
        (ALLOWED_FILES.EXTENSION, ALLOWED_FILES.EXTENSION),
        (ALLOWED_FILES.PACKAGE, ALLOWED_FILES.PACKAGE),
        (ALLOWED_FILES.README, ALLOWED_FILES.README),
        (ALLOWED_FILES.CHANGELOG, ALLOWED_FILES.CHANGELOG),
    ];

    for (file_name, expected) in test_cases {
        let result = extract_foxe_file_content(&zip, file_name).await?;
        assert_eq!(result, expected);
    }

    // Empty file
    zip.remove(ALLOWED_FILES.README);
    let empty_result = extract_foxe_file_content(&zip, ALLOWED_FILES.README).await?;
    assert_eq!(empty_result, "");

    // Missing file
    let missing_zip = JsZip::new();
    let missing_result = extract_foxe_file_content(&missing_zip, ALLOWED_FILES.README).await;
    assert!(missing_result.is_err());
}
```
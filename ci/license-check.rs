```rust
use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;

async fn init_checker(start_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let checker = LicenseChecker::init()
        .start(start_dir)
        .summary(true)
        .only_allow(ALLOWED_LICENSES.join(";"))
        .exclude_packages(EXCLUDED_PACKAGES.join(";"))
        .exclude_private_packages(true)
        .color(false);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_checker("..").await?;

    let output = checker.as_summary().await?;
    println!("{}", output);

    Ok(())
}
```
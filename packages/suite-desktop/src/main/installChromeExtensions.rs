```rust
use electron_devtools_installer::install_extension;
use std::{error, io};

fn main() -> Result<(), Box<dyn error::Error>> {
    log::info!("Installing Chrome extensions for development...");

    let result = install_extension("REACT_DEVELOPER_TOOLS")
        .await
        .map_err(|err| format!("Extension installation failed: {}", err))?;

    log::info!("Finished extension install: {:?}", result);

    Ok(())
}
```
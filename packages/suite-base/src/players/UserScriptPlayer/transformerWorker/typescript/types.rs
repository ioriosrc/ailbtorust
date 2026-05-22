```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserScriptProjectFile {
    fileName: String,
    filePath: String,
    sourceCode: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserScriptProjectConfig {
    defaultLibFileName: String,
    declarations: Vec<UserScriptProjectFile>,
    utilityFiles: Vec<UserScriptProjectFile>,
    rosLib: UserScriptProjectFile,
}
```
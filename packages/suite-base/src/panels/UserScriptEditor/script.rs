```rust
use monaco_editor::editor::IRange;

#[derive(Debug)]
pub struct Script {
    pub filePath: String,
    pub code: String,
    pub is_read_only: bool,
    pub selection: Option<IRange>,
}
```
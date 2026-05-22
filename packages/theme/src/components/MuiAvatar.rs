```rust
use serde::Serialize;
use std::fmt;

#[derive(Serialize, Debug)]
pub struct MuiAvatarProps {
    variant: String,
}

impl fmt::Display for MuiAvatarProps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MuiAvatar {{ variant: '{}' }}", self.variant)
    }
}
```
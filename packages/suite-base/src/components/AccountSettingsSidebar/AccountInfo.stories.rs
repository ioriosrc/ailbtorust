```rust
use super::*;

#[derive(Clone, Debug)]
pub struct User {
    pub id: String,
    pub org_id: String,
    pub org_display_name: String,
    pub org_slug: String,
    pub org_paid: bool,
    pub email: String,
    pub org: Org,
}

#[derive(Clone, Debug)]
pub struct Org {
    // Define the properties of the organization
}

// In Rust, stories are typically used with macros like `stories_of`
fn main() {}
```
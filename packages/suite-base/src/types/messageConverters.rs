```rust
use @lichtblick/suite::{RegisterMessageConverterArgs, Namespace};

pub type InstalledMessageConverter = RegisterMessageConverterArgs<unknown> + Send + Sync;
```
```rust
use std::rc::Rc;

/// Exposes the mosaic path at which a panel is located. Unlike calling `mosaicWindowActions.getPath()` during render, subscribing to this context will trigger a
/// re-render when the path changes.
pub struct MosaicPathContext {
    pub mosaic_path: Rc<MosaicPath>,
}
```
```rust
use std::future::Future;
use std::pin::Pin;

struct LocalLayouts;
impl LayoutLoader for LocalLayouts {
    const NAMESPACE: &'static str = "local";
    type Item = LayoutInfo;

    fn fetch_layouts(&self) -> Pin<Box<dyn Future<Output = Result<Vec<Self::Item>, Box<dyn std::error::Error>>>>> {
        Box::pin(async move { Ok(vec![]) })
    }
}
```
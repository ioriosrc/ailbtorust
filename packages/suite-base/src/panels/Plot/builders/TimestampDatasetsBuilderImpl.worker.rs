```rust
use comlink::{ComlinkExt, Remote};

pub struct TimestampDatasetsBuilderImpl;

impl ComlinkExt for TimestampDatasetsBuilderImpl {
    type Remote = Remote<Self>;
}

fn main() {
    let timestamp_datasets_builder: Remote<TimestampDatasetsBuilderImpl> = TimestampDatasetsBuilderImpl;
}
```
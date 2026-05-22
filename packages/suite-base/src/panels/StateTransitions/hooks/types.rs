```rust
use std::any::{Any, TypeId};

struct ChartDatasets;

struct PathState;

pub type UseStateTransitionsData = (Vec<PathState>, ChartDatasets, Option<f64>);
```
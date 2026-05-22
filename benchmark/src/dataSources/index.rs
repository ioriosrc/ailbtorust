```rust
use crate::synthetic_data_source_factory::SyntheticDataSourceFactory;
use crate::mcap_local_benchmark_data_source_factory::McapLocalBenchmarkDataSourceFactory;

pub fn export() {
    SyntheticDataSourceFactory::new();
    McapLocalBenchmarkDataSourceFactory::new();
}
```

Note: The actual implementation of the `SyntheticDataSourceFactory` and `McapLocalBenchmarkDataSourceFactory` is not provided in this Rust code snippet, as it would typically be part of a larger module or library.
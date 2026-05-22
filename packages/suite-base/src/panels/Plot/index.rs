```rust
use lightrisk::components::Panel;
use lightrisk::plots::{Plot, DEFAULT_PLOT_CONFIG};

pub fn plot_panel() -> Panel<Plot> {
    Plot::new()
        .with_config(DEFAULT_PLOT_CONFIG)
        .into()
}
```
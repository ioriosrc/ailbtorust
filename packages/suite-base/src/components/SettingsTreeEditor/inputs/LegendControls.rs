```rust
use mui::prelude::*;

fn main() {
    AppBuilder::new()
        .title("Legend Controls")
        .render(|cx| {
            let legend_count = use_legend_count(cx);

            view! {
                cx,
                <div data-testid="legend-controls">
                    <IconButton onClick={legend_count.increment}>
                        <AddIcon />
                    </IconButton>
                    <IconButton onClick={legend_count.decrement}>
                        <RemoveIcon />
                    </IconButton>
                </div>
            }
        })
        .launch();
}
```

Note: This Rust code uses a different library (`mui`) and the `use_legend_count` hook doesn't exist in standard Rust. You would need to implement your own logic to manage the legend count state and dispatch appropriate actions when buttons are clicked.
```rust
use std::collections::HashMap;

pub fn mui_table_row() -> HashMap<String, String> {
    let mut style_overrides = HashMap::new();

    style_overrides.insert(
        "root".to_string(),
        format!(
            r#"
              &.Mui-disabled {{
                .{0} {{
                  color: {1};
                }}
              }}
            "#,
            table_cell_classes.root.to_string(),
            theme.palette.text.disabled.to_string()
        )
    );

    style_overrides
}
```
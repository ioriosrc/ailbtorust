```rust
pub fn mui_table_cell() -> OverrideComponentReturn<"MuiTableCell"> {
  OverrideComponentReturn {
    styleOverrides: {
      head: {
        fontWeight: 700,
      },
      sticky_header: |theme| {
        Box::new({
          style: theme.palette.background.paper,
        })
      },
    },
  }
}
```
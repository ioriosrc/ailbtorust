```rust
use tss_react::make_styles;

pub fn use_styles() -> make_styles! {
  menu_list {
    min_width: 180,
    max_width: 220,
  }
  truncate {
    align_self: "center !important",
  }
}
```
Note: The `tss_react` crate is a Rust package that helps with creating styled components based on CSS-in-JS libraries. It's used here to create styles for React functional components.
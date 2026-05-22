```rust
use styled_components::{styled, css};

pub fn MuiDialogActions() -> styled<()>
where
    Self: Sized,
{
    styled("div")
        .with_properties(
            css! {
                gap: theme.spacing(1),
                padding: theme.spacing(3),

                > :not(:first-of-type): {
                    margin-left: "inherit";
                }
            },
        )
}
```
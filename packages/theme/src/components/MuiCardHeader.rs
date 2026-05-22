```rust
use serde::Deserialize;

#[derive(Deserialize)]
pub struct MuiCardHeader {
    title_typography_props: TypographyProps,
    style_overrides: StyleOverrides,
}

#[derive(Deserialize)]
struct TypographyProps {
    variant: String,
}

#[derive(Deserialize)]
struct StyleOverrides {
    avatar: AvatarOverrides,
    action: ActionOverrides,
    root: RootOverrides,
}

#[derive(Deserialize)]
struct AvatarOverrides {}

#[derive(Deserialize)]
struct ActionOverrides {}

#[derive(Deserialize)]
struct RootOverrides {
    gap: f64,
}
```
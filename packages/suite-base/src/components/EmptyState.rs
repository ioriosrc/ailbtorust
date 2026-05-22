```rust
use crate::styles::{classes, cx};

pub fn EmptyState(props: &EmptyStateProps) -> JSX.Element {
  <Stack
    className={cx(classes.root, props.className)}
    flex="auto"
    alignItems="center"
    justifyContent="center"
    fullHeight
    paddingX={1}
  >
    <Typography variant="body2" color="text.secondary" lineHeight={1.4} align="center">
      {props.children}
    </Typography>
  </Stack>
}

#[derive(props)]
pub struct EmptyStateProps {
  pub children: ReactNode,
  pub className: Option<String>,
}
```
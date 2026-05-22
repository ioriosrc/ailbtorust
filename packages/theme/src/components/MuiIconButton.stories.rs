```rust
use fluentui::icons::{Delete12Regular, Delete16Regular, Delete20Regular, Delete28Regular, Fingerprint20Regular, ShoppingBag20Regular};
use mui::material::{IconButtonProps, IconButton as MuiIconButton, Stack};

#[derive(Default, Clone)]
struct IconButtonExample;

impl IconButtonExample {
    fn render() -> impl 'static + std::fmt::Display {
        let colors = ["inherit", "primary", "secondary", "success", "error", "info", "warning"];

        html! {
            <Stack direction="row" justify_content="center" align_items="center" padding={2}>
                <MuiIconButton aria-label="delete">
                    <Delete20Regular />
                </MuiIconButton>
                <MuiIconButton aria-label="delete" disabled color="primary">
                    <Delete20Regular />
                </MuiIconButton>
                <MuiIconButton color="secondary" aria-label="add an alarm">
                    <ClockAlarm20Regular />
                </MuiIconButton>
                <MuiIconButton color="primary" aria-label="add to shopping cart">
                    <ShoppingBag20Regular />
                </MuiIconButton>
            </Stack>
        }
    }

    fn sizes() -> impl 'static + std::fmt::Display {
        let colors = ["inherit", "primary", "secondary", "success", "error", "info", "warning"];

        html! {
            <Stack direction="row" justify_content="center" align_items="center" padding={2}>
                <MuiIconButton aria-label="delete" size="small">
                    <Delete12Regular />
                </MuiIconButton>
                <MuiIconButton aria-label="delete" size="small">
                    <Delete16Regular />
                </MuiIconButton>
                <MuiIconButton aria-label="delete" size="large">
                    <Delete20Regular />
                </MuiIconButton>
                <MuiIconButton aria-label="delete" size="large">
                    <Delete28Regular />
                </MuiIconButton>
            </Stack>
        }
    }

    fn colors() -> impl 'static + std::fmt::Display {
        let colors = ["inherit", "primary", "secondary", "success", "error", "info", "warning"];

        html! {
            <Stack direction="row" justify_content="center" align_items="center" padding={2}>
                {colors.iter().map(|color| {
                    html! {
                        <MuiIconButton color={color} key={color}>
                            <Fingerprint20Regular />
                        </MuiIconButton>
                    }
                })}
            </Stack>
        }
    }
}
```
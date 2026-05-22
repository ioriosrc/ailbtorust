```rust
use crate::components::{Stack, Button};
use crate::context::WorkspaceActions;
use crate::styles::{use_styles};

#[derive(Debug)]
pub struct ViewProps {
    on_open: Option<fn()>,
}

#[derive(Debug)]
pub struct View {
    props: ViewProps,
    dialog_actions: WorkspaceActions,
}

impl View {
    pub fn new(props: ViewProps, dialog_actions: WorkspaceActions) -> Self {
        Self { props, dialog_actions }
    }

    pub fn render(&self) -> impl 'static + Fn() -> ReactNode {
        move || {
            let classes = use_styles();
            let { on_open } = self.props;

            <div className={classes.content}>
                {self.props.children}
            </div>
            <Stack
                direction="row"
                justifyContent="space-between"
                alignItems="center"
                paddingX="4px"
                paddingBottom="4px"
                paddingTop="2px"
            >
                <Button
                    startIcon={<ChevronLeftIcon fontSize="large" />}
                    onClick={move || {
                        self.dialog_actions.dataSource.open("start");
                    }}
                >
                    Back
                </Button>
                <Stack direction="row" gap={2}>
                    <Button
                        color="inherit"
                        variant="outlined"
                        onClick={move || {
                            self.dialog_actions.dataSource.close();
                        }}
                    >
                        Cancel
                    </Button>
                    <Button variant="contained" onClick={on_open} disabled={on_open.is_none()}>
                        Open
                    </Button>
                </Stack>
            </Stack>
        }
    }
}
```

Este código Rust é uma implementação funcional do componente `View` com base no original TypeScript/React, mantendo o mesmo comportamento e estilização.
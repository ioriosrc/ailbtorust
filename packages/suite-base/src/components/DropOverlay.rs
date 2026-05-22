```rust
use std::rc::Rc;

use mui::{
    components::{Backdrop, Dialog},
    theme::Theme,
};

use crate::components::{alpha, Typography};

pub fn DropOverlay(props: PropsWithChildren<{ open: bool }>) -> Rc<Dialog> {
    let classes = useStyles();

    Rc::new(
        Dialog::new()
            .full_screen(true)
            .open(props.open)
            .style({
                "z-index": 10000000,
            })
            .classes(classes)
            .children(DropOverlayContent { children: props.children }),
    )
}

fn DropOverlayContent { children } => {
    let theme = &Theme::current();

    Box::new(
        DialogContentWrapper::default()
            .border({ border: format!("2px dashed {}", theme.palette.text.primary) })
            .padding(theme.spacing(5))
            .children(children),
    )
}
```
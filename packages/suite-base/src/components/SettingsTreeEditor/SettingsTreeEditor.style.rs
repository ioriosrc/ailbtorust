```rust
use crate::shared::tss_react_mui::{make_styles, use_styles};

pub fn useStyles() -> impl use_styles::UseStyles<'static> {
    make_styles!(|theme| ({
        appBar: {
            top: 0,
            marginRight: "1px",
            zIndex: theme.zIndex.appBar,
            padding: theme.spacing(0.5),
            position: "sticky",
            background_color: theme.palette.background.paper,
        },
        fieldGrid: {
            display: "grid",
            grid_template_columns: "minmax(20%, 20ch) auto",
            column_gap: theme.spacing(1),
        },
        textField: {
            ".MuiOutlinedInput-notchedOutline": {
                border: "none",
            },
        },
        startAdornment: {
            display: "flex",
        },
    }))
}
```
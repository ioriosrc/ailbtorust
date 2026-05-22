```rust
use styled_components::{css, StyledComponent};

pub fn get_styled_components() -> Vec<StyledComponent<&'static str>> {
    let styles = vec![
        css! {
            .autocomplete {
                ".MuiInputBase-root.MuiInputBase-sizeSmall" => {
                    padding_inline: 0,
                    padding_block: theme.spacing(0.3125),
                },
            }
        },
        css! {
            .clearIndicator {
                margin_right: theme.spacing(-0.25),
                opacity: theme.palette.action.disabledOpacity,

                ":hover" => {
                    background: "transparent",
                    opacity: 1,
                },
            }
        },
        css! {
            .error {} // No equivalent in Rust
        },
        css! {
            .fieldLabel {
                color: theme.palette.text.secondary,
                overflow: "hidden",
                textOverflow: "ellipsis",
                whiteSpace: "nowrap",
                minWidth: 0,
            }
        },
        css! {
            .fieldWrapper {
                width: 100%;
                overflow: hidden;
                marginRight: theme.spacing(0.5);
                height: "100%";
                align-items: center;
                text-align: end;
            }
        },
        css! {
            .styledToggleButtonGroup {
                background: theme.palette.action.hover,
                gap: theme.spacing(0.25),
                overflow_x: "auto",

                "& .MuiToggleButtonGroup-grouped" => {
                    margin: theme.spacing(0.55),
                    borderRadius: theme.shape.borderRadius,
                    paddingTop: 0,
                    paddingBottom: 0,
                    borderColor: "transparent !important",
                    lineHeight: 1.75,

                    "&.Mui-selected" => {
                        background: theme.palette.background.paper,
                        borderColor: "transparent",

                        "&:hover" => {
                            borderColor: theme.palette.action.active,
                        },
                    },
                    "&:not(:first-of-type)" => {
                        borderRadius: theme.shape.borderRadius,
                    },
                    "&:first-of-type" => {
                        borderRadius: theme.shape.borderRadius,
                    },
                },
            }
        },
        css! {
            .slider {
                color: theme.palette.secondary.main,
                top: "2px",
                height: 3,
                width: "calc(100% - 30px)",
                left: "10px",

                "& .MuiSlider-thumb" => {
                    "&:focus, &:hover, &.Mui-active, &.Mui-focusVisible": {
                        boxShadow: "none",
                    },
                },
            }
        },
    ];

    styles.into_iter().map(|style| style).collect()
}
```
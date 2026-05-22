```rust
use fluentui::{icons::ArrowDownload20Filled, icons::Delete20Regular};
use flume::prelude::*;
use mui::{
    components::{
        Button,
        IconButton,
        Dialog,
        DialogContent,
        DialogActions,
        DialogTitle,
        TextField,
    },
    theme::{custom_typography},
    util::download_text_file,
};

use crate::components::{CopyButton, HoverableIconButton, Stack};
use crate::theme::styles::ShareJsonModalStyles;

#[derive(Debug)]
pub struct ShareJsonModalProps {
    open: bool,
    onClose: Box<dyn Fn()>,
    onChange: Box<dyn Fn(&str)>,
    initialValue: serde_json::Value,
    title: String,
}

impl Component for ShareJsonModalProps {
    fn render(self) -> Element {
        let { classes, styles } = self.use_styles<ShareJsonModalStyles>();
        let (value, set_value) = use_state(String::from(
            serde_json::to_string(&self.initial_value).unwrap_or_default(),
        ));

        let { decoded_value, error } = useMemo(() => {
            match serde_json::from_str(&value).map_err(|err| err.into()) {
                Ok(decoded_value) => ({ decoded_value, error: None }),
                Err(error) => ({ decoded_value: None, error }),
            }
        }, [value]);

        let handleSubmit = use_callback(|| {
            self.onChange.call(&serde_json::to_string(decoded_value).unwrap_or_default());
            self.on_close.call();
        }, [self.onChange, self.on_close, decoded_value]);

        let handle_download = use_callback(|| download_text_file(value.clone(), "layout.json"), [value]);

        let getText = use_callback(move || value.clone(), [value]);

        (
            <Dialog open={self.open} onClose=self.on_close}>
                <DialogTitle className={classes.dialog_title}>
                    {self.title}
                    <IconButton onClick=self.on_close} edge="end">
                        <CloseIcon />
                    </IconButton>
                </DialogTitle>
                <DialogContent>
                    <TextField
                        className={classes.textarea}
                        fullWidth
                        multiline
                        rows={10}
                        value=value
                        onChange={|event: Event| set_value(event.data().as_string().unwrap_or_default())}
                        autoFocus
                        error=error != None
                        helperText=match error {
                            Some(error) => "The JSON provided is invalid.",
                            None => " ",
                        }
                        slotProps={{
                            htmlInput: { data_testid: "share-json-input" },
                            formHelperText: { variant: "standard" },
                        }}
                        spellCheck={false}
                    />
                </DialogContent>
                <DialogActions>
                    <Stack direction="row" gap=1>
                        <IconButton onClick=handle_download>Download</IconButton>
                        <CopyButton color="inherit" getText=get_text />
                        <HoverableIconButton
                            active_color="error"
                            onClick=move || set_value(String::from("{}"))
                            title="Clear"
                            aria-label="Clear"
                            icon=<Delete20Regular />}
                        />
                    </Stack>

                    <Stack flex="auto" />

                    <Button disabled=error != None variant="contained" onClick=handleSubmit>
                        Apply
                    </Button>
                </DialogActions>
            </Dialog>,
        )
    }
}

#[derive(Debug)]
struct ShareJsonModalStyles;

impl Styles for ShareJsonModalStyles {
    fn root(&self, props: &ShareJsonModalProps) -> Vec<Rule> {
        vec![
            Rule::selector(":host"),
            Rule::properties({
                "display": "flex",
                "flex-direction": "column",
                "width": "100%",
                "max-width": "500px",
                "margin: auto",
                "padding": "20px",
                "border-radius": "8px",
                "box-shadow": "0 4px 8px rgba(0, 0, 0, 0.1)",
            }),
        ]
    }

    fn textarea(&self, props: &ShareJsonModalProps) -> Vec<Rule> {
        vec![
            Rule::selector(":host ::slotted textarea"),
            Rule::properties({
                "background-color": props.styles.textarea.background_color,
                "font-family": custom_typography.font_monospace,
                "max-height": "60vh",
                "overflow-y": "auto",
                "padding": props.styles.textarea.padding,
                "box-sizing": "border-box",
            }),
        ]
    }
}
```
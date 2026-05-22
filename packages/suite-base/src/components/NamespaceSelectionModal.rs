```rust
use std::fs::{self};

use materialize::{
    dialogs::{Dialog, DialogActions, DialogContent},
    icons::icons as Icons,
    lists::{ListItem, ListItemButton, ListItemIcon, ListItemText},
    typography::Typography,
    {prelude::*, Element},
};
use serde_json::{Value};

use crate::{NamespaceSelectionModalProps, Namespace};

pub struct NamespaceSelectionModal {
    open: bool,
    onClose: fn(),
    onSelect: fn(Namespace),
    files: Vec<File>,
}

impl NamespaceSelectionModal {
    pub fn new(
        open: bool,
        onClose: fn(),
        onSelect: fn(Namespace),
        files: Vec<File>,
    ) -> Self {
        NamespaceSelectionModal { open, onClose, onSelect, files }
    }

    pub fn render(&self) -> Element {
        let classes = useStyles();

        Dialog::new()
            .open(self.open)
            .on_close(self.on_close)
            .max_width("sm")
            .full_width()
            .render(|_| {
                DialogContent::new().render(|_| {
                    Typography::new()
                        .variant("body1")
                        .classes(classes.file_type_text)
                        .render(|_| {
                            Typography::new()
                                .variant("body2")
                                .classes(classes.file_names_text)
                                .render(|_| {
                                    Typography::new()
                                        .variant("body1")
                                        .classes(classes.question_text)
                                        .render(|_| {
                                            List::new().render(|_| {
                                                ListItem::new().disable_padding().render(|_| {
                                                    ListItemButton::new()
                                                        .selected(self.selected_namespace() == "local")
                                                        .on_click(|| self.set_selected_namespace("local"))
                                                        .render(|_| {
                                                            Icons::Computer.render()
                                                                .with_classes(classes.icon)
                                                                .render(|_| {
                                                                    Typography::new()
                                                                        .primary("Local")
                                                                        .secondary(
                                                                            "Install only on this device. Files will be stored locally and won't be shared with your organization.",
                                                                        )
                                                                        .render(|_| {}),
                                                                });
                                                            Typography::new()
                                                                .primary("Organization")
                                                                .secondary(
                                                                    "Install for your entire organization. Files will be shared with all members of your organization.",
                                                                )
                                                                .render(|_| {}),
                                                        });
                                                    });
                                                });
                                            });
                                        })
                                    .build(),
                                }),
                            }),
                        });

                    DialogActions::new().render(|_| {
                        Button::new()
                            .on_click(self.on_close)
                            .classes(classes.cancel_button)
                            .render(|_| Typography::new().primary("Cancel").render(|_| {})),
                        Button::new()
                            .on_click(self.handle_select)
                            .variant("contained")
                            .classes(classes.install_button)
                            .render(|_| Typography::new().primary("Install").render(|_| {})),
                    });
                })
            })
    }

    fn set_selected_namespace(&mut self, namespace: &str) {
        self.selected_namespace = namespace;
    }

    fn selected_namespace(&self) -> &str {
        &self.selected_namespace
    }
}

impl NamespaceSelectionModalProps {
    pub fn new(
        open: bool,
        onClose: fn(),
        onSelect: fn(Namespace),
        files: Vec<File>,
    ) -> Self {
        NamespaceSelectionModalProps { open, onClose, onSelect, files }
    }

    pub fn render(&self) -> Element {
        NamespaceSelectionModal::new(self.open, self.on_close, self.on_select, self.files).render()
    }
}
```

```rust
use std::{fs};

pub struct File {
    name: String,
}

impl File {
    pub fn new(name: &str) -> Self {
        File { name: name.to_string() }
    }

    pub fn path(&self) -> String {
        format!("path/to/files/{}", self.name)
    }
}
```
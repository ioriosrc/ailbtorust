```rust
use std::error::Error;

#[derive(Debug)]
pub struct LayoutSelectionState {
    pub busy: bool;
    pub error: Option<Error>;
    pub online: bool;
    pub last_selected_id: Option<String>;
    pub multi_action: Option<{ action: MultiAction; ids: Vec<String>; }>;
    pub selected_ids: Vec<String>;
}

#[derive(Debug)]
pub enum LayoutSelectionAction {
    ClearMultiAction,
    QueueMultiAction(MultiAction),
    SelectId { id: Option<String>, layouts: Option<Vec<Layout>>, shift_key: bool, mod_key: bool },
    SetBusy(bool),
    SetError(Option<Error>),
    SetOnline(bool),
    ShiftMultiAction,
}

#[derive(Debug)]
pub enum LayoutActionMenuItem {
    Item {
        text: String,
        secondary_text: Option<String>,
        key: String,
        onClick: fn(&mut Self, event: web_sys::MouseEvent) -> Result<(), Error>,
        disabled: bool,
        debug: bool,
        "data-testid": Option<&str>,
    },
    Divider { key: String, debug: bool },
    Header { key: String, text: String, debug: bool },
}

pub type SignInPromptProps = {
    onDismiss: fn() -> (),
};

#[derive(Debug)]
pub enum UnsavedChangesResolution {
    Cancel,
    Discard,
    MakePersonal { name: String },
    Overwrite,
}
```
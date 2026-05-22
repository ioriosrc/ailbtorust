```rust
use std::cmp;
use std::collections::HashSet;

type Action = (
    LayoutSelectionActionType,
    LayoutSelectionActionValue,
    Option<LayoutSelectionActionId>,
);

#[derive(Debug, PartialEq)]
enum LayoutSelectionActionType {
    ClearMultiAction,
    QueueMultiAction { action: String, ids: Vec<String> },
    ShiftMultiAction,
    SelectId { id: String, mod_key: bool, shift_key: bool },
    SetBusy(bool),
    SetError(String),
    SetOnline(bool),
}

#[derive(Debug, PartialEq)]
struct LayoutSelectionActionValue {
    // Define the structure of your action values as needed
}

#[derive(Debug, PartialEq)]
struct LayoutSelectionActionId {
    id: String,
}

#[derive(Default, Debug, PartialEq)]
pub struct LayoutSelectionState {
    busy: bool,
    error: Option<String>,
    online: bool,
    last_selected_id: Option<String>,
    selected_ids: HashSet<String>,
    multi_action: Option<MultiAction>,
}

type MultiAction = {
    action: String,
    ids: Vec<String>,
};

impl Default for LayoutSelectionState {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutSelectionState {
    pub fn new() -> Self {
        LayoutSelectionState {
            busy: false,
            error: None,
            online: true,
            last_selected_id: None,
            selected_ids: HashSet::new(),
            multi_action: None,
        }
    }

    pub fn select_id(&mut self, id: String, mod_key: bool, shift_key: bool) -> Self {
        if mod_key {
            let mut new_selection = self.selected_ids.clone();
            new_selection.insert(id);
            self.selected_ids = new_selection;
        } else if shift_key {
            self.multi_action = None;
        } else {
            self.multi_action = None;
            self.selected_ids.clear();
            self.selected_ids.insert(id);
        }
        self.last_selected_id = Some(id);

        // Implement the logic for other actions similarly

        self.clone()
    }

    // Implement the logic for other actions similarly
}
```
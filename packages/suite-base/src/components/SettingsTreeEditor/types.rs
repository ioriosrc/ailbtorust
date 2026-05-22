```rust
use std::collections::HashSet;

// Define the types as per your requirements
type ActionsMenuProps = (Vec<SettingsTreeNodeAction>, fn(&str) -> ());
type NodeEditorProps = (
  fn(&SettingsTreeAction),
  bool,
  Option<&[&str]>,
  &[&str],
  &Immutable<SettingsTreeNode>,
);
type SelectVisibilityFilterValue = &str;
type FieldEditorProps = (fn(&SettingsTreeAction), &Immutable<SettingsTreeField>, &[&str]);
type SettingsTreeEditorProps = (&'static str, &Immutable<SettingsTree>);
type DragItem = (&[&str],);
type NodeEditorState = {
  editing: bool,
  focusedPath: Option<&[&str]>,
  open: bool,
  visibilityFilter: SelectVisibilityFilterValue,
};
```
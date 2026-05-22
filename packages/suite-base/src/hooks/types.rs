```rust
use react::createElement;

type UseLayoutActions = {
  onRenameLayout: (item: Layout, newName: String) -> std::result::Result<(), ()>;
  onDuplicateLayout: (item: Layout) -> std::result::Result<(), ()>;
  onDeleteLayout: (item: Layout) -> std::result::Result<(), ()>;
  onRevertLayout: (item: Layout) -> std::result::Result<(), ()>;
  onOverwriteLayout: (item: Layout) -> std::result::Result<(), ()>;
  confirmModal: Option<ReactElement>;
};

type LayoutSetupOptions = {
  state: LayoutSelectionState;
  dispatch: Dispatch<LayoutSelectionAction>;
};
```
```rust
use mockall::mock;
use mockall::predicate::{eq, eq_op};

#[cfg(test)]
mod tests {
    use super::*;

    struct LayoutState {
        shared_panel_state: Option<HashMap<String, String>>,
        selected_layout: Option<SelectedLayout>,
    }

    struct SelectedLayout {
        id: String,
        loading: bool,
        data: Option<HashMap<String, Any>>,
        name: String,
        edited: bool,
    }

    #[test]
    fn test_update_shared_panel_state_with_empty_data() {
        let mut mock_layout_state = MockLayoutState::new();
        mock_layout_state.expect_selected_layout().return_value(None);

        let mut result = use_update_shared_panel_state(mock_layout_state, set_layout_state);
        act!(result.update_shared_panel_state(panel_type.clone(), Some({ [key_test]: value_test })));

        assert!(!mock_layout_state.set_layout_state.is_called());
    }

    #[test]
    fn test_update_shared_panel_state_with_non_empty_data() {
        let mut mock_layout_state = MockLayoutState::new();
        let selected_layout: SelectedLayout = BasicBuilder.selected_layout().build();
        mock_layout_state.expect_selected_layout().return_value(Some(selected_layout));

        let mut result = use_update_shared_panel_state(mock_layout_state, set_layout_state);
        act!(result.update_shared_panel_state(panel_type.clone(), Some({ [key_test]: value_test })));

        assert!(mock_layout_state.set_layout_state.is_called_with(
            mock_layout_state
                .selected_layout()
                .return_value(Some(selected_layout))
                .unwrap()
                .clone(),
            {
                shared_panel_state: Some({
                    let mut map = HashMap::new();
                    map.insert(panel_type.clone(), value_test.clone());
                    map
                }),
            }
        ));
    }

    fn set_layout_state(_: LayoutStateRef, new_state: Box<dyn Fn(&LayoutState) -> LayoutState>) {
        // Implementation of set_layout_state
    }

    fn use_update_shared_panel_state(
        layout_state_ref: MockLayoutStateRef,
        set_layout_state: Box<dyn Fn(&LayoutState) -> LayoutState>,
    ) -> UseUpdateSharedPanelStateResult {
        // Implementation of use_update_shared_panel_state
    }
}
```
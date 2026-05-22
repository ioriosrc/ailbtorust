```rust
use crate::builder::*;
use crate::settings_action_reducer::*;

#[cfg(test)]
mod tests {
    use super::*;

    fn build_update_action() -> SettingsTreeAction {
        let path = BasicBuilder.strings();
        let value = BasicBuilder.string();
        let action = SettingsTreeAction {
            action: "update",
            payload: BasicBuilder::payload("autocomplete", &path, &value),
        };
        action
    }

    fn build_perform_node_action() -> SettingsTreeAction {
        let id = BasicBuilder.string();
        let path = BasicBuilder.strings();
        let action = SettingsTreeAction {
            action: "perform-node-action",
            payload: BasicBuilder::payload("id", &path, &id),
        };
        action
    }

    fn setup(props_override: Option<SettingsActionReducerProps>) -> (SettingsActionReducerProps, SettingsTreeAction) {
        let settings_tree_action = BasicBuilder.sample(["perform-node-action", "update"]);
        let action = build_perform_node_action();
        if settings_tree_action == "update" {
            let updated_action = build_update_action();
            action.update(payload(updated_action));
        }

        let props: SettingsActionReducerProps = match props_override {
            Some(props) => props,
            None => BasicBuilder.props(),
        };

        (props, action)
    }

    #[test]
    fn test_should_throw_an_error_for_a_perform_node_action_action() {
        let (props, action) = setup(Some(build_perform_node_action()));
        assert_eq!(settings_action_reducer(props), Err(Error::ActionNotHandled));
    }

    #[test]
    fn test_should_update_a_general_property_when_path_is_general() {
        let value = BasicBuilder.string();
        let path = BasicBuilder.strings();
        let action = build_update_action();
        let (props, _) = setup(Some(action));

        let result = settings_action_reducer(props);
        assert_eq!(result.path, vec!["general", &path[1]]);
        assert_eq!(result.value, value);
    }

    #[test]
    fn test_should_throw_an_error_for_an_unexpected_path_0() {
        let action = build_update_action();
        let (props, _) = setup(Some(action));

        assert_eq!(
            settings_action_reducer(props),
            Err(Error::InvalidPath("general".to_string()))
        );
    }

    #[test]
    fn test_immer_should_return_the_same_config_if_the_action_type_is_not_handled() {
        let action = BasicBuilder.sample(["unknown-action", "payload"]);
        let (props, _) = setup(Some(action));

        assert_eq!(settings_action_reducer(props), props);
    }
}
```
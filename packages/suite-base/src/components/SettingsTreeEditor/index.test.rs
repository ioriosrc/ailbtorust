```rust
use crate::settings_tree_editor::*;
use crate::mocks::*;

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_test::{setup, teardown};

    #[test]
    fn test_render_settings_tree_editor() {
        setup();

        let node_label = BasicBuilder.string();
        let node_label2 = BasicBuilder.string();

        let render_component = async move {
            let props: SettingsTreeEditorProps = SettingsTreeEditorProps::new()
                .with_variant("panel")
                .with_settings(settings_with_filter_enabled())
                .with_nodes(vec![
                    NodeInfo {
                        label: String::from(node_label),
                    },
                    NodeInfo {
                        label: String::from(node_label2),
                    },
                ])
                .build();

            let ui = render(
                DndProvider::new(SettingsTreeEditor::new(props))
                    .use_backend(HTML5Backend)
                    .build(),
            );

            assert!(ui.get_by_test_id("panel-settings-filter-input").is_some());
        };

        assert_ok!(render_component());

        teardown();
    }

    #[test]
    fn test_filter_for_node() {
        setup();

        let node_label = BasicBuilder.string();
        let node_label2 = BasicBuilder.string();

        let render_component = async move {
            let props: SettingsTreeEditorProps = SettingsTreeEditorProps::new()
                .with_variant("panel")
                .with_settings(settings_with_filter_enabled())
                .with_nodes(vec![
                    NodeInfo {
                        label: String::from(node_label),
                    },
                    NodeInfo {
                        label: String::from(node_label2),
                    },
                ])
                .build();

            let ui = render(
                DndProvider::new(SettingsTreeEditor::new(props))
                    .use_backend(HTML5Backend)
                    .build(),
            );

            let input_field = ui.get_by_test_id("panel-settings-filter-input").unwrap();
            user_event::change(&input_field, node_label);

            assert!(ui.get_text_content().contains(node_label));
            assert!(!ui.get_text_content().contains(node_label2));

            teardown();
        };

        assert_ok!(render_component());
    }

    #[test]
    fn test_clear_filter() {
        setup();

        let node_label = BasicBuilder.string();
        let node_label2 = BasicBuilder.string();

        let render_component = async move {
            let props: SettingsTreeEditorProps = SettingsTreeEditorProps::new()
                .with_variant("panel")
                .with_settings(settings_with_filter_enabled())
                .with_nodes(vec![
                    NodeInfo {
                        label: String::from(node_label),
                    },
                    NodeInfo {
                        label: String::from(node_label2),
                    },
                ])
                .build();

            let ui = render(
                DndProvider::new(SettingsTreeEditor::new(props))
                    .use_backend(HTML5Backend)
                    .build(),
            );

            let input_field = ui.get_by_test_id("panel-settings-filter-input").unwrap();
            user_event::change(&input_field, node_label);

            let clear_button = ui.get_by_text("Clear Filter");
            assert!(clear_button.is_some());

            user_event::click(&clear_button.unwrap());

            assert_eq!(ui.get_text_content(), "");
        };

        assert_ok!(render_component());
    }
}
```
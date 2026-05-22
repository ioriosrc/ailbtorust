```rust
use test_case::test_case;

mod tests {
    use super::*;

    #[test_case]
    fn should_render_correctly_with_provided_paths() {
        let props = PathLegendProps {
            height_per_topic: BasicBuilder.number({ min: 0, max: 5000 }),
            paths: vec![BasicBuilder.string(), BasicBuilder.string()],
            save_config: mock_save_config,
            set_focused_path: mock_set_focused_path,
        };

        let ui = render_component(props);

        assert!(ui.rendering().text("path-0").is_some());
        assert!(ui.rendering().text("path-1").is_some());
    }

    #[test_case]
    fn should_render_default_text_when_paths_array_is_empty() {
        let props = PathLegendProps {
            paths: Vec::new(),
            save_config: mock_save_config,
            set_focused_path: mock_set_focused_path,
        };

        render_component(props);

        assert!(ui.rendering().text("Click to add a series").is_some());
    }

    #[test_case]
    async fn should_call_set_focused_path_and_open_panel_settings_when_edit_button_is_clicked() {
        let row = 0;
        let panel_id = BasicBuilder.string();
        let props = PathLegendProps {
            height_per_topic: BasicBuilder.number({ min: 0, max: 5000 }),
            paths: vec![BasicBuilder.string(), BasicBuilder.string()],
            save_config: mock_save_config,
            set_focused_path: mock_set_focused_path,
            panel_id,
        };

        let ui = render_component(props);
        let edit_button = ui.rendering().find("path-edit-button").unwrap();

        await ui.user.click(edit_button);

        assert!(edit_button.is_visible());
        assert_eq!(mock_set_selected_panel_ids.call_count(), 1);
        assert_eq!(mock_open_panel_settings.call_count(), 1);
        assert_eq!(mock_set_focused_path.call_args().nth(0), (vec![panel_id],));
    }

    #[test_case]
    async fn should_call_save_config_with_updated_paths_when_delete_button_is_clicked() {
        let row = 0;
        let props = PathLegendProps {
            height_per_topic: BasicBuilder.number({ min: 0, max: 5000 }),
            paths: vec![BasicBuilder.string(), BasicBuilder.string()],
            save_config: mock_save_config,
            set_focused_path: mock_set_focused_path,
        };

        let ui = render_component(props);
        let delete_button = ui.rendering().find("path-delete-button").unwrap();

        await ui.user.click(delete_button);

        assert_eq!(mock_save_config.call_args().nth(0), (vec![props.paths[1]]));
    }

    #[test_case]
    fn should_apply_the_correct_height_for_each_topic() {
        let props = PathLegendProps {
            height_per_topic: BasicBuilder.number({ min: 0, max: 5000 }),
            paths: vec![BasicBuilder.string(), BasicBuilder.string()],
            save_config: mock_save_config,
            set_focused_path: mock_set_focused_path,
        };

        let ui = render_component(props);

        assert!(ui.rendering().find("path-row-0").unwrap().style().get_text().contains("height: 500px;"));
        assert!(ui.rendering().find("path-row-1").unwrap().style().get_text().contains("height: 500px;"));
    }
}
```
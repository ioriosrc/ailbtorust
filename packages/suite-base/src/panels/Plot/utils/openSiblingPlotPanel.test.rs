```rust
use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BasicBuilder, OpenSiblingPanel};
    use test_builders::PlotBuilder;

    #[test]
    fn should_call_open_sibling_panel_with_correct_parameters() {
        let topic_name = BasicBuilder.string();
        let config = PlotBuilder.config();

        open_sibling_plot_panel(&mut |sibling_config_creator| {
            sibling_config_creator(config);
        }, topic_name);

        assert_eq!(
            open_sibling_panel.mock_calls(),
            vec![OpenSiblingPanel::new(vec![
                OpenSiblingPanel::with_siblings(vec![], vec![topic_name.clone()])
                    .with_config(config)
                    .build(),
            ])],
        );
    }

    #[test]
    fn should_add_a_new_topic_name_to_the_paths_if_not_present() {
        let topic_name = BasicBuilder.string();
        let config = PlotBuilder.config();

        open_sibling_plot_panel(&mut |sibling_config_creator| {
            sibling_config_creator(config);
        }, topic_name);

        assert_eq!(
            config.paths,
            vec![
                PlotBuilder::path(vec![topic_name.clone()]).with_timestamp_method("receiveTime").build(),
            ],
        );
    }

    #[test]
    fn should_not_duplicate_an_existing_topic_name_in_the_paths() {
        let topic_name = BasicBuilder.string();
        let path = PlotBuilder::path(vec![topic_name.clone(), topic_name, topic_name]);
        let config = PlotBuilder::config().with_paths(vec![path, path, path]);

        open_sibling_plot_panel(&mut |sibling_config_creator| {
            sibling_config_creator(config);
        }, topic_name);

        assert_eq!(
            config.paths,
            vec![
                PlotBuilder::path(vec![topic_name.clone()]).with_timestamp_method("receiveTime").build(),
            ],
        );
    }
}
```
```rust
use async_test::prelude::*;
use test_builders::*;
use rstest::*;

mod plot_legend {
    use super::*;
    use crate::components::PanelContext;
    use crate::context::CurrentLayoutContext;
    use crate::test_builders::number as sidebar_dimension_builder;

    #[rstest]
    fn renders_plot_legend_without_crashing() {
        setup();
        assert!(screen().get_by_title("Add series").is_displayed());
    }

    #[rstest]
    async fn toggles_legend_visibility_when_icon_button_is_clicked() {
        let mock_save_config = mock_fn!();

        setup({
            show_legend: false,
            save_config: mock_save_config,
        });

        await user_event::setup().click(screen().get_by_role("button"));

        assert!(mock_save_config.called_with({ show_legend: true }));
    }

    #[rstest]
    async fn renders_paths_from_props() {
        let paths = vec![
            { value: BasicBuilder.string(), enabled: true },
            { value: BasicBuilder.string(), enabled: true },
        ];

        setup({ paths });

        assert!(screen().get_by_text(BasicBuilder.string()).is_displayed());
    }

    #[rstest]
    async fn calls_on_click_path_when_a_path_is_clicked() {
        let mock_on_click_path = mock_fn!();

        let paths = vec![
            { value: BasicBuilder.string(), enabled: true },
        ];

        setup({
            paths,
            onClick_path: mock_on_click_path,
        });

        await user_event::setup().click(screen().get_by_text(BasicBuilder.string()));

        assert!(mock_on_click_path.called_with(0));
    }
}
```
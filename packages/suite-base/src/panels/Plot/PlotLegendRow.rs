```rust
use fluentui::icons::{Add12Regular, ErrorCircle16Filled, Square12Filled};
use mui::material::{Checkbox, Tooltip, Typography, ButtonBase};
use mui::theme::ButtonBaseClasses;
use mui::util::color::get_color_from_index;
use react_i18next::t;
use rostime::is_time;
use suite::utils::plot_colors::get_line_color;
use suite_base::context::{CurrentLayoutContext, PanelContext, WorkspaceActions};
use suite_base::util::plot_paths::PlotPath;
use tsx::prelude::*;

struct PlotLegendRowProps {
    has_mismatched_data_length: bool,
    index: usize,
    onClick_path: fn(),
    path: PlotPath,
    paths: Vec<PlotPath>,
    value: Option<Box<dyn Any>>,
    value_source: String,
}

impl Component for PlotLegendRowProps {
    type State = ();

    fn render(&self) -> JSX<Self::State> {
        let { open_panel_settings } = use_workspace_actions();
        let { id: panel_id } = use_panel_context();
        let { selected_panels, set_selected_panels_ids } } = use_selected_panels();
        let { classes, cx } = useStyles();

        // When there are no series configured we render an extra row to show an "add series" button.
        let is_add_series_row = self.paths.len() == 0;

        let handle_delete_path: fn(&mut Self::State) = &mut |state| {
            let mut new_paths = self.paths.clone();
            if let Some(path) = new_paths.get_mut(self.index) {
                path.enabled = !path.enabled;
            }
            set_selected_panels_ids([panel_id]);
            open_panel_settings();
            state.onClick_path();
        };

        let show_plot_values_in_legend = self.value.is_some();

        let render_value = |value: Box<dyn Any>| -> String {
            match value.downcast_ref::<i64>() {
                Some(value) => format!("{}", value),
                _ => format!("{:?}", value),
            }
        };

        <div
            class={cx(
                classes.root,
                {
                    [classes.show_plot_value]: show_plot_values_in_legend,
                },
            )}
            onClick={move || handle_delete_path(state)}
        >
            <div class={classes.list_icon}>
                <Checkbox
                    class={classes.checkbox}
                    checked={self.path.enabled}
                    size="small"
                    title="Toggle visibility"
                    style={{ color: get_color_from_index(self.path.color, self.index) }}
                    icon={<Square12Regular />}
                    checkedIcon={<Square12Filled />}
                    onClick={|event| event.stopPropagation()}
                />
            </div>
            <div
                class={classes.plot_name}
                style={{
                    grid_column: !show_plot_values_in_legend.into_iter().collect::<Vec<&str>>()[0],
                }}
            >
                <Typography
                    noWrap={show_plot_values_in_legend}
                    flex="auto"
                    variant="body2"
                    class={cx({ [classes.disabled_path_label]: !self.path.enabled })}
                    data-testid="plot-legend-row-path-label"
                >
                    {is_add_series_row {
                        t("clickToAddASeries")
                    } else {
                        plot_paths::plot_path_display_name(&self.path, self.index)
                    }}
                </Typography>
                {self.has_mismatched_data_length && (
                    <Tooltip
                        placement="top"
                        title="Mismatch in the number of elements in x-axis and y-axis messages"
                    >
                        <ErrorCircle16Filled class={classes.error_icon} />
                    </Tooltip>
                )}
            </div>
            {show_plot_values_in_legend && (
                <div class={classes.plot_value}>
                    <Typography
                        variant="body2"
                        align="right"
                        color={self.value_source == "hover" {
                            t("warning.main")
                        } else {
                            t("text.secondary")
                        }}
                    >
                        {render_value(self.value.clone())}
                    </Typography>
                </div>
            )}
            <div class={classes.action_button}>
                {self.index == self.paths.len() && (
                    <ButtonBase
                        title="Add series"
                        aria-label="Add series"
                        onClick={move || state.onClick_path()}
                        data-testid="add-series"
                    >
                        <Add12Regular />
                    </ButtonBase>
                )} else {
                    <ButtonBase
                        title="Delete series"
                        aria-label="Delete series"
                        onClick={move || handle_delete_path(state)}
                        data-testid="delete-series"
                    >
                        <Dismiss12Regular />
                    </ButtonBase>
                }}
            </div>
        </div>
    }
}
```
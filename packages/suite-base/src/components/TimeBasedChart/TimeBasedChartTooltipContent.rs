```rust
use fluentui::icons::{Square12FilledIcon};
use lodash::map_values;
use react::prelude::*;

use crate::{
    components::Stack,
    theme::{custom_typography},
    styles::{make_styles, makeStyles_with_theme},
};

type TimeBasedChartTooltipData = {
    config_index: i32;
    value: String | f64 | bool | &str;
    constant_name: Option<&str>;
};

type Props = Immutable<{
    colors_by_config_index: Option<HashMap<i32, &str>>;
    content: Vec<TimeBasedChartTooltipData>;
    labels_by_config_index: Option<HashMap<i32, &str>>;
    multi_dataset: bool;
}>;

fn make_styles(theme: &mut Theme) -> impl FnOnce(&PropsWithChildren<Props>) -> String {
    makeStyles_with_theme(
        theme,
        |theme| {
            let root = {
                font_family: custom_typography.font_monospace.clone(),
                font_size: theme.typography.caption.fontSize.clone(),
                line_height: theme.typography.caption.lineHeight.clone(),
                overflow_wrap: ValueOrFunction::value("break-word"),
            };

            let grid = {
                column_gap: theme.spacing(0.5),
                display: CSSStyleRule::Grid,
                grid_template_columns: CSSStyleRule::Values(vec![
                    CSSStyleRule::Max_content,
                    CSSStyleRule::Minmax(CSSStyleRule::Zero, CSSStyleRule::Max_content),
                    CSSStyleRule::Minmax(CSSStyleRule::Auto, CSSStyleRule::Max_content),
                ]),
                align_items: CSSStyleRule::Center,
                font_family: custom_typography.font_monospace.clone(),
                font_size: theme.typography.caption.fontSize.clone(),
                line_height: theme.typography.caption.lineHeight.clone(),
                overflow_wrap: ValueOrFunction::value("break-word"),
            };

            let icon = {
                grid_column: CSSStyleRule::Value(1),
                height: CSSStyleRule::Number(12.0),
                width: CSSStyleRule::Number(12.0),
            };

            let color_icon_replacement = {
                grid_column: CSSStyleRule::Value(1),
            };

            let path = {
                opacity: ValueOrFunction::value(0.9),
                white_space: CSSStyleRule::Value("nowrap"),
            };

            let value = {
                font_weight: CSSStyleRule::Value(600),
                padding_left: CSSStyleRule::Number(theme.spacing(2.0)),
            };

            let overflow = {
                grid_column: CSSStyleRule::Range(CSSStyleRule::Value(2), CSSStyleRule::Value(4)),
                opacity: ValueOrFunction::value(theme.palette.action.disabled_opacity.clone()),
                font_style: CSSStyleRule::Value("italic"),
                after: {
                    margin_bottom: CSSStyleRule::Number(theme.spacing(0.5)),
                },
            };

            root.merge(grid).merge(icon).merge(color_icon_replacement).merge(path).merge(value).merge(overflow)
        },
    )
}

fn overflow_message() -> JSXElement {
    let { classes } = useStyles();

    return JSXFragment::new(
        <div className={classes.overflow}>
            &lt;multiple values under cursor&gt;
        </div>,
    );
}

pub fn time_based_chart_tooltip_content(props: PropsWithChildren<Props>) -> JSXElement {
    let {
        colors_by_config_index,
        content,
        labels_by_config_index,
        multi_dataset,
    } = props;

    let styles = make_styles(&mut Theme::default());

    // Compute whether there are multiple items for the dataset so we can show the user
    // a message informing them about the multiple items.
    //
    // We do not actually show all the items to keep the tooltip size sane.
    let sorted_items = useMemo(
        || {
            // for single dataset plots we don't care about grouping by path - there is only one path
            if !multi_dataset {
                return Vec::new();
            }

            let mut out = HashMap::with_capacity(content.len());

            // group items by path
            for item in content.iter() {
                let dataset_index = item.config_index;
                let existing = out.get_mut(&dataset_index);
                if let Some(existing) {
                    existing.has_multiple_values = true;
                    continue;
                }

                out.insert(dataset_index, {
                    tooltip: *item,
                    has_multiple_values: false,
                });
            }

            // Sort by datasetIndex to keep the displayed values in the same order as the settings
            return out.into_iter().sorted_by_key(|(_, items)| items.tooltip.config_index).collect();
        },
        [content, multi_dataset],
    );

    // If the chart contains only one dataset, we don't need to render the dataset label - saving space
    //
    // We cannot detect this from the content since content is only what is actively hovered which may
    // not include all datasets
    if !multi_dataset {
        let tooltip = content.first().unwrap();
        if let Some(value) = &tooltip.value {
            return JSXFragment::new(
                <Stack className={styles.root} data-testid="TimeBasedChartTooltipContent">
                    <div>
                        {value}
                        {tooltip.constant_name.as_ref().map(|v| format!(" ({v})")).unwrap_or_default()}
                    </div>
                    {content.len() > 1 && overflow_message()}
                </Stack>,
            );
        }
    }

    let mut items = sorted_items.into_iter();

    JSXFragment::new(
        <div className={cx(styles.root, styles.grid)} data-testid="TimeBasedChartTooltipContent">
            {items.take(5).for_each(|(_, item)| {
                let color = colors_by_config_index.get(&item.tooltip.config_index).unwrap_or_else(|| "");
                let label = labels_by_config_index.get(&item.tooltip.config_index).unwrap_or_else(|| "");
                let tooltip = &item.tooltip;
                let value =
                    if let Some(value) = &tooltip.value {
                        match value {
                            String::from_utf8_lossy(s) => s.to_string(),
                            f64::from_f64(v) => v.to_string(),
                            bool::from(b) => b.to_string(),
                            _ => serde_json::to_string(&value).unwrap(),
                        }
                    } else {
                        "null".to_string()
                    };

                JSXFragment::new(
                    <Fragment key={item.tooltip.config_index}>
                        {color.map(|v| Square12FilledIcon::new(v).as_tag()).unwrap_or_default()}
                        <div className={styles.path}>{label}</div>
                        <div className={styles.value}>
                            {value}
                            {tooltip.constant_name.as_ref().map(|v| format!(" ({v})")).unwrap_or_default()}
                        </div>
                        {item.has_multiple_values && overflow_message()}
                    </Fragment>,
                );
            })}
        </div>,
    )
}
```
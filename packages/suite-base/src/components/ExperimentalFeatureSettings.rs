```rust
use crate::components::*;
use crate::hooks::*;
use crate::settings::*;

#[allow(unused_imports)]
use crate::{AppSetting, AppEvent};

fn use_features() -> Vec<Feature> {
    let t = use_t("appSettings");

    let features: Vec<Feature> = vec![
        Feature {
            key: AppSetting::ENABLE_MEMORY_USE_INDICATOR,
            name: t!("memoryUseIndicator"),
            description: <>{t!("memoryUseIndicatorDescription")}</>,
        },
    ];

    if std::env::var("NODE_ENV") == Ok(String::from("development")) {
        features.push(Feature {
            key: AppSetting::ENABLE_LAYOUT_DEBUGGING,
            name: t!("layoutDebugging"),
            description: <>{t!("layoutDebuggingDescription")}</>,
        });
    }

    features
}

fn ExperimentalFeatureItem(props: &ExperimentalFeature) -> JSXElement {
    let { classes } = use_styles();
    let analytics = use_analytics();

    let (enabled, set_enabled) = use_app_config_value(&props.key);

    return (
        <FormControlLabel
            className={classes.form_control_label}
            control={
                <Checkbox
                    className={classes.checkbox}
                    checked={enabled.unwrap_or_default()}
                    onChange={|_, checked| {
                        set_enabled(checked);
                        analytics.log_event(AppEvent::EXPERIMENTAL_FEATURE_TOGGLE, {
                            feature: props.key,
                            checked,
                        });
                    }}
                />
            }
            label={
                <Stack gap={0.25} paddingLeft={0.5}>
                    <Typography fontWeight="600">{props.name}</Typography>
                    <Typography variant="body2" color="text.secondary">
                        {props.description}
                    </Typography>
                </Stack>
            }
        />
    );
}

pub fn ExperimentalFeatureSettings() -> JSXElement {
    let features = use_features();
    let { t } = use_t("appSettings");

    return (
        <Stack gap={2}>
            {features.is_empty() && (
                <Typography fontStyle="italic">{t!("noExperimentalFeatures")}</Typography>
            )}
            {features.iter().map(|feature| ExperimentalFeatureItem(feature)).collect()}
        </Stack>
    );
}
```
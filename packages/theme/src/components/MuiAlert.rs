```rust
use fluentui::{icon::*, override_component};

pub fn mui_alert() -> override_component::OverrideComponentReturn<'_, "MuiAlert"> {
    override_component::define("MuiAlert", |builder| {
        builder.default_props({
            icon_mapping: {
                error: FluentUiIconErrorCircle20Regular,
                info: FluentUiIconInfo20Regular,
                success: FluentUiIconCheckmarkCircle20Regular,
                warning: FluentUiIconWarning20Regular,
            },
        });

        builder.style_overrides({
            message: {
                line_height: 1.5,
            },
            standard: |theme| {
                theme.border = Some(theme.palette.standard_border);
            },
            standard_warning: |theme| {
                theme.border_color = Some(theme.palette.warning.main);
            },
            standard_error: |theme| {
                theme.border_color = Some(theme.palette.error.main);
            },
            standard_info: |theme| {
                theme.border_color = Some(theme.palette.info.main);
            },
            standard_success: |theme| {
                theme.border_color = Some(theme.palette.success.main);
            },
        });
    })
}
```
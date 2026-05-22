```rust
use std::rc::Rc;

use serde_json::{Value, to_value};

use crate::{
    constants::SUPPORTED_DATA_TYPES,
    types::PieChartConfig,
};

pub type UseSettingsTreeProps = {
    config: PieChartConfig;
    path_parse_error: Option<String>;
    error: Option<String>;
    legend_count: usize;
};

#[derive(Debug)]
pub struct SettingsTreeNode {
    label: String,
    fields: HashMap<String, SettingField>,
}

#[derive(Debug)]
pub struct SettingField {
    label: String,
    input_type: String,
    value: Value,
    error: Option<String>,
    valid_types: Vec<&'static str>,
}

impl UseSettingsTreeProps {
    pub fn new(config: PieChartConfig, path_parse_error: Option<String>, error: Option<String>, legend_count: usize) -> Self {
        Self {
            config,
            path_parse_error,
            error,
            legend_count,
        }
    }
}

pub struct UseSettingsTree {
    general_settings: Rc<SettingsTreeNode>,
}

impl UseSettingsTree {
    pub fn new(props: UseSettingsTreeProps) -> Self {
        let fields = props.config.clone().into_fields();
        let mut general_settings = SettingsTreeNode {
            label: "General settings".to_string(),
            fields,
        };
        if let Some(path_error) = props.path_parse_error {
            general_settings.fields["path"].error = Some(path_error);
        }
        if let Some(error) = props.error {
            general_settings.error = Some(error);
        }

        UseSettingsTree { general_settings: Rc::new(general_settings) }
    }

    pub fn get_general_settings(&self) -> Rc<SettingsTreeNode> {
        self.general_settings.clone()
    }
}
```
```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use std::collections::HashMap;
use std::sync::Arc;

use i18n::{LanguagePack, Translation};
use i18n::react::IntlProvider;
use i18n::reactor::Trans;
use i18n::state_transitions::*;
use lichtblick::suite::{SettingsTreeAction, SettingsTreeNode, SettingsTreeNodeActionItem, SettingsTreeNodes};
use lichtblick::suite_base::{
    panels::StateTransitionsConstants,
    panels::shared::constants::PLOTABLE_ROS_TYPES,
    utils::assign_default_colors_to_series as _,
    utils::handle_reorder_series_action as _,
};
use lichtblick::suite_base::providers::PanelStateContextProvider;
use lichtblick::suite_base::types::{PathState, SeriesAction, SeriesActionId, StateTransitionConfig};
use lichtblick::suite_base::{SaveConfig, SaveConfigError};

use immer::immer;

use crate::shared::*;
use memoize_weak::memoize_weak;

pub fn set_series_action(action: &SeriesAction) -> SettingsTreeNodeActionItem {
    let label = format!("delete series");
    let icon = "Clear";
    let id = SeriesActionId::DELETE;
    SettingsTreeNodeActionItem {
        display: "inline",
        icon,
        id,
        label,
        type: "action",
    }
}

pub fn make_series_node(
    index: usize,
    paths: &PathState,
    can_delete: bool,
    can_reorder: bool,
    t: Trans<'_>,
) -> SettingsTreeNode {
    let action = set_series_action(&SeriesAction::DELETE);
    let label = format!("series {}", index);

    let fields = if !can_delete {
        HashMap::new()
    } else {
        let value = paths.value.clone();
        let label = paths.label.clone();
        let timestamp_method = paths.timestamp_method.clone();

        let valid_types = PLOTABLE_ROS_TYPES;

        HashMap::from([
            ("value", {
                let mut input = "messagepath".to_string();
                if valid_types.len() > 1 {
                    input.push_str(" (");
                    for (i, typ) in valid_types.iter().enumerate() {
                        if i > 0 {
                            input.push(',');
                        }
                        input.push_str(typ);
                    }
                    input.push_str(")");
                }
                let error = if paths.is_default_color && !paths.has_points {
                    Some(t!("pathErrorMessage"))
                } else {
                    None
                };

                InputField {
                    label,
                    input,
                    placeholder: "auto".to_string(),
                    value,
                    error,
                }
            }),
            ("label", InputField {
                label: t("labels.label"),
                input: "string",
                value: paths.label.clone(),
            }),
            (
                "timestampMethod",
                SelectField {
                    label: t("labels.timestamp"),
                    options: PLOTABLE_ROS_TYPES.iter().map(|typ| TypValue { typ, label: typ.to_string() }).collect::<Vec<_>>(),
                    value: paths.timestamp_method.clone(),
                },
            ),
        ])
    };

    SettingsTreeNode {
        actions: can_delete.then_some(vec![action]),
        label,
        reorderable: can_reorder,
        icon: can_reorder.then_some("DragHandle"),
        fields,
    }
}

pub fn make_root_series_node(
    paths: &[PathState],
    t: Trans<'_>,
) -> SettingsTreeNode {
    let children = paths.iter().enumerate().map(|(index, path)| {
        make_series_node(index, path, true, false, t)
    }).collect::<Vec<_>>();

    let actions = if !paths.is_empty() {
        vec![set_series_action(&SeriesAction::ADD)]
    } else {
        Vec::new()
    };

    SettingsTreeNode {
        label: t("labels.series"),
        children,
        actions,
    }
}

pub fn build_settings_tree(
    config: StateTransitionConfig,
    paths: &[PathState],
    focused_path: Option<Vec<&str>>,
) -> SettingsTreeNodes {
    let (x_axis_max_value, xAxis_min_value, xAxis_range, show_points) = (
        config.x_axis_max_value,
        config.x_axis_min_value,
        config.x_axis_range,
        config.show_points,
    );

    let mut nodes: HashMap<&str, SettingsTreeNode> = HashMap::new();

    for path in paths.iter() {
        let series_node = make_series_node(path.index(), path, true, false, t);
        nodes.insert(path.to_string(), series_node);
    }

    let general_fields = if !config.is_synced && config.show_points {
        vec![
            ("isSynced", BooleanField { value: config.is_synced }),
            ("showPoints", BooleanField { value: config.show_points }),
        ]
    } else {
        Vec::new()
    };

    let xAxis_fields = if !config.x_axis_max_value.is_default_color && !config.x_axis_max_value.has_points {
        vec![
            ("xAxisMaxValue", TextField { label: t("max"), value: config.x_axis_max_value.to_string(), error: Some(t!("maxXError")) }),
            ("xAxisMinValue", TextField { label: t("min"), value: config.x_axis_min_value.to_string() }),
            ("xAxisRange", TextField { label: t("secondsRange"), value: config.x_axis_range.to_string() }),
        ]
    } else {
        vec![
            (
                "xAxisMaxValue",
                SelectField {
                    label: t("max"),
                    options: PLOTABLE_ROS_TYPES.iter().map(|typ| TypValue { typ, label: typ.to_string() }).collect::<Vec<_>>(),
                    value: config.x_axis_max_value.to_string(),
                },
            ),
            (
                "xAxisMinValue",
                SelectField {
                    label: t("min"),
                    options: PLOTABLE_ROS_TYPES.iter().map(|typ| TypValue { typ, label: typ.to_string() }).collect::<Vec<_>>(),
                    value: config.x_axis_min_value.to_string(),
                },
            ),
            (
                "xAxisRange",
                SelectField {
                    label: t("secondsRange"),
                    options: PLOTABLE_ROS_TYPES.iter().map(|typ| TypValue { typ, label: typ.to_string() }).collect::<Vec<_>>(),
                    value: config.x_axis_range.to_string(),
                },
            ),
        ]
    };

    let paths_fields = make_root_series_node(paths, t);

    SettingsTreeNodes {
        general: GeneralFieldGroup {
            fields: general_fields,
        },
        xAxis: XAxisFieldGroup {
            fields: xAxis_fields,
        },
        paths: paths_fields,
    }
}

pub struct IUsePanelSettings {
    action_handler: Arc<dyn Fn(SettingsTreeAction)>,
    focused_path: Option<Vec<&str>>,
    nodes: SettingsTreeNodes,
}

impl IUsePanelSettings {
    fn new(
        config: StateTransitionConfig,
        save_config: SaveConfig<StateTransitionConfig>,
        paths: &[PathState],
        focused_path: Option<Vec<&str>>,
    ) -> Self {
        let action_handler = Arc::new(move |action| {
            if action.action == "reorder-node" {
                let source_index = action.payload.path[1].parse::<usize>().unwrap();
                let target_index = action.payload.target_path[1].parse::<usize>().unwrap();
                save_config(
                    produce::<StateTransitionConfig>(|draft| {
                        assign_default_colors_to_series(draft.paths);
                        handle_reorder_series_action(draft, source_index, target_index);
                    }),
                );
            } else if action.action == "update" {
                let { input, path, value } = action.payload;

                if input == "boolean" && path == ["general", "isSynced"] {
                    save_config({ is_synced: value });
                } else if input == "boolean" && path == ["general", "showPoints"] {
                    save_config({ show_points: value });
                } else if path[0] == "xAxis" {
                    save_config(
                        produce::<StateTransitionConfig>(|draft| {
                            _.set(draft, path.slice(1), value);

                            // X min/max and range are mutually exclusive.
                            if path[1] == "xAxisRange" {
                                draft.x_axis_min_value = None;
                                draft.x_axis_max_value = None;
                            } else if path[1] == "xAxisMinValue" || path[1] == "xAxisMaxValue" {
                                draft.x_axis_range = None;
                            }
                        }),
                    );
                } else {
                    save_config(
                        produce::<StateTransitionConfig>(|draft| {
                            if draft.paths.len() == 0 {
                                draft.paths.push({ value: config.x_axis_max_value });
                            }
                            _.set(draft, path, value);
                        }),
                    );
                }
            }

            if action.action == "perform-node-action" {
                if action.id == SeriesActionId::ADD {
                    save_config(
                        produce::<StateTransitionConfig>(|draft| {
                            if draft.paths.len() == 0 {
                                draft.paths.push({ value: config.x_axis_max_value });
                            }
                            draft.paths.push({ value: config.x_axis_min_value });
                        }),
                    );
                } else if action.id == SeriesActionId::DELETE {
                    let index = path[1].parse::<usize>().unwrap();
                    save_config(
                        produce::<StateTransitionConfig>(|draft| {
                            draft.paths.remove(index);
                        }),
                    );
                }
            }
        });

        Self {
            action_handler,
            focused_path,
            nodes: build_settings_tree(config, paths, focused_path.unwrap_or_default()),
        }
    }

    fn action_handler(&self) -> Arc<dyn Fn(SettingsTreeAction)> {
        self.action_handler.clone()
    }

    fn focused_path(&self) -> Option<Vec<&str>> {
        self.focused_path.clone()
    }

    fn nodes(&self) -> &SettingsTreeNodes {
        &self.nodes
    }
}

pub fn use_panel_settings(
    config: StateTransitionConfig,
    save_config: SaveConfig<StateTransitionConfig>,
    paths: &[PathState],
    focused_path: Option<Vec<&str>>,
) -> IUsePanelSettings {
    letIntlProvider::new(LanguagePack::from(vec![Translation::from("en-US")])).render_with_context::<I18nContext, ()>(|| IUsePanelSettings::new(config, save_config, paths, focused_path))
}
```rust
use std::collections::HashMap;

fn rule_to_string(rule: &IndicatorRule) -> String {
    let operator = match rule.operator.as_str() {
        "=" => "=",
        "<" => "<",
        "<=" => "≤",
        ">" => ">",
        ">=" => "≥",
        _ => panic!("Unsupported operator"),
    };
    format!("data {} {}", operator, rule.rawValue)
}

pub fn settings_action_reducer(prev_config: IndicatorConfig, action: SettingsTreeAction) -> IndicatorConfig {
    let mut draft = prev_config;

    match action.action.as_str() {
        "perform-node-action" => {
            if action.payload.path[0] == "rules" {
                if action.payload.id == "delete-rule" {
                    draft.rules.remove(action.payload.path[1].parse::<usize>().unwrap());
                } else if (
                    action.payload.id == "add-rule" ||
                    action.payload.id == "add-rule-above" ||
                    action.payload.id == "add-rule-below"
                ) {
                    let insert_index = match action.payload.id.as_str() {
                        "add-rule-above" => action.payload.path[1].parse::<usize>().unwrap(),
                        "add-rule-below" => action.payload.path[1].parse::<usize>().unwrap() + 1,
                        _ => draft.rules.len(),
                    };
                    draft.rules.insert(insert_index, {
                        operator: rule.operator.clone(),
                        raw_value: rule.rawValue.clone(),
                        color: format!("#{:06x}", rand::random::<u32>()),
                        label: "Label",
                    });
                } else if action.payload.id == "move-up" {
                    let mut moved_rule = draft.rules.remove(action.payload.path[1].parse::<usize>().unwrap());
                    draft.rules.insert(action.payload.path[1].parse::<usize>().unwrap() - 1, moved_rule);
                } else if action.payload.id == "move-down" {
                    let mut moved_rule = draft.rules.remove(action.payload.path[1].parse::<usize>().unwrap());
                    draft.rules.insert(action.payload.path[1].parse::<usize>().unwrap() + 1, moved_rule);
                }
            }
        },
        "update" => match action.payload.path[0].as_str() {
            "general" => {
                draft.general = action.payload.value;
            },
            "rules" => {
                if action.payload.path[1] == "default" {
                    draft.rules[action.payload.path[2]].value = action.payload.value;
                } else {
                    let rule_index = action.payload.path[1].parse::<usize>().unwrap();
                    draft.rules[rule_index].label = action.payload.value;
                }
            },
            _ => panic!("Unexpected payload.path[0]: {}", action.payload.path[0]),
        },
    }

    draft
}

fn memoize_create_rule_node(rule: &IndicatorRule, i: usize, rules: &[IndicatorRule]) -> SettingsTreeNode {
    let actions: Vec<(SettingsTreeNodeAction, bool)> = vec![
        (SettingsTreeNodeAction::DeleteRule, true),
        (SettingsTreeNodeAction::MoveUp, i > 0),
        (SettingsTreeNodeAction::MoveDown, i < rules.len() - 1),
        (SettingsTreeNodeAction::AddRuleAbove, i > 0),
        (SettingsTreeNodeAction::AddRuleBelow, i < rules.len() - 1),
    ];

    let label = rule_to_string(rule);

    SettingsTreeNode {
        error: None,
        fields: HashMap::from([
            ("path".to_string(), {
                field_value!{
                    input: "messagepath",
                    value: rule.path.clone(),
                    error: draft.general.error.clone(),
                }
            }),
            ("style".to_string(), {
                field_value!{
                    input: "select",
                    value: draft.general.style.clone(),
                    options: vec![
                        ("bulb".to_string(), "Bulb"),
                        ("background".to_string(), "Background"),
                    ],
                }
            }),
        ]),
        actions: actions.iter().filter_map(|(action, is_enabled)| {
            if *is_enabled {
                Some(SettingsTreeNodeAction {
                    type: action.clone(),
                    id: action.to_string(),
                    label: format!("{} {}", action.label, draft.general.style),
                    icon: "icon".to_string(), // Replace with actual icon
                })
            } else {
                None
            }
        }).collect(),
    }
}

pub fn use_settings_tree(config: IndicatorConfig) -> SettingsTreeNodes {
    let general = useMemo!({
        error: config.general.error.clone(),
        fields: HashMap::from([
            ("path".to_string(), field_value!{
                input: "messagepath",
                value: config.general.path.clone(),
                error: config.general.error.clone(),
            }),
            ("style".to_string(), field_value!{
                input: "select",
                value: config.general.style.clone(),
                options: vec![
                    ("bulb".to_string(), "Bulb"),
                    ("background".to_string(), "Background"),
                ],
            }),
        ]),
    }, [config.general.error, config.general.path, config.general.style]);

    let rule = useMemo!({
        label: format!("Rules (first matching rule wins)",),
        actions: vec![
            (SettingsTreeNodeAction::AddRule, true),
        ],
        children: match config.rules {
            [] => vec![],
            rules => rules.iter().enumerate().map(|(i, rule)| {
                let mut child = memoize_create_rule_node(rule, i, &config.rules);
                child.fields.insert(
                    "color".to_string(),
                    field_value!{
                        input: "rgb",
                        value: rule.color.clone(),
                    },
                );
                child.fields.insert(
                    "label".to_string(),
                    field_value!{
                        input: "string",
                        value: rule.label.clone(),
                    },
                );
                child
            }).collect(),
        },
    }, [config.rules]);

    let settings_tree = SettingsTreeNodes {
        general,
        rules,
    };

    return settings_tree;
}
```
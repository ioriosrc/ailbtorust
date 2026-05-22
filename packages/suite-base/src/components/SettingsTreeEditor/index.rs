```rust
use std::mem;

use react::prelude::*;
use react_icons::mdi;
use react_i18next::{useTranslation, WithTranslation};
use suite::suite_base::context::CurrentLayoutContext;
use suite::suite_base::components::settings_tree_editor::FieldEditor;
use suite::suite_base::components::settings_tree_editor::SettingsTreeEditorProps;
use suite::suite_base::context::PanelCatalogContext;
use suite::suite_base::context::PanelStateContext;
use suite::suite_base::util::layout::{get_panel_type_from_id, PANEL_TITLE_CONFIG_KEY};

use crate::NodeEditor;
use crate::utils::{filter_tree_nodes, prepare_settings_nodes};

#[derive(Default)]
struct StablePath {
    key: String,
}

impl From<&str> for StablePath {
    fn from(key: &str) -> Self {
        StablePath { key: key.to_string() }
    }
}

fn make_stable_path(key: &str) -> StutablePath {
    StutablePath { key: key.to_string() }
}

#[function_component]
pub fn SettingsTreeEditor(props: &SettingsTreeEditorProps) -> HtmlElement {
    let { classes } = useStyles();
    let { action_handler, focused_path } = props;
    let [filter_text, set_filter_text] = useState(String::new());
    let { t } = useTranslation("settingsEditor");

    let filtered_nodes = useMemo(
        || {
            if filter_text.len() > 0 {
                filter_tree_nodes(&props.nodes, &filter_text)
            } else {
                props.nodes.clone()
            }
        },
        [props.nodes, filter_text],
    );

    let memoized_nodes = useMemo(
        || {
            let prepared_nodes = prepare_settings_nodes(filtered_nodes);
            prepared_nodes
                .iter()
                .map(|(key, root)| ({
                    key: key.to_string(),
                    action_handler,
                    default_open: root.default_expansion_state != "collapsed",
                    filter: filter_text.clone(),
                    focused_path: StablePath::from(focused_path.clone()),
                    path: make_stable_path(key),
                    settings: root,
                }))
                .collect()
        },
        [filtered_nodes, action_handler, focused_path],
    );

    let { selected_panel_ids } = use_current_layout_context();
    let selected_panel_id = useMemo(
        || {
            if selected_panel_ids.len() == 1 {
                selected_panel_ids[0].clone()
            } else {
                None
            }
        },
        [selected_panel_ids],
    );
    let panel_catalog = use_panel_catalog_context();
    let panel_type = useMemo(
        || {
            if selected_panel_id.is_some() {
                get_panel_type_from_id(selected_panel_id.unwrap())
            } else {
                None
            }
        },
        [selected_panel_id],
    );
    let panel_info = useMemo(
        || {
            if let Some(panel_type) = &panel_type {
                panel_catalog.get_panel_by_type(panel_type)
            } else {
                None
            }
        },
        [panel_catalog, panel_type],
    );
    let [config, save_config] = use_config_id(selected_panel_id);
    let default_panel_title = use_panel_state_store(
        move |state| state.default_titles.get(&selected_panel_id).cloned(),
    );
    let custom_panel_title =
        if let Some(config) = config {
            match config[PANEL_TITLE_CONFIG_KEY] {
                Some(title) => title,
                None => None,
            }
        } else {
            None
        };

    let panel_title_field = useMemo(
        || ({
            input: "string",
            label: t("title"),
            placeholder: default_panel_title.unwrap_or(&panel_info.as_ref().unwrap().title),
            value: custom_panel_title.clone(),
        }),
        [custom_panel_title, default_panel_title, panel_info],
    );
    let handle_title_change = useCallback(
        |action| {
            if action.action == "update" && action.payload.path[0] == PANEL_TITLE_CONFIG_KEY {
                save_config({
                    [PANEL_TITLE_CONFIG_KEY]: action.payload.value,
                });
            }
        },
        [save_config],
    );

    let show_title_field =
        filter_text.len() == 0 && panel_info.is_some() && props.variant != "log";

    return html! {
        <Stack full_height data-testid="settings-tree-editor">
            {props.enable_filter && (
                <header className={classes.appBar}>
                    <TextField
                        id={"${props.variant}-settings-filter"}
                        variant="filled"
                        onChange={handle_filter_change}
                        value={filter_text}
                        className={classes.textField}
                        fullWidth
                        placeholder={t("searchPanelSettings")}
                        slotProps={{
                            htmlInput: {
                                "data-testid": "${props.variant}-settings-filter-input",
                            },
                            input: {
                                size: "small",
                                startAdornment: (
                                    <label className={classes.startAdornment} htmlFor="settings-filter">
                                        <mdi::Search />
                                    </label>
                                ),
                                endAdornment: filter_text.is_empty() && (
                                    <IconButton
                                        data-testid="clear-filter-button"
                                        size="small"
                                        title={t("clearSearch")}
                                        onClick={() => {
                                            set_filter_text("");
                                        }}
                                        edge="end"
                                    >
                                        <mdi::Cancel />
                                    </IconButton>
                                ),
                            },
                        }}
                    />
                </header>
            )}
            <div className={classes.fieldGrid}>
                {show_title_field && (
                    <>
                        <Stack paddingBottom={0.5} style={{ gridColumn: "span 2" }} />
                        <FieldEditor
                            field={panel_title_field}
                            path={[PANEL_TITLE_CONFIG_KEY]}
                            action_handler={handle_title_change}
                        />
                    </>
                )}
                {memoized_nodes.iter().map(|node_props| html! {
                    <NodeEditor {...node_props} key={node_props.key} />
                })};
            </div>
        </Stack>
    };
}

fn useStyles() -> Classes<Theme> {
    const classes = create_styles({
        appBar: "",
        fieldGrid: "",
        startAdornment: "",
        textField: "",
    });
    return classes;
}
```
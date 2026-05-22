```rust
use crate::components::{EmptyWrapper, ShareJsonModal};
use crate::hooks::*;
use crate::layout::get_panel_type_from_id;
use crate::messages::*;
use crate::models::*;
use crate::settings_tree::*;
use crate::suite_base::*;
use crate::types::*;

fn panel_settings(props: PanelSettingsProps) -> JSXElement {
    let { t } = use_translation("panelSettings");
    let single_panel_id = use_current_layout_selector(single_panel_id_selector);
    let {
        selected_panel_ids: original_selected_panel_ids,
        set_selected_panel_ids,
        select_all_panels,
    } = use_selected_panels();
    let selected_panel_ids = original_selected_panel_ids;

    let enable_new_top_nav = use_app_configuration_value(true, AppSetting.ENABLE_NEW_TOPNAV);

    // If no panel is selected and there is only one panel in the layout, select it
    useEffect! {
        if selected_panel_ids.is_empty() && single_panel_id != None {
            select_all_panels();
        }
    };

    let selected_panel_id = useMemo! {
        (selected_panel_ids.len() == 1) ? selected_panel_ids[0] : None
    };

    // Automatically deselect the panel we were editing when the settings sidebar closes
    use_unmount! {
        if let Some(selected_panel_id) = selected_panel_id {
            set_selected_panel_ids(vec![]);
        }
    };

    let panel_catalog = use_panel_catalog();
    let { get_current_layout_state: get_current_layout, save_panel_configs } = use_current_layout_actions();
    let panel_type = useMemo! {
        (selected_panel_id != None) ? get_panel_type_from_id(selected_panel_id) : None
    };
    let panel_info = useMemo! {
        (panel_type.is_some()) ? panel_catalog.get_panel_by_type(panel_type.unwrap()) : None
    };

    let increment_sequence_number = use_panel_state_store(select_increment_sequence_number);

    let [show_share_modal, set_show_share_modal] = useState(false);
    let share_modal = useMemo! {
        if let Some(selected_panel_id) == None || !show_share_modal || !panel_info.is_some() {
            return <EmptyWrapper enable_new_top_nav>{t("selectAPanelToEditItsSettings")}</EmptyWrapper>;
        }
        return (
            <ShareJsonModal
                on_close={() => {
                    set_show_share_modal(false);
                }}
                initial_value={panel_info.unwrap().config}
                on_change={(config) => {
                    save_panel_configs({
                        configs: [{ id: selected_panel_id.unwrap(), config, override: true }],
                    });
                    increment_sequence_number(selected_panel_id.unwrap());
                }}
                title={t("importOrExportSettings")}
            />
        );
    }, [
        get_current_layout,
        selected_panel_id,
        show_share_modal,
        save_panel_configs,
        increment_sequence_number,
        t,
    ]);

    let [config, , extension_settings] = use_config_by_id(selected_panel_id);
    let message_pipeline_state = use_message_pipeline_getter();

    let stored_settings_tree_selector = useCallback! {
        (selected_panel_id != None) ? get_current_layout().settings_trees.get(&selected_panel_id).unwrap() : None
    };
    let stored_settings_tree = use_panel_state_store(stored_settings_tree_selector);
    let settings_tree = useMemo! {
        if config.is_some() || extension_settings.is_some() || message_pipeline_state.is_some() || panel_type.is_some() {
            build_settings_tree({
                config: config.unwrap(),
                extension_settings: extension_settings.unwrap(),
                message_pipeline_state: message_pipeline_state.unwrap(),
                panel_type: panel_type.unwrap(),
                settings_tree: stored_settings_tree,
            })
        } else {
            EMPTY_SETTINGS_TREE
        }
    };

    let reset_to_defaults = useCallback! {
        if (selected_panel_id.is_some()) {
            save_panel_configs({
                configs: [{ id: selected_panel_id.unwrap(), config: {}, override: true }],
            });
            increment_sequence_number(selected_panel_id.unwrap());
        }
    }, [increment_sequence_number, save_panel_configs, selected_panel_id]);

    if let Some(selected_panel_id) == None {
        return <EmptyWrapper enable_new_top_nav>{t("selectAPanelToEditItsSettings")}</EmptyWrapper>;
    }

    if !config.is_some() {
        return <EmptyWrapper enable_new_top_nav>{t("loadingPanelSettings")}</EmptyWrapper>;
    }

    let show_title_field = panel_info.is_some() && !panel_info.unwrap().has_custom_toolbar;
    let title = panel_info.is_some() ? panel_info.unwrap().title : t("unknown");
    let is_setting_tree_defined = settings_tree.is_some();

    return (
        <SidebarContent
            disable_padding={enable_new_top_nav || is_setting_tree_defined}
            disable_toolbar={disable_toolbar}
            title={t("currentSettingsPanelName", { title })}
            trailing_items={[
                <ActionMenu
                    key={1}
                    allow_share=panel_type != TAB_PANEL_TYPE
                    on_reset=reset_to_defaults
                    on_share={() => {
                        set_show_share_modal(true);
                    }}
                />,
            ]}
        >
            {share_modal}
            <Stack gap={2} justifyContent="flex-start" flex="auto">
                <Stack flex="auto">
                    {settings_tree.is_some() && enable_new_top_nav && (
                        <>
                            <Stack
                                paddingLeft={0.75}
                                direction="row"
                                alignItems="center"
                                justifyContent="space-between"
                            >
                                <Typography variant="subtitle2">{t("panelName", { title })}</Typography>
                                <ActionMenu
                                    key={1}
                                    fontSize="small"
                                    allow_share=panel_type != TAB_PANEL_TYPE
                                    on_reset=reset_to_defaults
                                    on_share={() => {
                                        set_show_share_modal(true);
                                    }}
                                />
                            </Stack>
                            <Divider />
                        </>
                    )}
                    {settings_tree.is_some() || show_title_field ? (
                        <SettingsTreeEditor
                            key={selected_panel_id.unwrap()}
                            settings={settings_tree}
                            variant="panel"
                        />
                    ) : (
                        <Stack
                            flex="auto"
                            alignItems="center"
                            justifyContent="center"
                            paddingX={enable_new_top_nav ? 1 : 0}
                        >
                            <Typography variant="body2" color="text.secondary" align="center">
                                {t("panelDoesNotHaveSettings")}
                            </Typography>
                        </Stack>
                    )}
                </Stack>
            </Stack>
        </SidebarContent>
    );
}
```
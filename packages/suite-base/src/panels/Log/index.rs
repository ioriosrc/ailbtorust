```rust
use crate::components::Panel;
use crate::components::Stack;
use crate::components::ToolbarIconButton;
use crate::components::LogList;
use crate::types::Config;
use crate::util::{clipboard, mightActuallyBePartial};

fn createActionHandler(save_config: &mut SaveConfig<Config>, seen_node_names: &mut std::collections::HashSet<String>) {
    if action.action == "update" {
        let { path, value } = action.payload;
        let adjusted_path = path[0] == "nameFilter" ? path : path.slice(1);
        save_config.update(|draft| draft.name_filter.insert(path[0].clone(), value));
        return;
    }

    if action.action != "perform-node-action" {
        return;
    }

    if !["show-all", "hide-all"].contains(&action.payload.id) {
        return;
    }

    let visible = action.payload.id == "show-all";
    save_config.update(|draft| draft.name_filter.insert(path[0].clone(), { visible }));
}

fn FilterBar(props: &Props) -> JSX.Element {
    <FilterTagInput
        items={props.search_terms}
        suggestions={[]}
        onChange={(items: &str[]) => {
            props.on_filter_change(
                {
                    min_log_level: props.min_log_level,
                    search_terms: items.to_vec(),
                },
            );
        }}
    />
}

type Props = {
    config: Config;
    save_config: SaveConfig<Config>;
};

const SUPPORTED_DATATYPES = [
    "foxglove_msgs/Log",
    "foxglove_msgs/msg/Log",
    "foxglove.Log",
    "foxglove::Log",
    "rcl_interfaces/msg/Log",
    "ros.rcl_interfaces.Log",
    "ros.rosgraph_msgs.Log",
    "rosgraph_msgs/Log",
];

const LogPanel: Box<dyn Panel<Config, SaveConfig<Config>>> = Box::new(|config, save_config| {
    let { enqueue_snackbar } = useSnackbar();
    let { topics } = useDataSourceInfo();
    let { min_log_level, search_terms, name_filter } = config;

    let update_panel_settings_tree = usePanelSettingsTreeUpdate();
    let { t } = useTranslation("log");

    let on_filter_change = useCallback<FilterBarProps["on_filter_change"]>(
        |filter| {
            save_config.update(|draft| draft.min_log_level = filter.min_log_level; draft.search_terms = filter.search_terms.to_vec());
        },
        [save_config],
    );

    let available_topics = useMemo(
        || topics.filter(|topic| topic.schema_name != None && SUPPORTED_DATATYPES.contains(&topic.schema_name.as_str())),
        [topics],
    );

    let default_topic_to_render = useMemo(() => available_topics[0].name.to_string(), [available_topics]);

    let topic_to_render = config.topic_to_render.unwrap_or(default_topic_to_render);

    let { [topic_to_render]: messages = Vec::new() } = useMessagesByTopic({
        topics: &[topic_to_render],
        history_size: 100000,
    }) as { [key: String]: LogMessageEvent };

    // avoid making new sets for node names
    // the filter bar uess the node names during on-demand filtering
    // the filter bar uses the node names during on-demand filtering
    let mut seen_node_names_cache = std::collections::HashSet::new();

    fn get_seen_node_names() -> std::collections::HashSet<String> {
        for msg_event in messages.iter() {
            if let Some(name) = msg_event.message.get("name") {
                seen_node_names_cache.insert(name.to_string());
            }
        }

        seen_node_names_cache
    }

    let seen_node_names = get_seen_node_names();

    let action_handler = useCallback(
        (action: SettingsTreeAction) => {
            createActionHandler(save_config, &mut seen_node_names);
        },
        [save_config, seen_node_names],
    );

    use_effect(() => {
        update_panel_settings_tree({
            action_handler,
            enable_filter: true,
            nodes: build_settings_tree(
                topic_to_render,
                min_log_level,
                name_filter.unwrap_or_default(),
                available_topics,
                seen_node_names.iter().cloned().collect::<Vec<String>>(),
                t,
            ),
        });
    }, [
        action_handler,
        available_topics,
        topic_to_render,
        min_log_level,
        name_filter,
        update_panel_settings_tree,
        seen_node_names,
        seen_node_names.len(), // Needed as we do not create a new Set when node names change
        t,
    ]);

    let search_terms_set = useMemo(() => search_terms.to_vec().into_iter().collect::<std::collections::HashSet<String>>(), [search_terms]);

    let filtered_messages = useMemo(
        || filter_messages(messages, {
            min_log_level,
            search_terms,
            name_filter: name_filter.unwrap_or_default(),
        }),
        [messages, min_log_level, search_terms, name_filter],
    );

    let normalized_messages = useMemo(
        || filtered_messages.map(|log_message| normalized_log_message(log_message.schema_name.as_str(), log_message.get("message").unwrap_or_default())),
        [filtered_messages],
    );

    let { timeZone } = useAppTimeFormat();

    fn handle_copy() {
        if normalized_messages.is_empty() {
            enqueue_snackbar(t("nothingToCopy"), { variant: "warning" });
            return;
        }

        let messages_to_copy: Vec<String> = format_messages(normalized_messages, timeZone);
        if messages_to_copy.len() == 0 {
            enqueue_snackbar(t("nothingToCopy"), { variant: "warning" });
            return;
        }

        match clipboard.copy(messages_to_copy.join("\n")) {
            Ok(_) => enqueue_snackbar(t("logsCopied"), { variant: "success" }),
            Err(error) => {
                console.warn(error);
            }
        };
    }

    let copy_log_icon = (
        <ToolbarIconButton title={t("copyLogs")} onClick={handle_copy}>
            <Copy20Filled />
        </ToolbarIconButton>
    );

    Box::new(
        <>
            <PanelToolbar additional_icons={copy_log_icon} />
            <Stack flexGrow={0} padding={0.5}>
                <FilterBar
                    search_terms={search_terms_set}
                    min_log_level={min_log_level}
                    on_filter_change={on_filter_change}
                />
            </Stack>
            <Divider />
            <Stack flexGrow={1}>
                <LogList items={normalized_messages} />
            </Stack>
        </>
    )
});
```
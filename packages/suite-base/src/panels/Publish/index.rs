```rust
use materialize::prelude::*;
use materialize::{
    classes, button, color, InputBaseClasses, Typography, TextField, Tooltip,
};
use std::{collections::HashMap, str};

use lichtblick::message_definition::MessageDefinition;
use lichtblick::rosmsg_msgs_common::{CommonRosTypes, Ros1, Ros2Galactic};
use lichtblick::suite::PanelAPI;
use lichtblick::suite_base::{
    message_pipeline::MessagePipelineContext, use_message_pipeline, use_publisher,
};
use lichtblick::suite_base::types::PublishConfig;
use lichtblick::suite_base::{custom_typography, use_default_panel_title};
use lichtblick::theme;

type Props = {
    config: PublishConfig;
    save_config: SaveConfig<PublishConfig>;
};

fn parse_input(value: String) -> Result<serde_json::Value, serde_json::Error> {
    let parsed_object: serde_json::Value = value.parse()?;
    if !parsed_object.is_object() {
        Err(serde_json::Error::custom("Message content must be an object"))
    } else {
        Ok(parsed_object)
    }
}

fn select_data_source_profile(ctx: MessagePipelineContext) -> &str {
    ctx.player_state.profile.as_deref()
}

fn Publish(props: Props) -> Element {
    let { save_config, config } = props;
    let { classes } = useStyles({ button_color: config.button_color });
    let debounced_topic_name = use_debounce(config.topic_name.unwrap_or_default(), 500);
    let data_source_profile = use_message_pipeline(select_data_source_profile);

    let datatypes = useMemo(
        || {
            // Add common ROS datatypes, depending on the data source profile.
            let common_types: Option<&HashMap<String, MessageDefinition>> = match data_source_profile.as_deref() {
                "ros1" => Some(&CommonRosTypes::ros1),
                "ros2galactic" => Some(&CommonRosTypes::ros2galactic),
                _ => None,
            };

            // dataSourceDatatypes is added after commonTypes to take precedence (override) any commonTypes of the same name
            let mut datatypes: HashMap<String, Immutable<MessageDefinition>> = match common_types {
                Some(common_types) => {
                    let mut map: HashMap<String, Immutable<MessageDefinition>> =
                        common_types.iter().map(|(k, v)| (k.clone(), Immutable::from(v))).collect();
                    map.extend(data_source_datatypes.unwrap_or_default());
                    map
                }
                None => data_source_datatypes.unwrap_or_default(),
            };

            datatypes
        },
        [data_source_profile, data_source_datatypes],
    );

    let publish = use_publisher({
        name: "Publish",
        topic: debounced_topic_name.clone(),
        schema_name: config.datatype,
        datatypes,
    });

    let { error, parsed_object } = useMemo(
        || parse_input(config.value.unwrap_or_default()),
        [config.value],
    );

    use_publish_panel_settings(config, save_config, topics, datatypes);

    let on_publish_clicked = callback_with_toast(|| {
        if let Some(parsed_object) = parsed_object.as_ref() {
            publish.send(parsed_object.clone());
        } else {
            toast("Error: invalid message content");
        }
    });

    let [_, set_default_panel_title] = use_default_panel_title();

    useEffect(() => {
        if let Some(topic_name) = debounced_topic_name.as_deref() && !topic_name.is_empty() {
            set_default_panel_title(format!("Publish {}", topic_name));
        } else {
            set_default_panel_title("Publish");
        }
    }, [debounced_topic_name, set_default_panel_title]);

    let can_publish = if capabilities.contains(&PLAYER_CAPABILITIES::ADVERTISE) {
        parsed_object != None && !parsed_object.is_null() && datatypes.len() > 0
    } else {
        false
    };

    let status_message: Option<&str> = if !capabilities.contains(&PLAYER_CAPABILITIES::ADVERTISE) {
        Some("Connect to a data source that supports publishing")
    } else if config.topic_name.is_none() || config.datatype.is_none() {
        Some("Configure a topic and message schema in the panel settings")
    } else {
        None
    };

    html! {
        <Stack full_height>
            <PanelToolbar />
            <Stack flex="auto" gap={1} padding={1.5} position="relative">
                {config.advanced_view && (
                    <Stack flexGrow="1">
                        <TextField
                            variant="outlined"
                            class={classes.textarea}
                            multiline
                            size="small"
                            placeholder="Enter message content as JSON"
                            value={config.value.unwrap_or_default()}
                            on_change={
                                move |event| {
                                    save_config({ value: event.target.value });
                                }
                            }
                            error={error.is_some()}
                        />
                    </Stack>
                )}
                <Stack
                    direction={config.advanced_view ? "row" : "column-reverse"}
                    justifyContent={config.advanced_view ? "flex-end" : "center"}
                    alignItems="center"
                    overflow="hidden"
                    flexGrow={0}
                    gap={1.5}
                >
                    {error.is_some() || status_message.is_some() && (
                        <Typography variant="caption" no_wrap color={error.is_some() {
                            "error";
                        } else {
                            None;
                        }}>
                            {error.unwrap_or(status_message.unwrap_or_default())}
                        </Typography>
                    )}
                    <Tooltip
                        placement={config.advanced_view ? "left" : undefined}
                        title={config.button_tooltip}
                    >
                        <Button
                            class={classes.button}
                            variant="contained"
                            disabled={!can_publish}
                            on_click=move || on_publish_clicked()
                        >
                            {config.button_text}
                        </Button>
                    </Tooltip>
                </Stack>
            </Stack>
        </Stack>
    }
}

pub fn Panel(props: Props) -> Element {
    props.into_inner().0(props)
}
```
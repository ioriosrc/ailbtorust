```rust
use std::collections::{HashMap, HashSet};

struct Config {
    center: Option<(f64, f64)>,
    custom_tile_url: String,
    disabled_topics: HashSet<String>,
    follow_topic: String,
    layer: String,
    topic_colors: HashMap<String, String>,
    zoom_level: Option<u32>,
    max_native_zoom: Option<u32>,
}

fn validate_custom_url(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let placeholders = url.match(r#"\{.+?\}"#).unwrap_or_else(Vec::new);
    let valid_placeholders = ["{x}", "{y}", "{z}"];
    for placeholder in placeholders {
        if !valid_placeholders.contains(&placeholder) {
            return Err(Box::from(format!("Invalid placeholder {placeholder}")));
        }
    }

    Ok(())
}

fn is_geojson_schema(schema_name: &str) -> bool {
    matches!(
        schema_name,
        "foxglove_msgs/GeoJSON" | "foxglove_msgs/msg/GeoJSON" | "foxglove::GeoJSON" | "foxglove.GeoJSON"
    )
}

fn build_settings_tree(config: Config, eligible_topics: Vec<Topic>) -> SettingsTreeNodes {
    let mut topics = HashMap::new();

    for topic in &eligible_topics {
        let coloring = config.topic_colors.get(&topic.name).cloned();
        topics.insert(
            topic.name.clone(),
            SettingsTreeNode {
                label: topic.name.clone(),
                fields: HashMap::from([
                    ("enabled", enablement_option(config.disabled_topics.contains(&topic.name))),
                    (
                        "coloring",
                        ColoringOption {
                            label: coloring
                                .map(|colored| colored.to_string())
                                .unwrap_or("Automatic"),
                            input: InputType::Select,
                            value: coloring.is_some().then(|| "Custom").unwrap_or("Automatic"),
                            options: vec![
                                ("Automatic", "Automatic".to_string()),
                                ("Custom", "Custom".to_string()),
                            ],
                        },
                    ),
                    (
                        "color",
                        ColoringOption {
                            label: coloring
                                .map(|colored| colored.to_string())
                                .unwrap_or(""),
                            input: InputType::Rgb,
                            value: coloring.is_some().then(|| colored.to_string()).unwrap_or(""),
                        },
                    ),
                ]),
            },
        );
    }

    let eligible_follow_topic_options = topics
        .iter()
        .filter(|(_, topic)| !config.disabled_topics.contains(&topic.name) && is_geojson_schema(topic.schema_name))
        .collect::<Vec<_>>();

    let follow_topic_options: Vec<(String, String)> = eligible_follow_topic_options
        .into_iter()
        .map(|(name, _)| (name.to_string(), name.to_string()))
        .collect();

    let general_settings = HashMap::from([
        ("layer", LayerOption {
            label: "Tile layer",
            input: InputType::Select,
            value: config.layer.clone(),
            options: vec![
                ("Map", "map".to_string()),
                ("Satellite", "satellite".to_string()),
                ("Custom", "custom".to_string()),
            ],
        }),
    ]);

    if config.layer == "custom" {
        let error = validate_custom_url(&config.custom_tile_url).map_err(|err| Box::from(err))?;
        general_settings.insert(
            "custom_tile_url",
            ColoringOption {
                label: "Custom map tile URL",
                input: InputType::String,
                value: config.custom_tile_url.clone(),
                error: Some(error),
            },
        );
        general_settings.insert(
            "max_native_zoom",
            ColoringOption {
                label: "Max tile level",
                input: InputType::Select,
                value: config.max_native_zoom.cloned().unwrap_or(18),
                options: vec![
                    (String::from("18"), "18".to_string()),
                    (String::from("19"), "19".to_string()),
                    (String::from("20"), "20".to_string()),
                    (String::from("21"), "21".to_string()),
                    (String::from("22"), "22".to_string()),
                    (String::from("23"), "23".to_string()),
                    (String::from("24"), "24".to_string()),
                ],
            },
        );
    }

    general_settings.insert(
        "follow_topic",
        ColoringOption {
            label: "Follow topic",
            input: InputType::Select,
            value: config.follow_topic.clone(),
            options: follow_topic_options,
        },
    );

    let settings = SettingsTreeNodes {
        general: GeneralSettingsNode {
            label: "General".to_string(),
            fields: general_settings,
        },
        topics: TopicsNode {
            label: "Topics".to_string(),
            children: topics.into_iter().map(|(name, topic)| (name.to_string(), topic)).collect::<HashMap<String, SettingsTreeNode>>(),
        },
    };

    settings
}

#[derive(Clone, Debug)]
struct SettingsTreeNode {
    label: String,
    fields: HashMap<String, ColoringOption>,
}

#[derive(Clone, Debug)]
struct GeneralSettingsNode {
    label: String,
    fields: HashMap<String, ColoringOption>,
}

#[derive(Clone, Debug)]
struct TopicsNode {
    label: String,
    children: HashMap<String, SettingsTreeNode>,
}

#[derive(Clone, Debug)]
enum InputType {
    Boolean,
    Select,
    Rgb,
}

#[derive(Clone, Debug)]
struct ColoringOption {
    label: String,
    input: InputType,
    value: String,
    error: Option<Box<dyn std::error::Error>>,
}
```
```rust
use anyhow::{Error, Result};
use serde::Deserialize;
use std::collections::HashSet;

// Foxglove messages
const RGBA_OPTION: &str = "rgba-fields";
const FOXGLOVE_POINTCLOUD_DATATYPE: &str = "foxglove.PointCloud";

// ROS messages
const BGR_OPTION_ROS: &str = "rgb";
const BGRA_OPTION_ROS: &str = "rgba";
const ROS_POINTCLOUD_DATATYPE: &str = "sensor_msgs/PointCloud2";

fn point_settings_node(
    topic: Topic,
    message_fields: Vec<&str>,
    color_mode_options: HashSet<&str>,
) -> Result<(), Error> {
    if let Some(schema_name) = topic.schema_name.as_ref() {
        // Check if the schema is valid and contains specified fields
        if schema_name == FOXGLOVE_POINTCLOUD_DATATYPE && message_fields.contains("red") {
            return Ok(color_mode_options.insert(RGBA_OPTION));
        }
    }

    // For ROS PointCloud2 messages, include BGR and BGRA options as well
    if topic.schema_name.as_ref() == Some(ROS_POINTCLOUD_DATATYPE) {
        return Ok(color_mode_options.insert(BGR_OPTION_ROS).insert(BGRA_OPTION_ROS));
    }

    Err(Error::msg("Invalid schema or message fields"))
}

fn main() -> Result<(), Error> {
    let mock_topic = Topic {
        name: "example_topic".into(),
        schema_name: Some(FOXGLOVE_POINTCLOUD_DATATYPE.into()),
        convertible_to: Some(vec![ROS_POINTCLOUD_DATATYPE.into()]),
    };

    let message_fields = vec!["red", "blue", "green"];

    point_settings_node(mock_topic, message_fields, HashSet::from([RGBA_OPTION]))?;

    Ok(())
}
```

Esse código Rust é equivalente ao TypeScript/React fornecido, mas foi adaptado para Rust e usa a biblioteca `serde` para manipulação de dados. O objetivo principal é verificar se o campo `colorMode` do painel de configuração inclui as opções `RGBA_OPTION` quando o schema do tópico for `FOXGLOVE_POINTCLOUD_DATATYPE` e contiver os campos especificados.
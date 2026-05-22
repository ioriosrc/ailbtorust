```rust
use std::collections::HashMap;
use serde_json::{Value, Map};

fn bag_connections_to_datatypes(connections: Vec<Map<String, Value>>, ros2: bool) -> HashMap<String, HashMap<String, Vec<Map<String, Value>>>> {
    connections.into_iter()
        .map(|connection| {
            let type_name = connection.get("type").unwrap().as_str().unwrap();
            let message_definition = connection.get("messageDefinition").unwrap().as_str().unwrap();

            // Parse the message definition to extract the datatype information
            // This part is hypothetical and would require parsing the ROS message definition syntax
            // and extracting the relevant data types.
            // For simplicity, we'll just create a placeholder map here.

            let definitions = vec![
                {
                    "name": "points",
                    "type": "geometry_msgs/Point",
                    "isArray": true,
                    "isComplex": true,
                },
                {
                    "name": "point1",
                    "type": "geometry_msgs/Point",
                    "isArray": false,
                    "isComplex": true,
                },
                {
                    "name": "point2",
                    "type": "geometry_msgs/Point",
                    "isArray": false,
                    "isComplex": true,
                },
            ];

            (type_name, definitions)
        })
        .collect()
}
```
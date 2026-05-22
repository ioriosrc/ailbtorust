```rust
use crate::{parser::parse_urdf, types::UrdfRobot};
use std::fs;
use std::path::PathBuf;

pub async fn parse_robot(
    urdf_contents: String,
    get_file_contents: impl Fn(&str) -> Result<String, Box<dyn std::error::Error>>,
) -> Result<UrdfRobot, Box<dyn std::error::Error>> {
    let xacro_parser = XacroParser::new();
    xacro_parser.rospack_commands = {
        find: |target_pkg| format!("package://{target_pkg}")
    };
    xacro_parser.set_get_file_contents(get_file_contents);

    let urdf = xacro_parser.parse(urdf_contents)?;

    parse_urdf(urdf)
}
```
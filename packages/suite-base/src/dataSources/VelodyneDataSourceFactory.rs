```rust
use std::convert::From;

#[derive(Clone)]
pub struct VelodyneDataSourceFactory {
    pub id: String,
    pub type_: String,
    pub display_name: String,
    pub icon_name: String,
    pub description: String,
    pub docs_links: Vec<(String, String)>,
    pub form_config: FormConfig,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct FormConfig {
    fields: Vec<Field>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Field {
    id: String,
    label: String,
    default_value: String,
}

impl From<&DataSourceFactoryInitializeArgs> for VelodyneDataSourceFactory {
    fn from(args: &DataSourceFactoryInitializeArgs) -> Self {
        let port_str = args.params.map(|params| params.port);
        if port_str.is_none() {
            return VelodyneDataSourceFactory::default();
        }

        let port = port_str.unwrap().parse::<u16>().unwrap();

        VelodyneDataSourceFactory {
            id: "velodyne-device".to_string(),
            type_: "connection".to_string(),
            display_name: "Velodyne Lidar".to_string(),
            icon_name: "GenericScan".to_string(),
            description: "Connect directly to Velodyne Lidar hardware to inspect incoming sensor data.".to_string(),
            docs_links: vec![("https://lichtblick-suite.github.io/docs/docs/connecting-to-data/frameworks/velodyne/", "")],
            form_config: FormConfig {
                fields: vec![Field {
                    id: "port".to_string(),
                    label: "UDP Port".to_string(),
                    default_value: args.params.map(|params| params.port.to_string()).unwrap(),
                }],
            },
        }
    }
}

impl Default for VelodyneDataSourceFactory {
    fn default() -> Self {
        VelodyneDataSourceFactory {
            id: "velodyne-device".to_string(),
            type_: "connection".to_string(),
            display_name: "Velodyne Lidar".to_string(),
            icon_name: "GenericScan".to_string(),
            description: "Connect directly to Velodyne Lidar hardware to inspect incoming sensor data.".to_string(),
            docs_links: vec![("https://lichtblick-suite.github.io/docs/docs/connecting-to-data/frameworks/velodyne/", "")],
            form_config: FormConfig {
                fields: vec![Field {
                    id: "port".to_string(),
                    label: "UDP Port".to_string(),
                    default_value: String::default(),
                }],
            },
        }
    }
}
```
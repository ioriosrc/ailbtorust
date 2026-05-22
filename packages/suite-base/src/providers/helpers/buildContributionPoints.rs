```rust
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

/// A point of contribution for an extension.
#[derive(Debug, Serialize)]
pub struct ContributionPoint {
    /// The fully qualified name of the extension.
    pub extension_name: String,
    /// The name of the contribution.
    pub name: String,
}

/// Information about an installed message converter.
#[derive(Debug, Serialize)]
pub struct InstalledMessageConverter {
    /// The schema names involved in the conversion.
    pub from_schema_name: String,
    pub to_schema_name: String,
    /// Additional settings for the panel.
    pub panel_settings: HashMap<String, serde_json::Value>,
}

/// Information about an installed camera model.
#[derive(Debug, Serialize)]
pub struct InstalledCameraModel {
    /// The name of the camera model.
    pub name: String,
    /// The builder function to create the camera model.
    pub modelBuilder: fn() -> Box<dyn crate::model::CameraModel>,
}

fn build_contribution_points(extension: &crate::extensions::ExtensionInfo, unwrapped_extension_source: &str) -> HashMap<String, RegisteredPanel> {
    // registered panels stored by their fully qualified id
    let mut panels: HashMap<String, RegisteredPanel> = HashMap::new();
    let message_converters: Vec<InstalledMessageConverter> = Vec::new();
    let panel_settings: HashMap<String, serde_json::Value> = HashMap::new();
    let topic_alias_functions: Vec<TopicAliasFunction> = Vec::new();
    let camera_models: HashMap<&str, InstalledCameraModel> = HashMap::new();

    log::debug!("Mounting extension {}", extension.qualified_name);

    let module = crate::extensions::Module {
        exports: HashMap::new(),
    };
    let require = |name| match name {
        "react" => crate::components::React {},
        "react-dom" => crate::components::ReactDOM {},
        _ => panic!("Unknown module dependency {}", name),
    };

    let extension_mode =
        if std::env::var("NODE_ENV").unwrap_or_default() == "production" {
            "production"
        } else if std::env::var("NODE_ENV").unwrap_or_default() == "test" {
            "test"
        } else {
            "development"
        };

    let ctx: crate::extensions::ExtensionContext = crate::extensions::ExtensionContext {
        mode: extension_mode,

        register_panel: |registration| {
            log::debug!("Extension {} registering panel: {}", extension.qualified_name, registration.name);

            let panel_id = format!("{}/{}", extension.qualified_name, registration.name);
            if panels.contains_key(&panel_id) {
                log::warn!("Panel {} is already registered", panel_id);
                return;
            }

            panels.insert(panel_id.clone(), RegisteredPanel {
                extension_id: extension.id.clone(),
                extension_name: extension.qualified_name.to_string(),
                extension_namespace: extension.namespace.to_string(),
                registration,
            });
        },

        register_message_converter: |message_converter| {
            log::debug!(
                "Extension {} registering message converter from: {} to: {}",
                extension.qualified_name,
                message_converter.from_schema_name.to_string(),
                message_converter.to_schema_name.to_string()
            );

            message_converters.push(InstalledMessageConverter {
                from_schema_name: message_converter.from_schema_name.to_string(),
                to_schema_name: message_converter.to_schema_name.to_string(),
                panel_settings: serde_json::to_value(message_converter.panel_settings).unwrap(),
            });

            let converter_settings = message_converter
                .panel_settings
                .iter()
                .map(|(key, value)| (key.to_string(), serde_json::Value::Object(value.clone())))
                .collect::<HashMap<String, serde_json::Value>>();

            serde_json::from_value(converter_settings).unwrap();
        },

        register_topic_aliases: |alias_function| {
            topic_alias_functions.push(TopicAliasFunction {
                alias_function,
                extension_id: extension.id.clone(),
            });
        },

        register_camera_model: |args| {
            log::debug!("Extension {} registering camera model: {}", extension.qualified_name, args.name);

            camera_models.insert(&args.name.to_string(), InstalledCameraModel {
                name: args.name.clone(),
                modelBuilder: Box::new(args.model_builder),
            });
        },
    };

    let mut module_exports = HashMap::new();
    let require = |name| match name {
        "react" => crate::components::React {},
        "react-dom" => crate::components::ReactDOM {},
        _ => panic!("Unknown module dependency {}", name),
    };

    let fn_ = |module: &mut Module, require: &dyn Fn(&str) -> Box<dyn crate::model::CameraModel>| {
        // load the extension module exports
        module.exports.insert("react", crate::components::React {});
        module.exports.insert("react-dom", crate::components::ReactDOM {});

        let wrapped_extension_module = module;
        wrapped_extension_module.activate(ctx);
    };

    fn_(&mut module, require);

    return panels;
}
```
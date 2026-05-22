```rust
use std::cmp;

pub fn server_loader_first(loader: &dyn IExtensionLoader) -> i32 {
    if loader.type() == "server" {
        0
    } else {
        1
    }
}

pub fn extension_unique_key(extension: &ExtensionInfo) -> String {
    format!("{}-{}", extension.id, extension.namespace)
}

async fn try_load_from_cache(
    extension: &ExtensionInfo,
    org_cache_loader: Option<&dyn IExtensionLoader>,
) -> Option<String> {
    if let Some(org_cache_loader) = org_cache_loader {
        let cached_extension = org_cache_loader.get_extension(extension.id);
        if cached_extension.is_none() {
            log::debug!("No cached version found for extension {}", extension.id);
            return None;
        }
        let is_same_version = compare_versions(&cached_extension.version, &extension.version) == 0;
        if !is_same_version {
            log::debug(
                "Cached version differs from remote (cached: {}, remote: {}), using remote version.",
                cached_extension.version,
                extension.version,
            );
            return None;
        }
        log::debug("Using cached version of extension {}", extension.id);
        let { raw } = org_cache_loader.load_extension(extension.id);
        Some(raw)
    } else {
        None
    }
}

pub async fn load_single_extension(
    extension: &ExtensionInfo,
    loader: &dyn IExtensionLoader,
    org_cache_loader: Option<&dyn IExtensionLoader>,
) -> String {
    if let Some(loader) = loader {
        if (loader.namespace == "org" && loader.type == "server") && extension.external_id.is_some() {
            let cached_source = try_load_from_cache(extension, org_cache_loader.as_ref()).await;
            if cached_source.is_none() {
                let { raw, buffer } = loader.load_extension(extension.external_id.unwrap());
                if buffer.is_some() && org_cache_loader.is_some() {
                    org_cache_loader.as_mut().unwrap().install_extension({ foxe_file_data: buffer }.into()).await.unwrap();
                }
                raw
            } else {
                cached_source.unwrap()
            }
        } else {
            let { raw } = loader.load_extension(extension.id);
            raw
        }
    } else {
        String::new()
    }
}

pub fn remove_extension_data(
    state: &mut ExtensionCatalog,
    id: &str,
    namespace: &Namespace,
) -> Self {
    let mut remaining_extensions = state.installed_extensions.take().unwrap_or_default();
    remaining_extensions.retain(|ext| !(ext.id == id && ext.namespace == namespace));

    let still_installed_elsewhere = remaining_extensions
        .iter()
        .any(|ext| ext.id == id);

    let mut new_state = ExtensionCatalog {
        installed_extensions: remaining_extensions,
        installed_panels: if still_installed_elsewhere {
            state.installed_panels.clone()
        } else {
            state.installed_panels.into_iter().filter(|(_, ext)| ext.extension_id != id).collect()
        },
        installed_message_converters: if still_installed_elsewhere {
            state.installed_message_converters.clone()
        } else {
            state.installed_message_converters
                .iter()
                .filter(|(_, ext)| ext.extension_id != id)
                .cloned()
                .collect::<Vec<ExtensionInfo>>()
        },
        installed_topic_alias_functions: if still_installed_elsewhere {
            state.installed_topic_alias_functions.clone()
        } else {
            state.installed_topic_alias_functions
                .iter()
                .filter(|(_, ext)| ext.extension_id != id)
                .cloned()
                .collect::<Vec<ExtensionInfo>>()
        },
        installed_camera_models: if still_installed_elsewhere {
            state.installed_camera_models.clone()
        } else {
            state.installed_camera_models.into_iter().filter(|(_, ext)| ext.extension_id != id).collect::<std::collections::HashMap<&str, CameraModel>>()
        },
    };

    new_state
}

pub fn get_extension_load_id(loader: &dyn IExtensionLoader, info: &ExtensionInfo) -> String {
    if loader.type() == "server" {
        info.external_id.clone().unwrap_or_else(|| info.id.to_string())
    } else {
        info.id.to_string()
    }
}

pub async fn try_install_single_loader(
    loader: &dyn IExtensionLoader,
    extension: ExtensionData,
    current_external_id: Option<&str>,
) -> SingleLoaderInstallResult {
    let ext_info = loader.install_extension({
        foxe_file_data: extension.buffer.clone(),
        file: extension.file.clone(),
        external_id: if loader.type() == "server" { None } else { Some(current_external_id.map(|s| s.to_string())) },
    })
    .await;

    match ext_info {
        Ok(info) => {
            let external_id = if loader.type() == "server" { info.external_id.clone().unwrap_or_else(|| info.id.to_string()) } else { None };
            let { raw, buffer } = loader.load_extension(get_extension_load_id(loader, &info));
            let contribution_points = build_contribution_points(&info, raw);
            SingleLoaderInstallResult {
                loader_type: loader.type(),
                success: true,
                info,
                contribution_points,
                external_id,
            }
        },
        Err(error) => SingleLoaderInstallResult {
            loader_type: loader.type(),
            success: false,
            error: Box::new(error),
        },
    }
}
```
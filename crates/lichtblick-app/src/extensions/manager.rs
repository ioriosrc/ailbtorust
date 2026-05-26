// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

//! Extension manager: coordinates installation, loading, activation, and uninstallation.
//! Storage is async (IndexedDB via JsFuture). Uses spawn_local for async operations.

use std::collections::HashMap;

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

use super::loader::{get_extension_source, load_foxe_extension};
use super::storage;
use super::types::{ContributionPoints, ExtensionFormat, ExtensionInfo, RegisteredPanel, StoredExtension};

/// Global extension state, managed with Leptos signals.
#[derive(Clone, Copy)]
pub struct ExtensionState {
    /// List of installed extensions (metadata).
    pub installed: RwSignal<Vec<ExtensionInfo>>,
    /// Panels registered by all active extensions.
    pub panels: RwSignal<HashMap<String, RegisteredPanel>>,
    /// Loading/installing state
    pub is_loading: RwSignal<bool>,
    /// Last error message
    pub last_error: RwSignal<Option<String>>,
    /// Installation status message
    pub status_message: RwSignal<Option<String>>,
}

impl ExtensionState {
    pub fn new() -> Self {
        Self {
            installed: RwSignal::new(Vec::new()),
            panels: RwSignal::new(HashMap::new()),
            is_loading: RwSignal::new(false),
            last_error: RwSignal::new(None),
            status_message: RwSignal::new(None),
        }
    }

    /// Load all installed extensions from IndexedDB on startup.
    pub fn load_from_storage(&self) {
        let state = *self;
        spawn_local(async move {
            let extensions = storage::list_extensions().await;
            let infos: Vec<ExtensionInfo> = extensions.iter().map(|e| e.info.clone()).collect();
            log::info!("Loaded {} extensions from storage", infos.len());
            state.installed.set(infos);

            // Activate all legacy extensions
            for ext in &extensions {
                if ext.info.format == ExtensionFormat::Legacy {
                    activate_legacy_extension(&state, ext);
                }
            }
        });
    }

    /// Install a .foxe extension from raw bytes.
    /// Parsing is synchronous (pure Rust zip), storage is async (IndexedDB).
    pub fn install_foxe(&self, data: Vec<u8>) {
        self.is_loading.set(true);
        self.last_error.set(None);
        self.status_message.set(Some("Installing extension...".to_string()));

        // Parse synchronously (pure Rust zip — fast)
        let stored = match load_foxe_extension(&data) {
            Ok(s) => s,
            Err(e) => {
                log::error!("Failed to parse .foxe: {}", e);
                self.last_error.set(Some(format!("Invalid extension: {}", e)));
                self.status_message.set(None);
                self.is_loading.set(false);
                return;
            }
        };

        let ext_name = stored.info.display_name.clone();
        let ext_id = stored.info.id.clone();
        let version = stored.info.version.clone();
        log::info!("Parsed extension: {} v{} ({})", ext_name, version, ext_id);

        // Store async (IndexedDB)
        let state = *self;
        spawn_local(async move {
            match storage::put_extension(&stored).await {
                Ok(()) => {
                    log::info!("Extension stored: {} ({})", ext_name, ext_id);

                    // Activate
                    activate_legacy_extension(&state, &stored);

                    // Update installed list
                    state.installed.update(|list| {
                        list.retain(|e| e.id != ext_id);
                        list.push(stored.info.clone());
                    });

                    state.status_message.set(Some(format!(
                        "✓ Installed: {} v{}", ext_name, version
                    )));
                }
                Err(e) => {
                    log::error!("Failed to store extension: {}", e);
                    state.last_error.set(Some(format!("Storage error: {}", e)));
                    state.status_message.set(None);
                }
            }
            state.is_loading.set(false);
        });
    }

    /// Uninstall an extension by ID.
    pub fn uninstall(&self, id: String) {
        self.is_loading.set(true);
        self.last_error.set(None);

        let state = *self;
        let id_clone = id.clone();
        spawn_local(async move {
            match storage::delete_extension(&id_clone).await {
                Ok(()) => {
                    log::info!("Extension uninstalled: {}", id_clone);

                    state.installed.update(|list| {
                        list.retain(|e| e.id != id_clone);
                    });

                    state.panels.update(|panels| {
                        panels.retain(|_, panel| panel.extension_id != id_clone);
                    });

                    state.status_message.set(Some("Extension uninstalled".to_string()));
                }
                Err(e) => {
                    log::error!("Failed to uninstall extension: {}", e);
                    state.last_error.set(Some(format!("Uninstall error: {}", e)));
                }
            }
            state.is_loading.set(false);
        });
    }
}

/// Activate a legacy (.foxe) extension by executing its JS code.
fn activate_legacy_extension(state: &ExtensionState, stored: &StoredExtension) {
    let source = match get_extension_source(stored) {
        Ok(s) => s,
        Err(e) => {
            log::error!("Failed to get extension source for {}: {}", stored.info.id, e);
            return;
        }
    };

    let ext_id = stored.info.id.clone();
    let ext_name = stored.info.display_name.clone();

    match execute_extension_js(&source, &ext_id, &ext_name) {
        Ok(contributions) => {
            log::info!(
                "Extension {} activated: {} panels registered",
                ext_name,
                contributions.panels.len()
            );

            state.panels.update(|panels| {
                for panel in contributions.panels {
                    panels.insert(panel.id.clone(), panel);
                }
            });
        }
        Err(e) => {
            log::error!("Failed to activate extension {}: {}", ext_name, e);
        }
    }
}

/// Execute extension JS code in a sandboxed context and collect registrations.
fn execute_extension_js(
    source: &str,
    extension_id: &str,
    extension_name: &str,
) -> Result<ContributionPoints, String> {
    let result = js_execute_extension(source, extension_id, extension_name);

    if result.is_null() || result.is_undefined() {
        return Ok(ContributionPoints::default());
    }

    let mut contributions = ContributionPoints::default();

    if let Ok(array) = js_sys::Array::try_from(result.clone()) {
        for i in 0..array.length() {
            let panel_name = array.get(i);
            if let Some(name) = panel_name.as_string() {
                let panel_id = format!("{}.{}", extension_name, name);
                contributions.panels.push(RegisteredPanel {
                    id: panel_id,
                    name: name.clone(),
                    extension_id: extension_id.to_string(),
                    extension_name: extension_name.to_string(),
                });
            }
        }
    }

    Ok(contributions)
}

#[wasm_bindgen(inline_js = r#"
// ===== Global Extension Converter Registry =====
// Stores message converters registered by extensions.
// Key: fromSchemaName, Value: array of { converter, toSchemaName }
if (!globalThis.__extensionConverters) {
    globalThis.__extensionConverters = {};
}

export function js_execute_extension(source, extensionId, extensionName) {
    try {
        const module = { exports: {} };
        const mockRequire = (name) => {
            if (name === 'react' || name === 'react-dom') {
                return {
                    createElement: () => null,
                    Component: class {},
                    Fragment: 'fragment',
                    useState: () => [null, () => {}],
                    useEffect: () => {},
                    useCallback: (fn) => fn,
                    useMemo: (fn) => fn(),
                    useRef: () => ({ current: null }),
                    render: () => {},
                };
            }
            return {};
        };

        const fn_ = new Function('module', 'require', source);
        fn_(module, mockRequire, {});

        const registeredPanels = [];

        const ctx = {
            mode: 'production',
            registerPanel: (registration) => {
                registeredPanels.push(registration.name);
            },
            registerMessageConverter: (args) => {
                console.log('[Extension] Registered message converter:', args.fromSchemaName, '->', args.toSchemaName);
                if (!globalThis.__extensionConverters[args.fromSchemaName]) {
                    globalThis.__extensionConverters[args.fromSchemaName] = [];
                }
                globalThis.__extensionConverters[args.fromSchemaName].push({
                    converter: args.converter,
                    toSchemaName: args.toSchemaName,
                    extensionId: extensionId,
                });
            },
            registerTopicAliases: (aliasFunction) => {
                console.log('[Extension] Registered topic aliases');
            },
            registerCameraModel: (args) => {
                console.log('[Extension] Registered camera model:', args.name);
            },
        };

        if (module.exports && typeof module.exports.activate === 'function') {
            module.exports.activate(ctx);
        } else if (module.exports && module.exports.default && typeof module.exports.default.activate === 'function') {
            module.exports.default.activate(ctx);
        }

        return registeredPanels;
    } catch (e) {
        console.error('[Extension] Failed to execute:', extensionId, e);
        return null;
    }
}

// Check if any converters are registered for a given schema
export function js_has_converters(fromSchemaName) {
    return !!(globalThis.__extensionConverters[fromSchemaName] &&
              globalThis.__extensionConverters[fromSchemaName].length > 0);
}

// Get list of all registered fromSchemaNames
export function js_get_converter_schemas() {
    return Object.keys(globalThis.__extensionConverters).filter(
        k => globalThis.__extensionConverters[k].length > 0
    );
}

// Run all converters for a schema with a pre-decoded JS message object.
// Returns flat array of frame transform objects, or null if no transforms produced.
export function js_convert_message_with_object(fromSchemaName, messageObj, topicConfigOverride) {
    const converters = globalThis.__extensionConverters[fromSchemaName];
    if (!converters || converters.length === 0) return null;

    const defaultConfig = {
        showAxes: true,
        showPhysicalLanes: true,
        showLogicalLanes: true,
        showReferenceLines: true,
        showBoundingBox: true,
        show3dModels: false,
        caching: true,
        defaultModelPath: '/opt/models/vehicles/',
    };
    const topicConfig = topicConfigOverride || defaultConfig;

    const messageEvent = {
        topic: '',
        schemaName: fromSchemaName,
        receiveTime: { sec: 0, nsec: 0 },
        message: messageObj,
        sizeInBytes: 0,
        topicConfig,
    };

    const frames = [];
    for (const entry of converters) {
        try {
            const converted = entry.converter(messageObj, messageEvent, topicConfig, { emitAlert: () => {} });
            if (converted == null) continue;

            if (entry.toSchemaName === 'foxglove.FrameTransforms') {
                const transforms = converted.transforms || [];
                for (const tf of transforms) {
                    frames.push({
                        parent_frame_id: tf.parent_frame_id || '',
                        child_frame_id: tf.child_frame_id || '',
                        tx: tf.translation ? tf.translation.x || 0 : 0,
                        ty: tf.translation ? tf.translation.y || 0 : 0,
                        tz: tf.translation ? tf.translation.z || 0 : 0,
                        rx: tf.rotation ? tf.rotation.x || 0 : 0,
                        ry: tf.rotation ? tf.rotation.y || 0 : 0,
                        rz: tf.rotation ? tf.rotation.z || 0 : 0,
                        rw: tf.rotation ? tf.rotation.w || 1 : 1,
                        timestamp_sec: tf.timestamp ? tf.timestamp.sec || 0 : 0,
                        timestamp_nsec: tf.timestamp ? tf.timestamp.nsec || 0 : 0,
                    });
                }
            } else if (entry.toSchemaName === 'foxglove.FrameTransform') {
                const tf = converted;
                frames.push({
                    parent_frame_id: tf.parent_frame_id || '',
                    child_frame_id: tf.child_frame_id || '',
                    tx: tf.translation ? tf.translation.x || 0 : 0,
                    ty: tf.translation ? tf.translation.y || 0 : 0,
                    tz: tf.translation ? tf.translation.z || 0 : 0,
                    rx: tf.rotation ? tf.rotation.x || 0 : 0,
                    ry: tf.rotation ? tf.rotation.y || 0 : 0,
                    rz: tf.rotation ? tf.rotation.z || 0 : 0,
                    rw: tf.rotation ? tf.rotation.w || 1 : 1,
                    timestamp_sec: tf.timestamp ? tf.timestamp.sec || 0 : 0,
                    timestamp_nsec: tf.timestamp ? tf.timestamp.nsec || 0 : 0,
                });
            }
        } catch (e) {
            console.error('[Extension] Converter error:', entry.extensionId, fromSchemaName, '->', entry.toSchemaName, e);
        }
    }
    return frames.length > 0 ? frames : null;
}

// Returns { cubes: [...], lines: [...] } from SceneUpdate converters, or null.
export function js_convert_message_to_scene(fromSchemaName, messageObj, topicConfigOverride) {
    const converters = globalThis.__extensionConverters[fromSchemaName];
    if (!converters || converters.length === 0) return null;

    const defaultConfig = {
        showAxes: true,
        showPhysicalLanes: true,
        showLogicalLanes: true,
        showReferenceLines: true,
        showBoundingBox: true,
        show3dModels: false,
        caching: true,
        defaultModelPath: '/opt/models/vehicles/',
    };
    const topicConfig = topicConfigOverride || defaultConfig;

    const messageEvent = {
        topic: '',
        schemaName: fromSchemaName,
        receiveTime: { sec: 0, nsec: 0 },
        message: messageObj,
        sizeInBytes: 0,
        topicConfig,
    };

    const cubes = [];
    const lines = [];
    const triangles = [];
    for (const entry of converters) {
        if (entry.toSchemaName !== 'foxglove.SceneUpdate') continue;
        try {
            const converted = entry.converter(messageObj, messageEvent, topicConfig, { emitAlert: () => {} });
            if (converted == null) continue;

            const entities = converted.entities || [];
            for (const entity of entities) {
                const frame_id = entity.frame_id || '';

                // Extract cubes
                if (entity.cubes) {
                    for (const c of entity.cubes) {
                        const pos = c.pose?.position || {};
                        const ori = c.pose?.orientation || {};
                        const sz = c.size || {};
                        const col = c.color || {};
                        cubes.push({
                            frame_id,
                            px: pos.x || 0, py: pos.y || 0, pz: pos.z || 0,
                            ox: ori.x || 0, oy: ori.y || 0, oz: ori.z || 0, ow: ori.w != null ? ori.w : 1,
                            sx: sz.x || 1, sy: sz.y || 1, sz: sz.z || 1,
                            r: col.r != null ? col.r : 1,
                            g: col.g != null ? col.g : 1,
                            b: col.b != null ? col.b : 1,
                            a: col.a != null ? col.a : 1,
                        });
                    }
                }

                // Extract lines
                if (entity.lines) {
                    for (const l of entity.lines) {
                        const pos = l.pose?.position || {};
                        const ori = l.pose?.orientation || {};
                        const col = l.color || {};
                        const pts = [];
                        if (l.points) {
                            for (const p of l.points) {
                                pts.push(p.x || 0, p.y || 0, p.z || 0);
                            }
                        }
                        if (pts.length > 0) {
                            lines.push({
                                frame_id,
                                line_type: l.type || 0,
                                px: pos.x || 0, py: pos.y || 0, pz: pos.z || 0,
                                ox: ori.x || 0, oy: ori.y || 0, oz: ori.z || 0, ow: ori.w != null ? ori.w : 1,
                                r: col.r != null ? col.r : 1,
                                g: col.g != null ? col.g : 1,
                                b: col.b != null ? col.b : 1,
                                a: col.a != null ? col.a : 1,
                                points: pts,
                            });
                        }
                    }
                }

                // Extract triangles
                if (entity.triangles) {
                    for (const t of entity.triangles) {
                        const pos = t.pose?.position || {};
                        const ori = t.pose?.orientation || {};
                        const baseCol = t.color || {};
                        const pts = [];
                        const colors = [];
                        if (t.points) {
                            for (let i = 0; i < t.points.length; i++) {
                                const p = t.points[i];
                                pts.push(p.x || 0, p.y || 0, p.z || 0);
                                // Per-vertex color or fall back to base color
                                const vc = (t.colors && t.colors[i]) || baseCol;
                                colors.push(
                                    vc.r != null ? vc.r : 1,
                                    vc.g != null ? vc.g : 1,
                                    vc.b != null ? vc.b : 1,
                                    vc.a != null ? vc.a : 1
                                );
                            }
                        }
                        if (pts.length > 0) {
                            triangles.push({
                                frame_id,
                                px: pos.x || 0, py: pos.y || 0, pz: pos.z || 0,
                                ox: ori.x || 0, oy: ori.y || 0, oz: ori.z || 0, ow: ori.w != null ? ori.w : 1,
                                points: pts,
                                colors: colors,
                                indices: t.indices && t.indices.length > 0 ? Array.from(t.indices) : null,
                            });
                        }
                    }
                }
            }
        } catch (e) {
            console.error('[Extension] SceneUpdate converter error:', entry.extensionId, e);
        }
    }
    if (cubes.length === 0 && lines.length === 0 && triangles.length === 0) return null;
    return { cubes, lines, triangles };
}
"#)]
extern "C" {
    fn js_execute_extension(source: &str, extension_id: &str, extension_name: &str) -> JsValue;

    pub fn js_has_converters(from_schema_name: &str) -> bool;

    pub fn js_get_converter_schemas() -> JsValue;

    pub fn js_convert_message_with_object(from_schema_name: &str, message_obj: JsValue, topic_config: JsValue) -> JsValue;

    pub fn js_convert_message_to_scene(from_schema_name: &str, message_obj: JsValue, topic_config: JsValue) -> JsValue;
}

/// Provide the extension state via Leptos context.
pub fn provide_extension_state() -> ExtensionState {
    let state = ExtensionState::new();
    provide_context(state);
    state
}

/// Use the extension state from Leptos context.
pub fn use_extension_state() -> ExtensionState {
    expect_context::<ExtensionState>()
}

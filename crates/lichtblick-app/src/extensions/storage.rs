// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

//! Extension storage using IndexedDB (via JS helpers).
//! Uses JsFuture for safe async — no Closure::once, no .forget().
//! IndexedDB supports hundreds of MB, unlike localStorage (5MB limit).

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use super::types::StoredExtension;

#[wasm_bindgen(inline_js = r#"
const DB_NAME = 'lichtblick-extensions';
const STORE_NAME = 'extensions';
const DB_VERSION = 1;

function openDB() {
    return new Promise((resolve, reject) => {
        const req = indexedDB.open(DB_NAME, DB_VERSION);
        req.onupgradeneeded = (e) => {
            const db = e.target.result;
            if (!db.objectStoreNames.contains(STORE_NAME)) {
                db.createObjectStore(STORE_NAME);
            }
        };
        req.onsuccess = (e) => resolve(e.target.result);
        req.onerror = (e) => reject(e.target.error);
    });
}

export async function idb_put(key, value) {
    const db = await openDB();
    return new Promise((resolve, reject) => {
        const tx = db.transaction(STORE_NAME, 'readwrite');
        const store = tx.objectStore(STORE_NAME);
        const req = store.put(value, key);
        req.onsuccess = () => resolve();
        req.onerror = (e) => reject(e.target.error);
        tx.oncomplete = () => db.close();
    });
}

export async function idb_get(key) {
    const db = await openDB();
    return new Promise((resolve, reject) => {
        const tx = db.transaction(STORE_NAME, 'readonly');
        const store = tx.objectStore(STORE_NAME);
        const req = store.get(key);
        req.onsuccess = () => resolve(req.result !== undefined ? req.result : null);
        req.onerror = (e) => reject(e.target.error);
        tx.oncomplete = () => db.close();
    });
}

export async function idb_delete(key) {
    const db = await openDB();
    return new Promise((resolve, reject) => {
        const tx = db.transaction(STORE_NAME, 'readwrite');
        const store = tx.objectStore(STORE_NAME);
        const req = store.delete(key);
        req.onsuccess = () => resolve();
        req.onerror = (e) => reject(e.target.error);
        tx.oncomplete = () => db.close();
    });
}

export async function idb_get_all_keys() {
    const db = await openDB();
    return new Promise((resolve, reject) => {
        const tx = db.transaction(STORE_NAME, 'readonly');
        const store = tx.objectStore(STORE_NAME);
        const req = store.getAllKeys();
        req.onsuccess = () => resolve(JSON.stringify(req.result || []));
        req.onerror = (e) => reject(e.target.error);
        tx.oncomplete = () => db.close();
    });
}
"#)]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn idb_put(key: &str, value: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn idb_get(key: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn idb_delete(key: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn idb_get_all_keys() -> Result<JsValue, JsValue>;
}

/// Get all installed extensions from IndexedDB.
pub async fn list_extensions() -> Vec<StoredExtension> {
    let keys_result = match idb_get_all_keys().await {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };

    let keys_json = match keys_result.as_string() {
        Some(s) => s,
        None => return Vec::new(),
    };

    let keys: Vec<String> = match serde_json::from_str(&keys_json) {
        Ok(k) => k,
        Err(_) => return Vec::new(),
    };

    let mut extensions = Vec::new();
    for key in keys {
        if let Ok(val) = idb_get(&key).await {
            if let Some(json) = val.as_string() {
                if let Ok(ext) = serde_json::from_str::<StoredExtension>(&json) {
                    extensions.push(ext);
                }
            }
        }
    }
    extensions
}

/// Store an extension in IndexedDB.
pub async fn put_extension(ext: &StoredExtension) -> Result<(), String> {
    let json = serde_json::to_string(ext)
        .map_err(|e| format!("Serialize error: {}", e))?;

    idb_put(&ext.info.id, &json)
        .await
        .map_err(|e| format!("IndexedDB write error: {:?}", e))?;

    Ok(())
}

/// Delete an extension from IndexedDB by ID.
pub async fn delete_extension(id: &str) -> Result<(), String> {
    idb_delete(id)
        .await
        .map_err(|e| format!("IndexedDB delete error: {:?}", e))?;
    Ok(())
}

/// Get a single extension by ID.
pub async fn get_extension(id: &str) -> Option<StoredExtension> {
    let val = idb_get(id).await.ok()?;
    let json = val.as_string()?;
    serde_json::from_str(&json).ok()
}
